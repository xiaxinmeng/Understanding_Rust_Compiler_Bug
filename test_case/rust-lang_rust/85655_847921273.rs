rust
assert!(src.is_loopback() && dst.is_loopback());
socket.bind(src);
socket.connect(dst);
