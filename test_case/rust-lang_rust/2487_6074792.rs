
class socket {
    let sock: int;

    new() { self.sock = 1; }

    drop { }

    fn set_identity()  {
        closure { ||
            setsockopt_bytes(self.sock)
        }
    }
}

fn closure(f: fn@()) { f() }

fn setsockopt_bytes(_sock: int) { }
