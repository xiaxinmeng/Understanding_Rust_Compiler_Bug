
<anon>:1:30: 1:34 error: missing lifetime specifier [E0106]
<anon>:1 fn test(s: &str, t: &str) -> &str {
                                      ^~~~
<anon>:1:34: 1:34 help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s` or `t`
