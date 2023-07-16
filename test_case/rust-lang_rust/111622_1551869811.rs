plain
failures:

---- [ui] tests/ui/consts/unnormalized-param-env.rs stdout ----
normalized stderr:
error[E0277]: the trait bound `CS: CSpace<4>` is not satisfied
   |
   |
LL | /     fn trajectory_free<TF, S1>(&self, t: &TF)
LL | |     where
LL | |         CS::Traj: Sized,
LL | |         CS: CSpace<N>,
   | |______________________^ the trait `CSpace<4>` is not implemented for `CS`
help: consider restricting type parameter `CS`
   |
   |
LL | impl<CS: CSpace<4>> Obstacle<CS, N> for ObstacleSpace2df32 {


error[E0277]: the trait bound `CS: CSpace<4>` is not satisfied
   |
   |
LL |     fn trajectory_free<TF, S1>(&self, t: &TF)
   |        ^^^^^^^^^^^^^^^ the trait `CSpace<4>` is not implemented for `CS`
help: consider restricting type parameter `CS`
   |
   |
LL | impl<CS: CSpace<4>> Obstacle<CS, N> for ObstacleSpace2df32 {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args consts/unnormalized-param-env.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/unnormalized-param-env.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/unnormalized-param-env" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/unnormalized-param-env/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `CS: CSpace<4>` is not satisfied
  --> fake-test-src-base/consts/unnormalized-param-env.rs:23:5
   |
LL | /     fn trajectory_free<TF, S1>(&self, t: &TF)
LL | |     where
LL | |         CS::Traj: Sized,
LL | |         CS: CSpace<N>,
   | |______________________^ the trait `CSpace<4>` is not implemented for `CS`
help: consider restricting type parameter `CS`
   |
   |
LL | impl<CS: CSpace<4>> Obstacle<CS, N> for ObstacleSpace2df32 {


error[E0277]: the trait bound `CS: CSpace<4>` is not satisfied
  --> fake-test-src-base/consts/unnormalized-param-env.rs:23:8
   |
LL |     fn trajectory_free<TF, S1>(&self, t: &TF)
   |        ^^^^^^^^^^^^^^^ the trait `CSpace<4>` is not implemented for `CS`
help: consider restricting type parameter `CS`
   |
   |
LL | impl<CS: CSpace<4>> Obstacle<CS, N> for ObstacleSpace2df32 {
Build completed unsuccessfully in 0:12:56

error: aborting due to 2 previous errors

