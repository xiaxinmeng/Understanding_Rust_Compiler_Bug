
$ rustc -vV
rustc 1.57.0-nightly (5b210643e 2021-10-11)
binary: rustc
commit-hash: 5b210643ebf2485aafdf2494de8cf41941a64e95
commit-date: 2021-10-11
host: x86_64-unknown-linux-gnu
release: 1.57.0-nightly
LLVM version: 13.0.0
$ rustc src/test/ui/traits/issue-88586.rs -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
error[E0311]: the parameter type `Self` may not live long enough
  --> src/test/ui/traits/issue-88586.rs:5:1
   |
LL | / trait A where
LL | | //~^ ERROR the parameter type `Self` may not live long enough [E0311]
LL | | //~| ERROR the parameter type `Self` may not live long enough [E0311]
LL | |     for<'a> Self: 'a,
LL | | {
LL | | }
   | |_^
   |
   = help: consider adding an explicit lifetime bound `Self: 'a`...
   = note: ...so that the type `Self` will meet its required lifetime bounds...
note: ...that is required by this bound
  --> src/test/ui/traits/issue-88586.rs:8:19
   |
LL |     for<'a> Self: 'a,
   |                   ^^

error[E0311]: the parameter type `Self` may not live long enough
  --> src/test/ui/traits/issue-88586.rs:5:1
   |
LL | / trait A where
LL | | //~^ ERROR the parameter type `Self` may not live long enough [E0311]
LL | | //~| ERROR the parameter type `Self` may not live long enough [E0311]
LL | |     for<'a> Self: 'a,
LL | | {
LL | | }
   | |_^
   |
   = help: consider adding an explicit lifetime bound `Self: 'a`...
   = note: ...so that the type `Self` will meet its required lifetime bounds...
note: ...that is required by this bound
  --> src/test/ui/traits/issue-88586.rs:8:19
   |
LL |     for<'a> Self: 'a,
   |                   ^^

error: aborting due to 2 previous errors
