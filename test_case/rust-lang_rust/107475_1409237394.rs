
help: consider introducing a named lifetime parameter
1 | fn parse_type<'a>(iter: Box<dyn Iterator<Item=&'a str>+'static>) -> &'a str { iter.next() }
