
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `&mut T: std::marker::Unsize`
#1 [coerce_unsized_info] processing `<Redirectable<'_, T> as std::ops::CoerceUnsized<Redirectable<'_, U>>>`
#2 [coherent_trait] coherence checking all impls of trait `std::ops::CoerceUnsized`
#3 [analysis] running analysis passes on this crate
end of query stack
