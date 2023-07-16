
let found = for list.each |i| {
  if i == 2 { break }
} then { false } else { true }
if found { ... }
