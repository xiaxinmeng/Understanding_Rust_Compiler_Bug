
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `[^1; _]: std::cmp::PartialEq<&&[^4]>`
#1 [typeck] type-checking `<C<B, N> as std::cmp::PartialEq<&'a [A]>>::eq`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
