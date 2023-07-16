plain
travis_time:end:2885cb31:start=1560549390561877673,finish=1560549500272797708,duration=109710920035
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:18] .................................................................................................... 4000/5690
[00:58:21] ....i............................................................................................... 4100/5690
[00:58:23] ....................................................................i............................... 4200/5690
[00:58:25] .................................................................................................... 4300/5690
[00:58:33] .....................................................F.............................................. 4400/5690
[00:58:47] .................................................................................................... 4600/5690
[00:58:50] .................................................................................................... 4700/5690
[00:58:54] .................................................................................................... 4800/5690
[00:58:54] .................................................................................................... 4800/5690
[00:59:01] ........F........................................................................................... 4900/5690
[00:59:09] .................................................................................................... 5100/5690
[00:59:14] .................................................................................................... 5200/5690
[00:59:18] .................................................................................................... 5300/5690
[00:59:22] .................................................................................................... 5400/5690
[00:59:22] .................................................................................................... 5400/5690
[00:59:24] .................................................................................................... 5500/5690
[00:59:27] .................................................................................................... 5600/5690
[00:59:30] ............................i...............................F.F...........................
[00:59:30] 
[00:59:30] ---- [ui] ui/privacy/private-in-public-warn.rs stdout ----
[00:59:30] diff of stderr:
[00:59:30] 
[00:59:30] 
[00:59:30] 31    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 32 
[00:59:30] 33 error: private type `types::Priv` in public interface (error E0446)
[00:59:30] +    |
[00:59:30] + LL | /     pub trait Tr {
[00:59:30] + LL | /     pub trait Tr {
[00:59:30] + LL | |         const C: Priv = Priv;
[00:59:30] + LL | |
[00:59:30] + LL | |         type Alias = Priv;
[00:59:30] + ...  |
[00:59:30] + LL | |
[00:59:30] + LL | |     }
[00:59:30] +    |
[00:59:30] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30] +    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] + 
[00:59:30] + 
[00:59:30] + error: private type `types::Priv` in public interface (error E0446)
[00:59:30] 35    |
[00:59:30] 35    |
[00:59:30] 36 LL |         const C: Priv = Priv;
[00:59:30] 350    |
[00:59:30] 351    = help: the clause will not be checked when the type alias is used, and should be removed
[00:59:30] 352 
[00:59:30] - error: aborting due to 36 previous errors
[00:59:30] - error: aborting due to 36 previous errors
[00:59:30] + error: aborting due to 37 previous errors
[00:59:30] 354 
[00:59:30] 355 For more information about this error, try `rustc --explain E0446`.
[00:59:30] 356 
[00:59:30] 
[00:59:30] 
[00:59:30] The actual stderr differed from the expected stderr.
[00:59:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/private-in-public-warn.stderr
[00:59:30] To update references, rerun the tests and pass the `--bless` flag
[00:59:30] To only update this specific test, also pass `--test-args privacy/private-in-public-warn.rs`
[00:59:30] error: 1 errors occurred comparing output.
[00:59:30] status: exit code: 1
[00:59:30] status: exit code: 1
[00:59:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-in-public-warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/auxiliary" "-A" "unused"
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] stderr:
[00:59:30] stderr:
[00:59:30] ------------------------------------------
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30] note: lint level defined here
[00:59:30]   --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:5:9
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | #![deny(private_in_public)]
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         V1(Priv), //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         V2 { field: Priv }, //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30] LL | /     pub trait Tr {
[00:59:30] LL | /     pub trait Tr {
[00:59:30] LL | |         const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30] LL | |         //~^ WARNING hard error
[00:59:30] LL | |         type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30] ...  |
[00:59:30] LL | |         //~^ WARNING hard error
[00:59:30]    | |_____^
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `types::Priv` in public interface
[00:59:30]   --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:26:9
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `types::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         fn f1(arg: Priv) {} //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         fn f2() -> Priv { panic!() } //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         pub static ES: Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         pub fn ef1(arg: Priv); //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `types::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         pub fn ef2() -> Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `types::Priv` in public interface
[00:59:30]   --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:41:9
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `types::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub type Alias<T: PrivTr> = T; //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr1: PrivTr {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr2<T: PrivTr> {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30] LL | /     pub trait Tr3 {
[00:59:30] LL | /     pub trait Tr3 {
[00:59:30] LL | |         //~^ ERROR private trait `traits::PrivTr` in public interface
[00:59:30] LL | |         //~| WARNING hard error
[00:59:30] LL | |         type Alias: PrivTr;
[00:59:30] LL | |         fn f<T: PrivTr>(arg: T) {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30] LL | |         //~^ WARNING hard error
[00:59:30]    | |_____^
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         fn f<T: PrivTr>(arg: T) {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     impl<T: PrivTr> Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     impl<T: PrivTr> PubTr for Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits_where::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub type Alias<T> where T: PrivTr = T;
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits_where::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr2<T> where T: PrivTr {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits_where::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         fn f<T>(arg: T) where T: PrivTr {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits_where::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     impl<T> Pub<T> where T: PrivTr {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `traits_where::PrivTr` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     impl<T> PubTr for Pub<T> where T: PrivTr {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `generics::PrivTr<generics::Pub>` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr1: PrivTr<Pub> {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `generics::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr2: PubTr<Priv> {} //~ ERROR private type `generics::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `generics::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr3: PubTr<[Priv; 1]> {} //~ ERROR private type `generics::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `generics::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr4: PubTr<Pub<Priv>> {} //~ ERROR private type `generics::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `impls::Priv` in public interface
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `impls::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Alias = Priv; //~ ERROR private type `impls::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `aliases_pub::Priv` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |         pub fn f(arg: Priv) {} //~ ERROR private type `aliases_pub::Priv` in public interface
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `aliases_pub::Priv` in public interface
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `aliases_pub::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `aliases_pub::Priv` in public interface
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `aliases_pub::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `aliases_pub::Priv` in public interface
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `aliases_pub::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0446]: private type `aliases_pub::Priv` in public interface
[00:59:30]    |
[00:59:30] LL |     struct Priv;
[00:59:30] LL |     struct Priv;
[00:59:30]    |     - `aliases_pub::Priv` declared as private
[00:59:30] ...
[00:59:30] LL |         type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `aliases_priv::PrivTr1` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr1: PrivUseAliasTr {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private trait `aliases_priv::PrivTr1<aliases_priv::Priv2>` in public interface (error E0445)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] error: private type `aliases_priv::Priv2` in public interface (error E0446)
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
[00:59:30]    |
[00:59:30]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:59:30]    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[00:59:30] 
[00:59:30] 
[00:59:30] warning: bounds on generic parameters are not enforced in type aliases
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     pub type Alias<T: PrivTr> = T; //~ ERROR private trait `traits::PrivTr` in public interface
[00:59:30]    |
[00:59:30]    = note: #[warn(type_alias_bounds)] on by default
[00:59:30]    = help: the bound will not be checked when the type alias is used, and should be removed
[00:59:30] 
[00:59:30] 
[00:59:30] warning: where clauses are not enforced in type aliases
[00:59:30]   --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:75:29
[00:59:30]    |
[00:59:30] LL |     pub type Alias<T> where T: PrivTr = T;
[00:59:30]    |
[00:59:30]    = help: the clause will not be checked when the type alias is used, and should be removed
[00:59:30] 
[00:59:30] error: aborting due to 37 previous errors
---
[00:59:30] 6 
[00:59:30] 7 error[E0109]: type arguments are not allowed for this type
[00:59:30] +   --> $DIR/collections.rs:17:71
[00:59:30] +    |
[00:59:30] + LL |         <<Self as Collection<T>>::Family as CollectionFamily>::Member<U>;
[00:59:30] +    |                                                                       ^ type argument not allowed
[00:59:30] + error[E0109]: type arguments are not allowed for this type
[00:59:30] 8   --> $DIR/collections.rs:56:90
[00:59:30] 9    |
[00:59:30] 9    |
[00:59:30] 10 LL | fn floatify<C>(ints: &C) -> <<C as Collection<i32>>::Family as CollectionFamily>::Member<f32>
[00:59:30] 15    |
[00:59:30] 15    |
[00:59:30] 16 LL | fn floatify_sibling<C>(ints: &C) -> <C as Collection<i32>>::Sibling<f32>
[00:59:30] - 
[00:59:30] - error[E0109]: type arguments are not allowed for this type
[00:59:30] -   --> $DIR/collections.rs:17:71
[00:59:30] -    |
[00:59:30] -    |
[00:59:30] - LL |         <<Self as Collection<T>>::Family as CollectionFamily>::Member<U>;
[00:59:30] -    |                                                                       ^ type argument not allowed
[00:59:30] 25 error[E0109]: lifetime arguments are not allowed for this type
[00:59:30] 26   --> $DIR/collections.rs:24:50
[00:59:30] 
[00:59:30] 
[00:59:30] 
[00:59:30] The actual stderr differed from the expected stderr.
[00:59:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/collections.stderr
[00:59:30] To update references, rerun the tests and pass the `--bless` flag
[00:59:30] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/collections.rs`
[00:59:30] error: 1 errors occurred comparing output.
[00:59:30] status: exit code: 1
[00:59:30] status: exit code: 1
[00:59:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/auxiliary" "-A" "unused"
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] stderr:
---
[00:59:30] 
[00:59:30] error[E0109]: type arguments are not allowed for this type
[00:59:30]   --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:17:71
[00:59:30]    |
[00:59:30] LL |         <<Self as Collection<T>>::Family as CollectionFamily>::Member<U>;
[00:59:30]    |                                                                       ^ type argument not allowed
[00:59:30] error[E0109]: type arguments are not allowed for this type
[00:59:30]   --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:56:90
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | fn floatify<C>(ints: &C) -> <<C as Collection<i32>>::Family as CollectionFamily>::Member<f32>
[00:59:30] 
[00:59:30] error[E0109]: type arguments are not allowed for this type
[00:59:30]   --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:68:69
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | fn floatify_sibling<C>(ints: &C) -> <C as Collection<i32>>::Sibling<f32>
[00:59:30] 
[00:59:30] error[E0109]: lifetime arguments are not allowed for this type
[00:59:30]   --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:24:50
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     fn iterate<'iter>(&'iter self) -> Self::Iter<'iter>;
[00:59:30] 
[00:59:30] error[E0109]: lifetime arguments are not allowed for this type
[00:59:30]   --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:50:50
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     fn iterate<'iter>(&'iter self) -> Self::Iter<'iter> {
[00:59:30] 
[00:59:30] error: aborting due to 5 previous errors
[00:59:30] 
[00:59:30] For more information about this error, try `rustc --explain E0109`.
[00:59:30] For more information about this error, try `rustc --explain E0109`.
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] 
[00:59:30] ---- [ui] ui/wf/wf-trait-associated-type-region.rs stdout ----
[00:59:30] diff of stderr:
[00:59:30] 
[00:59:30] 1 error[E0309]: the associated type `<Self as SomeTrait<'a>>::Type1` may not live long enough
[00:59:30] +   --> $DIR/wf-trait-associated-type-region.rs:7:1
[00:59:30] +    |
[00:59:30] + LL | / trait SomeTrait<'a> {
[00:59:30] + LL | |     type Type1;
[00:59:30] + LL | |     type Type2 = &'a Self::Type1;
[00:59:30] + LL | |
[00:59:30] + LL | | }
[00:59:30] +    |
[00:59:30] +    |
[00:59:30] +    = help: consider adding an explicit lifetime bound `<Self as SomeTrait<'a>>::Type1: 'a`...
[00:59:30] + note: ...so that the reference type `&'a <Self as SomeTrait<'a>>::Type1` does not outlive the data it points at
[00:59:30] +   --> $DIR/wf-trait-associated-type-region.rs:7:1
[00:59:30] +    |
[00:59:30] + LL | / trait SomeTrait<'a> {
[00:59:30] + LL | |     type Type1;
[00:59:30] + LL | |     type Type2 = &'a Self::Type1;
[00:59:30] + LL | |
[00:59:30] + LL | | }
[00:59:30] + 
[00:59:30] + 
[00:59:30] + error[E0309]: the associated type `<Self as SomeTrait<'a>>::Type1` may not live long enough
[00:59:30] 2   --> $DIR/wf-trait-associated-type-region.rs:9:5
[00:59:30] 4 LL |     type Type2 = &'a Self::Type1;
[00:59:30] 
[00:59:30] 11 LL |     type Type2 = &'a Self::Type1;
[00:59:30] 12    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
---
[00:59:30] 17 
[00:59:30] 
[00:59:30] 
[00:59:30] The actual stderr differed from the expected stderr.
[00:59:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-region/wf-trait-associated-type-region.stderr
[00:59:30] To update references, rerun the tests and pass the `--bless` flag
[00:59:30] To only update this specific test, also pass `--test-args wf/wf-trait-associated-type-region.rs`
[00:59:30] error: 1 errors occurred comparing output.
[00:59:30] status: exit code: 1
[00:59:30] status: exit code: 1
[00:59:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-trait-associated-type-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-region" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-region/auxiliary" "-A" "unused"
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] stderr:
[00:59:30] stderr:
[00:59:30] ------------------------------------------
[00:59:30] error[E0309]: the associated type `<Self as SomeTrait<'a>>::Type1` may not live long enough
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | / trait SomeTrait<'a> {
[00:59:30] LL | |     type Type1;
[00:59:30] LL | |     type Type2 = &'a Self::Type1;
[00:59:30] LL | |     //~^ ERROR E0309
[00:59:30]    | |_^
[00:59:30]    |
[00:59:30]    |
[00:59:30]    = help: consider adding an explicit lifetime bound `<Self as SomeTrait<'a>>::Type1: 'a`...
[00:59:30] note: ...so that the reference type `&'a <Self as SomeTrait<'a>>::Type1` does not outlive the data it points at
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | / trait SomeTrait<'a> {
[00:59:30] LL | |     type Type1;
[00:59:30] LL | |     type Type2 = &'a Self::Type1;
[00:59:30] LL | |     //~^ ERROR E0309
[00:59:30]    | |_^
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0309]: the associated type `<Self as SomeTrait<'a>>::Type1` may not live long enough
[00:59:30]    |
[00:59:30] LL |     type Type2 = &'a Self::Type1;
[00:59:30]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:30]    |
[00:59:30]    |
[00:59:30]    = help: consider adding an explicit lifetime bound `<Self as SomeTrait<'a>>::Type1: 'a`...
[00:59:30] note: ...so that the reference type `&'a <Self as SomeTrait<'a>>::Type1` does not outlive the data it points at
[00:59:30]    |
[00:59:30] LL |     type Type2 = &'a Self::Type1;
[00:59:30]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:30] 
---
[00:59:30] 
[00:59:30] ---- [ui] ui/wf/wf-trait-associated-type-trait.rs stdout ----
[00:59:30] diff of stderr:
[00:59:30] 
[00:59:30] 1 error[E0277]: the trait bound `<Self as SomeTrait>::Type1: std::marker::Copy` is not satisfied
[00:59:30] +   --> $DIR/wf-trait-associated-type-trait.rs:9:1
[00:59:30] +    |
[00:59:30] + LL | / trait SomeTrait {
[00:59:30] + LL | |     type Type1;
[00:59:30] + LL | |     type Type2 = IsCopy<Self::Type1>;
[00:59:30] + LL | |
[00:59:30] + LL | | }
[00:59:30] +    | |_^ the trait `std::marker::Copy` is not implemented for `<Self as SomeTrait>::Type1`
[00:59:30] +    |
[00:59:30] +    = help: consider adding a `where <Self as SomeTrait>::Type1: std::marker::Copy` bound
[00:59:30] + note: required by `IsCopy`
[00:59:30] +   --> $DIR/wf-trait-associated-type-trait.rs:7:1
[00:59:30] +    |
[00:59:30] + LL | struct IsCopy<T:Copy> { x: T }
[00:59:30] + 
[00:59:30] + 
[00:59:30] + error[E0277]: the trait bound `<Self as SomeTrait>::Type1: std::marker::Copy` is not satisfied
[00:59:30] 2   --> $DIR/wf-trait-associated-type-trait.rs:11:5
[00:59:30] 3    |
[00:59:30] 4 LL |     type Type2 = IsCopy<Self::Type1>;
[00:59:30] 
[00:59:30] 11 LL | struct IsCopy<T:Copy> { x: T }
[00:59:30] 13 
[00:59:30] - error: aborting due to previous error
[00:59:30] + error: aborting due to 2 previous errors
[00:59:30] 15 
[00:59:30] 15 
[00:59:30] 16 For more information about this error, try `rustc --explain E0277`.
[00:59:30] 17 
[00:59:30] 
[00:59:30] 
[00:59:30] The actual stderr differed from the expected stderr.
[00:59:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-trait/wf-trait-associated-type-trait.stderr
[00:59:30] To update references, rerun the tests and pass the `--bless` flag
[00:59:30] To only update this specific test, also pass `--test-args wf/wf-trait-associated-type-trait.rs`
[00:59:30] error: 1 errors occurred comparing output.
[00:59:30] status: exit code: 1
[00:59:30] status: exit code: 1
[00:59:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-trait-associated-type-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-trait-associated-type-trait/auxiliary" "-A" "unused"
[00:59:30] ------------------------------------------
[00:59:30] 
[00:59:30] ------------------------------------------
[00:59:30] stderr:
[00:59:30] stderr:
[00:59:30] ------------------------------------------
[00:59:30] error[E0277]: the trait bound `<Self as SomeTrait>::Type1: std::marker::Copy` is not satisfied
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | / trait SomeTrait {
[00:59:30] LL | |     type Type1;
[00:59:30] LL | |     type Type2 = IsCopy<Self::Type1>;
[00:59:30] LL | |     //~^ ERROR E0277
[00:59:30] LL | | }
[00:59:30]    | |_^ the trait `std::marker::Copy` is not implemented for `<Self as SomeTrait>::Type1`
[00:59:30]    |
[00:59:30]    = help: consider adding a `where <Self as SomeTrait>::Type1: std::marker::Copy` bound
[00:59:30] note: required by `IsCopy`
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | struct IsCopy<T:Copy> { x: T }
[00:59:30] 
[00:59:30] 
[00:59:30] error[E0277]: the trait bound `<Self as SomeTrait>::Type1: std::marker::Copy` is not satisfied
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL |     type Type2 = IsCopy<Self::Type1>;
[00:59:30]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `<Self as SomeTrait>::Type1`
[00:59:30]    |
[00:59:30]    = help: consider adding a `where <Self as SomeTrait>::Type1: std::marker::Copy` bound
[00:59:30] note: required by `IsCopy`
[00:59:30]    |
[00:59:30]    |
[00:59:30] LL | struct IsCopy<T:Copy> { x: T }
[00:59:30] 
[00:59:30] error: aborting due to 2 previous errors
[00:59:30] 
[00:59:30] For more information about this error, try `rustc --explain E0277`.
---
[00:59:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:30] 
[00:59:30] 
[00:59:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:30] 
[00:59:30] 
[00:59:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:30] Build completed unsuccessfully in 0:55:26
---
travis_time:end:02e85650:start=1560553081547391124,finish=1560553081552883060,duration=5491936
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05c27c1c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e0d32fe
travis_time:start:0e0d32fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ec51ed0
$ dmesg | grep -i kill
