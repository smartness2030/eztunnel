#!/usr/bin/env python3

import sys
import socket
import signal
import asyncio

assert len(sys.argv) == 2, 'wrong argument count. expected `python3 sock_server.py 127.0.0.1:8080`'

#[HOST, PORT] = '192.168.0.112:6300'.split(':')
[HOST, PORT] = sys.argv[1].split(':')
PORT = int(PORT)

connections = set()

async def handle_client(conn, addr):
  print(f'connection accepted from {addr}')
  connections.add(conn)

  loop = asyncio.get_event_loop()

  while True:
    data_recv = await loop.sock_recv(conn, 1024)
    if not data_recv: break
    print(f'got {data_recv} from {addr}')
    await loop.sock_sendall(conn, data_recv + b'[reply;python]')

  conn.close()
  connections.remove(conn)
  print(f'connection with {addr} closed')

async def server_loop():
  # socket.SOCK_DGRAM
  sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM | socket.SOCK_NONBLOCK)
  sock.bind((HOST, PORT))
  sock.listen(1024)

  loop = asyncio.get_event_loop()

  def close_handler(signum, frame):
    for conn in connections:
      conn.close()
    sys.exit(0)

  signal.signal(signal.SIGINT, close_handler)

  print(f'listening on {(HOST, PORT)}')
  while True:
    conn, addr = await loop.sock_accept(sock)
    loop.create_task(handle_client(conn, addr))

asyncio.run(server_loop())
