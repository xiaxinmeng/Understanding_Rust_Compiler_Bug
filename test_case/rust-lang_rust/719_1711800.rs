
native "rust" mod rustrt {
    type rust_port;
}

native "rust-intrinsic" mod rusti {
    fn recv[T](port : *rustrt::rust_port) -> T;
}

obj _port[T](raw_port : @port_ptr) {
    fn recv() -> T {
        rusti::recv(**raw_port)
    }
}
