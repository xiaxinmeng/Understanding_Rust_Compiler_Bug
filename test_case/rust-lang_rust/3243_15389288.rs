 rust
const some_array : &'static [int] = &'static [1,2,3];

fn main() {
    io::println("Direct");
    for [1, 2, 3].each |i| {
        io::println(fmt!("%d", *i));
    }

    io::println("Indirect global");
    for (|| &some_array)().each |i| {
        io::println(fmt!("%d", *i));
    }

    io::println("Indirect local");
    for (|| &[1, 2, 3])().each |i| {
        io::println(fmt!("%d", *i));
    }
}
