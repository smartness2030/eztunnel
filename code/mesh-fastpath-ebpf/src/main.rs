#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::{
        BPF_ANY,
        BPF_F_INGRESS,
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB,
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB,
        sk_action,
    },
    macros::{
        map,
        sock_ops,
        sk_msg,
    },
    programs::{
        SockOpsContext,
        SkMsgContext,
    },
};
use aya_log_ebpf::{info, warn, error};

use mesh_fastpath_common::{SockPairTuple};
mod sock_hash;

const AF_INET: u8 = 2;

// select which port the program should handle
// 0    - all
// else - only this specific port
const PROXY_PORT: u16 = 0; //6379, 45678;

#[map]
static SOCKETS: sock_hash::SockHash<SockPairTuple> = sock_hash::SockHash::with_max_entries(65_536, 0);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

/// `intercept_active_sockets` (`sock_ops` hook)
///
/// adds the socket to `SockHash` if it's related to `PROXY_PORT`
/// .
#[sock_ops]
fn intercept_active_sockets(ctx: SockOpsContext) -> u32 {
    match try_intercept_active_sockets(&ctx) {
        Ok(ret) => ret,
        Err(ret) => {
            error!(&ctx, "`intercept_active_sockets` errored: {}", ret);
            return 0;
        },
    }
}

fn try_intercept_active_sockets<'a>(ctx: &SockOpsContext) -> Result<u32, &'a str> {
    let ops = unsafe { *ctx.ops };

    let family = ops.family as u8;

    if family != AF_INET {
        return Ok(0);
    }

    let local_ip = ops.local_ip4.swap_bytes();
    let local_port = ops.local_port as u16;

    let remote_ip = ops.remote_ip4.swap_bytes();
    let remote_port = ops.remote_port.swap_bytes() as u16;

    if
        !(PROXY_PORT == 0 || local_port == PROXY_PORT || remote_port == PROXY_PORT) ||
        !(ops.op == BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB || ops.op == BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB)
    {
        return Ok(0);
    }

    let sock_pair_tuple = SockPairTuple {
        local_ip,
        local_port,
        remote_ip,
        remote_port,
    };

    let result = SOCKETS.update(sock_pair_tuple, ctx.ops, BPF_ANY as u64);

    let op_str = match ops.op {
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB =>  " active",
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => "passive",
        _ => "",
    };

    if let Err(e) = result {
        info!(ctx, "[intercept_active_sockets] op {}; failed {}; {:i}:{} -> {:i}:{}",
            op_str,
            e,
            local_ip,
            local_port,
            remote_ip,
            remote_port,
        );
    } else {
        info!(ctx, "[intercept_active_sockets] op {};  saved; {:i}:{} -> {:i}:{}",
            op_str,
            local_ip,
            local_port,
            remote_ip,
            remote_port,
        );
    };

    return Ok(0);
}

/// `redirect_between_sockets` (`sk_msg` hook)
///
/// intercepts packet to bypass the network stack by redirecting it from a socket to another socket
/// .
#[sk_msg]
fn redirect_between_sockets(ctx: SkMsgContext) -> u32 {
    match try_redirect_between_sockets(&ctx) {
        Ok(ret) => ret,
        Err(ret) => {
            error!(&ctx, "`redirect_between_sockets` errored: {}", ret);
            return sk_action::SK_PASS;
        },
    }
}

fn try_redirect_between_sockets<'a>(ctx: &SkMsgContext) -> Result<u32, &'a str> {
    let msg = unsafe { *ctx.msg };

    let family = msg.family as u8;

    if family != AF_INET {
        return Ok(sk_action::SK_PASS);
    }

    let local_ip = msg.local_ip4.swap_bytes();
    let local_port = msg.local_port as u16;

    let remote_ip = msg.remote_ip4.swap_bytes();
    let remote_port = msg.remote_port.swap_bytes() as u16;

    let sock_pair_tuple = SockPairTuple {
        local_ip: remote_ip,
        local_port: remote_port,

        remote_ip: local_ip,
        remote_port: local_port,
    };

    let result = SOCKETS.redirect_msg(&ctx, sock_pair_tuple, BPF_F_INGRESS as u64) as u32;

    if result == sk_action::SK_PASS {
        info!(ctx, "[redirect_between_sockets] redirected {:i}:{} -> {:i}:{}",
            local_ip,
            local_port,
            remote_ip,
            remote_port,
        );
    } else {
        warn!(ctx, "[redirect_between_sockets]   fallback {:i}:{} -> {:i}:{}",
            local_ip,
            local_port,
            remote_ip,
            remote_port,
        );
    };

    return Ok(sk_action::SK_PASS);
}
