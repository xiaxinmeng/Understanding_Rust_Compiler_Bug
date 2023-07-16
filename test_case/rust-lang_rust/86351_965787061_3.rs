
query stack during panic:
#0 [resolve_instance] resolving instance `<dyn TestTrait<MyType = T> as TestTrait>::func`
#1 [mir_built] building MIR for `<impl at src/lib.rs:6:1: 13:2>::other_func`
#2 [unsafety_check_result] unsafety-checking `<impl at src/lib.rs:6:1: 13:2>::other_func`
#3 [mir_const] processing MIR for `<impl at src/lib.rs:6:1: 13:2>::other_func`
#4 [mir_promoted] processing `<impl at src/lib.rs:6:1: 13:2>::other_func`
#5 [mir_borrowck] borrow-checking `<impl at src/lib.rs:6:1: 13:2>::other_func`
#6 [analysis] running analysis passes on this crate
end of query stack
