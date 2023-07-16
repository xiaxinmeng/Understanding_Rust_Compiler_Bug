rust
fn foo(){}

#[test_case]
const foo_gensym: TestDescAndFn = TestDescAndFn {
    desc: TestDesc {
        name: TestName::Static("foo"),
        ignore: false,
        should_panic: ShouldPanic::No
    },
    testfn: TestFn::StaticTestFn(|| {
         assert_test_result(foo())
    }),
}
