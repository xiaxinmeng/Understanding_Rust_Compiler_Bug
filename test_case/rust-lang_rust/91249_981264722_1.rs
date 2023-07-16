rust
mod m {
    pub const x: i32 = 0;
}

macro mac_a($mac_b: ident) {
    mod m {
        pub const x: i32 = 1;
    }

    macro $mac_b() {
        crate::m::x + m::x
    }
}

mac_a!(mac_b);

fn main() {
    println!("{}", mac_b!());
}
