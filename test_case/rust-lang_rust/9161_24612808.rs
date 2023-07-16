
// first pair is the escape sequence
format!("{0, select, other{test}} }}", "a") // => ~"test} "

// second pair is the escape sequence
format!("{0, select, other{test}} }}", "a") // => ~"test }"
