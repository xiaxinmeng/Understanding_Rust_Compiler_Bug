rust
fn main() {
    for triple in rustc_target::spec::get_targets() {
        let target_triple = rustc_target::spec::TargetTriple::from_triple(&triple);
        let target = rustc_target::spec::Target::search(&target_triple).unwrap();
        let layout = rustc_target::abi::TargetDataLayout::parse(&target).unwrap();
        if
            layout.i8_align.abi.bits() != 8 ||
            layout.i16_align.abi.bits() != 16 ||
            layout.i32_align.abi.bits() != 32 ||
            layout.i64_align.abi.bits() != 64
        {
            print!("{:<30}", triple);
            print!("{:>4}", layout.i8_align.abi.bits());
            print!("{:>4}", layout.i16_align.abi.bits());
            print!("{:>4}", layout.i32_align.abi.bits());
            print!("{:>4}", layout.i64_align.abi.bits());
            println!("  ({})", target.max_atomic_width());
        }
    }
}
