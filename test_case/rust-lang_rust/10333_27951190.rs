
"foo".as_ascii().to_str() == "[f, o, o]" // because of generic impl of ToStr for &[T] with T: ToStr
"foo".as_ascii().into_str() == "foo" // Because into_str is implemented for &[Ascii]
