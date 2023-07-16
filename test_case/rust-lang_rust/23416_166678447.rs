
let v = v.iter().collect::<Vec<_>>().sort(); // Parens are not required
let v = (v.iter().collect(): Vec<_>).sort(); // Parens are required
// Formatted on several lines
let v = (v.iter().
           collect(): Vec<_>). // Looks bad, man
           sort();
