set -e
cd server
rustc server.rs
echo connect with: nc -U /home/giles/nuance.sock
./server
