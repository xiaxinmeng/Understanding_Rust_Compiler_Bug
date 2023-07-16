plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck  driver.rs -o ""/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck"/driver"
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

error: incompatible lifetime on type
error: incompatible lifetime on type
    --> driver.rs:159:14
     |
159  |             .collect()
     |              ^^^^^^^
     |
note: because this has an unmet lifetime requirement
     |
     |
1784 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ introduces a `'static` lifetime requirement
note: the lifetime `'tcx` as defined here...
    --> driver.rs:148:15
     |
148  | fn get_bodies<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<(String, BodyWithBorrowckFacts<'tcx>)> {
     |               ^^^^
note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
     |
     |
2609 | / impl<T> FromIterator<T> for Vec<T> {
2610 | |     #[inline]
2611 | |     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
2612 | |         <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
2614 | | }
     | |_^

error: aborting due to previous error; 1 warning emitted
error: aborting due to previous error; 1 warning emitted

make: *** [Makefile:19: all] Error 1



failures:
