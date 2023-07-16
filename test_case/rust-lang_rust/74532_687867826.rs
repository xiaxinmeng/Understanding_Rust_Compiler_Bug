rust
    for triple in rustc_target::spec::get_targets() {
        let target_triple = rustc_target::spec::TargetTriple::from_triple(&triple);
        let target = rustc_target::spec::Target::search(&target_triple).unwrap();
        let layout = rustc_target::abi::TargetDataLayout::parse(&target).unwrap();
        if target.max_atomic_width() > 64 {
            println!(
                "{:<30} {:>4} {}",
                triple,
                layout.i128_align.abi.bits(),
                target.max_atomic_width()
            );
        }
    }
