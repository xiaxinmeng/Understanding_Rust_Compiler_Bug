
git clone https://github.com/vakaras/rust.git -b issue-50716 issue-50716
cd issue-50716/
./x.py build
./x.py test src/tools/cargotest src/tools/cargo
rustup toolchain link issue-50716 $PWD/build/x86_64-unknown-linux-gnu/stage2/
echo 'struct Qey<Q: ?Sized>(Q);' > code.rs
rustc +issue-50716 --crate-type lib code.rs
warning: struct is never used: `Qey`
 --> code.rs:1:1
  |
1 | struct Qey<Q: ?Sized>(Q);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

error: internal compiler error: broken MIR in DefId(0/1:9 ~ code[8787]::Qey[0]::{{constructor}}[0]) (""): errors selecting obligation: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<Q as std::marker::Sized>)),depth=1),Unimplemented)]
 --> code.rs:1:1
  |
1 | struct Qey<Q: ?Sized>(Q);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
