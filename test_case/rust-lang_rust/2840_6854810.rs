
class apple {
    let one_field_is_required: int;

    new(n:int) {
        io::println(#fmt("apple(%d)", n));
        self.one_field_is_required = 0;
        if n != 0 {
            ret apple(n-1);
        };
    }
}

fn main() {
    apple(5);
}
