#![no_std]

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SockPairTuple {
    pub local_ip: u32,
    pub local_port: u16,

    pub remote_ip: u32,
    pub remote_port: u16,
}
