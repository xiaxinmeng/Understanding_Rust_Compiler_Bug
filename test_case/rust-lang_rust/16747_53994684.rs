
<anon>:5:1: 7:2 error: the parameter type `T` may not live long enough; consider adding an explicit lifetime bound `T:'a`...
<anon>:5 struct List<'a, T: ListItem<'a>> {
<anon>:6     slice: &'a [T],
<anon>:7 }
<anon>:5:1: 7:2 note: ...so that the reference type `&'a [T]` does not outlive the data it points at
<anon>:5 struct List<'a, T: ListItem<'a>> {
<anon>:6     slice: &'a [T],
<anon>:7 }
error: aborting due to previous error
