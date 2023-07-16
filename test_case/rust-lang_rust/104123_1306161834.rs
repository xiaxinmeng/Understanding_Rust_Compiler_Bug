plain

8 note: required by a bound in `is_maybe_transmutable`
9   --> $DIR/transmute-padding-ice.rs:11:14
10    |
- LL |       pub fn is_maybe_transmutable<Src, Dst>()
- LL |       where
- LL |       where
14 LL |           Dst: BikeshedIntrinsicFrom<
16 LL | |             Src,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice/transmute-padding-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmute/transmute-padding-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/transmute/transmute-padding-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
   |
   |
LL |     assert::is_maybe_transmutable::<B, A>();
   |                                        ^ `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
   |
   = help: the trait `BikeshedIntrinsicFrom<B, assert::Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>` is not implemented for `A`
note: required by a bound in `is_maybe_transmutable`
   |
   |
LL |           Dst: BikeshedIntrinsicFrom<
LL | |             Src,
LL | |             Context,
LL | |             Context,
LL | |             { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
LL | |         >,
   | |_________^ required by this bound in `is_maybe_transmutable`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
