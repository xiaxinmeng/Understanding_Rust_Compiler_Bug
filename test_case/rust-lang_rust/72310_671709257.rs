
let mut iter = (1..20).peekable();
while iter.next_if(|&x| x < 10).is_some() {}
