
/home/nmatsakis/tmp/issue-28853.rs:20:9: 20:23 error: cannot infer an appropriate lifetime due to conflicting requirements [E0495]
/home/nmatsakis/tmp/issue-28853.rs:20     map.get(&names[0])
                                              ^~~~~~~~~~~~~~
/home/nmatsakis/tmp/issue-28853.rs:16:1: 21:2 help: consider using an explicit lifetime parameter as shown: fn foo<'a, 'b>(names: &[&'a str], map: &'a Map<&'static str>)
 -> Option<&'a u32>
/home/nmatsakis/tmp/issue-28853.rs:16 fn foo<'a,'b>(names: &[&str],
/home/nmatsakis/tmp/issue-28853.rs:17               map: &'a Map<&'static str>)
/home/nmatsakis/tmp/issue-28853.rs:18               -> Option<&'a u32>
/home/nmatsakis/tmp/issue-28853.rs:19 {
/home/nmatsakis/tmp/issue-28853.rs:20     map.get(&names[0])
/home/nmatsakis/tmp/issue-28853.rs:21 }
error: aborting due to previous error
