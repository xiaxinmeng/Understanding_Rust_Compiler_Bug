
pub trait T {}

pub fn foo(a: impl T, b: impl T, times: usize) -> impl T {
    if times == 1 {
        return a;
    }
    foo(b, a, times - 1)
}
