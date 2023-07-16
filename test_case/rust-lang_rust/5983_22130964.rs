 rust
fn memFib(n: int) -> @fn(int) -> int {
    let mut memo = @~[0, 1];
    let fib = |n: int| -> int {
        let mut result: int = - 1;
        if n < memo.len() as int {
            result = memo[n];
        } else {
//            result = fib(n - 1) + fib(n - 2);
            memo.push(result);
        }
        return result;
    };
    return fib;
}


fn main() {}
