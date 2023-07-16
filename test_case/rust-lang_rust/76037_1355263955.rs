
CARGO_INCREMENTAL=1 cargo clean && cargo build && touch src/main.rs && time cargo build
<snip; compiling succeeds the first time, then yields the following the second time>
thread '<unnamed>' panicked at 'invalid enum variant tag while decoding `Mutability`, expected 0..2', /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/compiler/rustc_ast/src/ast.rs:778:41
