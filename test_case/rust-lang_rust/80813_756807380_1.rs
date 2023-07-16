diff
- fn foo<'a>(v: &'a mut Vec<&'a str>, s: &'a str) {
+ fn foo<'a>(v: &mut Vec<&'a str>, s: &'a str) {
      v.push(s);
  }
