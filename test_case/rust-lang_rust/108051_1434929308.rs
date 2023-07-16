
➜ ./x.py build --target x86_64-apple-darwin --stage 2 compiler/rustc --dry-run
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
Building stage0 library artifacts (x86_64-unknown-linux-gnu)
Building compiler artifacts (stage0 -> stage1)
Assembling stage1 compiler
Building stage1 library artifacts (x86_64-unknown-linux-gnu)
Building compiler artifacts (stage1 -> stage2)
Assembling stage2 compiler
Uplifting library (stage1 -> stage2)
Uplifting rustc (stage1 -> stage3)
