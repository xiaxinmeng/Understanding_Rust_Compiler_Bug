
   Compiling rustc-ap-rustc_span v659.0.0
error[E0407]: method `forward_checked` is not a member of trait `std::iter::Step`
  --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:11:1
   |
11 | / rustc_index::newtype_index! {
12 | |     pub struct CrateId {
13 | |         ENCODABLE = custom
14 | |     }
15 | | }
   | |_^ not a member of trait `std::iter::Step`
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `backward_checked` is not a member of trait `std::iter::Step`
  --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:11:1
   |
11 | / rustc_index::newtype_index! {
12 | |     pub struct CrateId {
13 | |         ENCODABLE = custom
14 | |     }
15 | | }
   | |_^ not a member of trait `std::iter::Step`
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `forward_checked` is not a member of trait `std::iter::Step`
   --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:118:1
    |
118 | / rustc_index::newtype_index! {
119 | |     /// A DefIndex is an index into the hir-map for a crate, identifying a
120 | |     /// particular definition. It should really be considered an interned
121 | |     /// shorthand for a particular DefPath.
...   |
128 | |     }
129 | | }
    | |_^ not a member of trait `std::iter::Step`
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `backward_checked` is not a member of trait `std::iter::Step`
   --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:118:1
    |
118 | / rustc_index::newtype_index! {
119 | |     /// A DefIndex is an index into the hir-map for a crate, identifying a
120 | |     /// particular definition. It should really be considered an interned
121 | |     /// shorthand for a particular DefPath.
...   |
128 | |     }
129 | | }
    | |_^ not a member of trait `std::iter::Step`
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `forward_checked` is not a member of trait `std::iter::Step`
    --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/symbol.rs:1034:1
     |
1034 | / rustc_index::newtype_index! {
1035 | |     pub struct SymbolIndex { .. }
1036 | | }
     | |_^ not a member of trait `std::iter::Step`
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `backward_checked` is not a member of trait `std::iter::Step`
    --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/symbol.rs:1034:1
     |
1034 | / rustc_index::newtype_index! {
1035 | |     pub struct SymbolIndex { .. }
1036 | | }
     | |_^ not a member of trait `std::iter::Step`
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0199]: implementing the trait `std::iter::Step` is not unsafe
  --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:11:1
   |
11 | / rustc_index::newtype_index! {
12 | |     pub struct CrateId {
13 | |         ENCODABLE = custom
14 | |     }
15 | | }
   | |_^
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0199]: implementing the trait `std::iter::Step` is not unsafe
   --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/def_id.rs:118:1
    |
118 | / rustc_index::newtype_index! {
119 | |     /// A DefIndex is an index into the hir-map for a crate, identifying a
120 | |     /// particular definition. It should really be considered an interned
121 | |     /// shorthand for a particular DefPath.
...   |
128 | |     }
129 | | }
    | |_^
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0199]: implementing the trait `std::iter::Step` is not unsafe
    --> /Users/cameron/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_span-659.0.0/symbol.rs:1034:1
     |
1034 | / rustc_index::newtype_index! {
1035 | |     pub struct SymbolIndex { .. }
1036 | | }
     | |_^
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 9 previous errors
