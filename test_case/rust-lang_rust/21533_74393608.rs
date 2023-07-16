 rust
struct Outer(*mut ProblemType);
struct ProblemType ([Outer; 8]);

fn breaks_compiler(_: Outer) {}
