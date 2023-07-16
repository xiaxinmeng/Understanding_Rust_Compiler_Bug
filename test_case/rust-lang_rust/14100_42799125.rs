 rust
enum TestResult { Pass(StrBuf), Fail(StrBuf) }

#[test] fn normal() { ... }
#[test] fn advanced() -> TestResult { ... }
