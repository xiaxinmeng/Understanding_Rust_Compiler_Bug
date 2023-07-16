
resource r(i: @mutable int) {
    *i = 100;
}

fn main() {
    let i = @mutable 200;
    alt (r(i)) {
      _ { }
    }
    assert i == 100;
}
