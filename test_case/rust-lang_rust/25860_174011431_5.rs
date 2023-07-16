
for<'a,'b> where<'b:'a, T:'b> fn(&'a &'b T, &'b T) -> &'a T 
<:
for<'a,'b> fn(&'static &'static T, &'b T) -> &'a T
? (today yes, under this RFC no)
