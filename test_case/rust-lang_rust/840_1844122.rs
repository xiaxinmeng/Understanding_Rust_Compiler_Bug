
fn main() {
    let regs = @{mutable eax: 0};
    alt true {
      true { }
      false { }
    };
    (*regs).eax = 1;
}
