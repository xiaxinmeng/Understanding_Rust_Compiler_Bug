
[src/librustdoc/clean/mod.rs:1101] tcx.explicit_item_bounds(self.def_id) = [
    (
        Binder(TraitPredicate(<<Self as b::IntoIterator>::IntoIter as std::marker::Sized>), []),
        /home/cynecx/dev/test-rustdoc/b/src/lib.rs:3:5: 3:48 (#0),
    ),
    (
        Binder(TraitPredicate(<<Self as b::IntoIterator>::IntoIter as std::iter::Iterator>), []),
        /home/cynecx/dev/test-rustdoc/b/src/lib.rs:3:20: 3:47 (#0),
    ),
    (
        Binder(ProjectionPredicate(ProjectionTy { substs: [<Self as b::IntoIterator>::IntoIter], item_def_id: DefId(2:7247 ~ core[51e2]::iter::traits::iterator::Iterator::Item) }, <Self as b::IntoIterator>::Item), []),
        /home/cynecx/dev/test-rustdoc/b/src/lib.rs:3:29: 3:46 (#0),
    ),
]
