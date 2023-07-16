
query stack during panic:
#0 [is_sized_raw] computing whether `[lignin::Node<'_, lignin::ThreadSafe>]` is `Sized`
#1 [layout_of] computing layout of `&[lignin::Node<'_, lignin::ThreadSafe>]`
#2 [layout_of] computing layout of `lignin::Node<'_, lignin::ThreadSafe>`
end of query stack
error: could not compile `schichler-dev`
