#!/usr/bin/env python3

import sys
import socket
import time
import signal

assert len(sys.argv) == 2, 'wrong argument count. expected `python3 sock_client.py 127.0.0.1:8080`'

#[HOST, PORT] = '192.168.0.112:6300'.split(':')
[HOST, PORT] = sys.argv[1].split(':')
PORT = int(PORT)

times = []

should_exit = 0
def signal_handler(signum, frame):
  global should_exit
  global sock
  should_exit += 1
  signame = signal.Signals(signum).name
  if should_exit >= 2:
    sock.close()
    sys.exit()

signal.signal(signal.SIGINT, signal_handler)

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((HOST, PORT))

while should_exit == 0:
  start = time.time_ns()
  try:
    data = f'ping-{start}'.encode()
    sock.sendall(data)
    data_recv = sock.recv(1024)
    if not data_recv: break
  except Exception as e:
    print(e)
    break
  end = time.time_ns()
  diff = end - start
  print(f'{len(times)},{diff}')
  times.append(diff)
  time.sleep(.1)

sock.close()
