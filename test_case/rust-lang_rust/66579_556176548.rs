plain
2019-11-20T17:52:17.8492984Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T17:52:17.8505957Z ##[command]git config gc.auto 0
2019-11-20T17:52:17.8510007Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T17:52:17.8513336Z ##[command]git config --get-all http.proxy
2019-11-20T17:52:17.8518363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66579/merge:refs/remotes/pull/66579/merge
---
2019-11-20T17:57:54.8265910Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-11-20T17:57:54.8789202Z error: expected one of `extern` or `fn`, found keyword `const`
2019-11-20T17:57:54.8790038Z    --> src/libcore/mem/maybe_uninit.rs:485:16
2019-11-20T17:57:54.8790264Z     |
2019-11-20T17:57:54.8790550Z 485 |     pub unsafe const fn assume_init(self) -> T {
2019-11-20T17:57:54.8791126Z     |                ^^^^^ expected one of `extern` or `fn` here
2019-11-20T17:57:57.7735322Z error: unused import: `crate::intrinsics`
2019-11-20T17:57:57.7735768Z  --> src/libcore/mem/maybe_uninit.rs:1:5
2019-11-20T17:57:57.7735989Z   |
2019-11-20T17:57:57.7736194Z 1 | use crate::intrinsics;
2019-11-20T17:57:57.7736194Z 1 | use crate::intrinsics;
2019-11-20T17:57:57.7736427Z   |     ^^^^^^^^^^^^^^^^^
2019-11-20T17:57:57.7736598Z   |
2019-11-20T17:57:57.7736822Z   = note: `-D unused-imports` implied by `-D warnings`
2019-11-20T17:57:57.7736883Z 
2019-11-20T17:58:00.2426154Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-11-20T17:58:01.2591143Z    Compiling libc v0.2.64
2019-11-20T17:58:01.3199592Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<[mem::maybe_uninit::MaybeUninit<T>; _]>` in the current scope
2019-11-20T17:58:01.3200222Z     |
2019-11-20T17:58:01.3200452Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:01.3200761Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:01.3201114Z ...
2019-11-20T17:58:01.3201114Z ...
2019-11-20T17:58:01.3201397Z 297 |             MaybeUninit::<[MaybeUninit<T>; LEN]>::uninit().assume_init()
2019-11-20T17:58:01.3202196Z     |                                                            ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[mem::maybe_uninit::MaybeUninit<T>; _]>`
2019-11-20T17:58:01.3282655Z error[E0599]: no method named `get_mut` found for type `&mut mem::maybe_uninit::MaybeUninit<T>` in the current scope
2019-11-20T17:58:01.3282983Z    --> src/libcore/mem/maybe_uninit.rs:361:18
2019-11-20T17:58:01.3283190Z     |
2019-11-20T17:58:01.3283455Z 361 |             self.get_mut()
2019-11-20T17:58:01.3283455Z 361 |             self.get_mut()
2019-11-20T17:58:01.3283780Z     |                  ^^^^^^^ method not found in `&mut mem::maybe_uninit::MaybeUninit<T>`
2019-11-20T17:58:01.3283997Z     |
2019-11-20T17:58:01.3284289Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:01.3284580Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:01.3285363Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:01.3296270Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<T>` in the current scope
2019-11-20T17:58:01.3296746Z    --> src/libcore/ptr/mod.rs:617:9
2019-11-20T17:58:01.3296932Z     |
2019-11-20T17:58:01.3297153Z 617 |     tmp.assume_init()
---
2019-11-20T17:58:01.3311169Z     |
2019-11-20T17:58:01.3311395Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:01.3311884Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:01.3316378Z 
2019-11-20T17:58:01.6068276Z error[E0599]: no method named `read` found for type `&mem::maybe_uninit::MaybeUninit<T>` in the current scope
2019-11-20T17:58:01.6069047Z    --> src/libcore/array/iter.rs:138:57
2019-11-20T17:58:01.6069531Z     |
2019-11-20T17:58:01.6069818Z 138 |         let out = unsafe { self.data.get_unchecked(idx).read() };
2019-11-20T17:58:01.6070222Z     |                                                         ^^^^ method not found in `&mem::maybe_uninit::MaybeUninit<T>`
2019-11-20T17:58:01.6070265Z 
2019-11-20T17:58:01.6104815Z error[E0599]: no method named `read` found for type `&mem::maybe_uninit::MaybeUninit<T>` in the current scope
2019-11-20T17:58:01.6105102Z    --> src/libcore/array/iter.rs:187:68
2019-11-20T17:58:01.6105304Z     |
2019-11-20T17:58:01.6105609Z 187 |         let out = unsafe { self.data.get_unchecked(self.alive.end).read() };
2019-11-20T17:58:01.6106222Z     |                                                                    ^^^^ method not found in `&mem::maybe_uninit::MaybeUninit<T>`
2019-11-20T17:58:01.6158247Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:01.6158515Z    --> src/libcore/array/iter.rs:252:75
2019-11-20T17:58:01.6158728Z     |
2019-11-20T17:58:01.6158728Z     |
2019-11-20T17:58:01.6159180Z 252 |             let mut new_data: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
2019-11-20T17:58:01.6159538Z     |                                                                           ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:01.6160154Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:01.6160527Z     |
2019-11-20T17:58:01.6160779Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:01.6161074Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:01.6161074Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:01.6165242Z 
2019-11-20T17:58:01.6188537Z error[E0599]: no method named `get_ref` found for type `&mem::maybe_uninit::MaybeUninit<T>` in the current scope
2019-11-20T17:58:01.6188975Z    --> src/libcore/array/iter.rs:259:58
2019-11-20T17:58:01.6189177Z     |
2019-11-20T17:58:01.6189679Z 259 |                 let clone = self.data.get_unchecked(idx).get_ref().clone();
2019-11-20T17:58:01.6190066Z     |                                                          ^^^^^^^ method not found in `&mem::maybe_uninit::MaybeUninit<T>`
2019-11-20T17:58:01.9014606Z    Compiling autocfg v0.1.6
2019-11-20T17:58:01.9014606Z    Compiling autocfg v0.1.6
2019-11-20T17:58:02.1032407Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<ffi::VaListImpl<'_>>` in the current scope
2019-11-20T17:58:02.1033019Z    --> src/libcore/ffi.rs:321:18
2019-11-20T17:58:02.1034271Z 321 |             dest.assume_init()
2019-11-20T17:58:02.1034271Z 321 |             dest.assume_init()
2019-11-20T17:58:02.1035322Z     |                  ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<ffi::VaListImpl<'_>>`
2019-11-20T17:58:02.1035933Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.1036185Z     |
2019-11-20T17:58:02.1036465Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.1036812Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:02.1036812Z     | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:02.1036875Z 
2019-11-20T17:58:02.3150771Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.3151102Z    --> src/libcore/slice/sort.rs:267:36
2019-11-20T17:58:02.3151324Z     |
2019-11-20T17:58:02.3151588Z 267 |             start_l = MaybeUninit::first_ptr_mut(&mut offsets_l);
2019-11-20T17:58:02.3152154Z     |                                    ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.3152609Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.3152811Z     |
2019-11-20T17:58:02.3153036Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3153036Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3153330Z     | ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.3153386Z 
2019-11-20T17:58:02.3181743Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.3182038Z    --> src/libcore/slice/sort.rs:268:34
2019-11-20T17:58:02.3182254Z     |
2019-11-20T17:58:02.3182506Z 268 |             end_l = MaybeUninit::first_ptr_mut(&mut offsets_l);
2019-11-20T17:58:02.3182841Z     |                                  ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.3183258Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.3183458Z     |
2019-11-20T17:58:02.3183671Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3183671Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3183951Z     | ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.3184016Z 
2019-11-20T17:58:02.3219427Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.3219749Z    --> src/libcore/slice/sort.rs:283:36
2019-11-20T17:58:02.3219961Z     |
2019-11-20T17:58:02.3220436Z 283 |             start_r = MaybeUninit::first_ptr_mut(&mut offsets_r);
2019-11-20T17:58:02.3221073Z     |                                    ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.3221711Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.3221908Z     |
2019-11-20T17:58:02.3222463Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3222463Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3222772Z     | ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.3226146Z 
2019-11-20T17:58:02.3252871Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.3253374Z    --> src/libcore/slice/sort.rs:284:34
2019-11-20T17:58:02.3253738Z     |
2019-11-20T17:58:02.3254163Z 284 |             end_r = MaybeUninit::first_ptr_mut(&mut offsets_r);
2019-11-20T17:58:02.3255304Z     |                                  ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.3255780Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.3255979Z     |
2019-11-20T17:58:02.3256219Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3256219Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.3256563Z     | ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.6424887Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 1024]>` in the current scope
2019-11-20T17:58:02.6425360Z    --> src/libcore/fmt/float.rs:24:64
2019-11-20T17:58:02.6425607Z     |
2019-11-20T17:58:02.6425607Z     |
2019-11-20T17:58:02.6425930Z 24  | ...                   false, buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6426692Z     |                                  ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 1024]>`
2019-11-20T17:58:02.6427444Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6427660Z     |
2019-11-20T17:58:02.6427915Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6428251Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6428251Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6428477Z     |
2019-11-20T17:58:02.6428798Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6429280Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6429740Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6435225Z 
2019-11-20T17:58:02.6478396Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>` in the current scope
2019-11-20T17:58:02.6478785Z    --> src/libcore/fmt/float.rs:24:81
2019-11-20T17:58:02.6479044Z     |
2019-11-20T17:58:02.6479597Z 24  | ...                   false, buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6480172Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>`
2019-11-20T17:58:02.6480878Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6481101Z     |
2019-11-20T17:58:02.6481365Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6481667Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6481667Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6481890Z     |
2019-11-20T17:58:02.6482333Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6482632Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6482916Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6541766Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 17]>` in the current scope
2019-11-20T17:58:02.6542088Z    --> src/libcore/fmt/float.rs:42:78
2019-11-20T17:58:02.6542285Z     |
2019-11-20T17:58:02.6542285Z     |
2019-11-20T17:58:02.6542543Z 42  | ...                   sign, precision, false, buf.get_mut(),
2019-11-20T17:58:02.6542904Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 17]>`
2019-11-20T17:58:02.6543358Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6543541Z     |
2019-11-20T17:58:02.6543764Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6544065Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6544065Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6544429Z     |
2019-11-20T17:58:02.6544718Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6545003Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6545252Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6552215Z 
2019-11-20T17:58:02.6642211Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>` in the current scope
2019-11-20T17:58:02.6642518Z    --> src/libcore/fmt/float.rs:43:56
2019-11-20T17:58:02.6643220Z 43  | ...                   parts.get_mut());
2019-11-20T17:58:02.6643220Z 43  | ...                   parts.get_mut());
2019-11-20T17:58:02.6643655Z     |                             ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>`
2019-11-20T17:58:02.6644185Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6644532Z     |
2019-11-20T17:58:02.6644792Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6645106Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6645106Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6645334Z     |
2019-11-20T17:58:02.6645645Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6645955Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6646244Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6654061Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 1024]>` in the current scope
2019-11-20T17:58:02.6654387Z    --> src/libcore/fmt/float.rs:82:62
2019-11-20T17:58:02.6654604Z     |
2019-11-20T17:58:02.6654604Z     |
2019-11-20T17:58:02.6654905Z 82  | ...                   upper, buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6655327Z     |                                  ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 1024]>`
2019-11-20T17:58:02.6655839Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6656054Z     |
2019-11-20T17:58:02.6656316Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6656672Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6656672Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6656888Z     |
2019-11-20T17:58:02.6657195Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6657538Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6657977Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6664379Z 
2019-11-20T17:58:02.6664918Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>` in the current scope
2019-11-20T17:58:02.6665178Z    --> src/libcore/fmt/float.rs:82:79
2019-11-20T17:58:02.6665426Z     |
2019-11-20T17:58:02.6665715Z 82  | ...                   upper, buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6666113Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>`
2019-11-20T17:58:02.6666739Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6666999Z     |
2019-11-20T17:58:02.6667289Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6667600Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6667600Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6667843Z     |
2019-11-20T17:58:02.6668145Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6668451Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6668726Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6703847Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 17]>` in the current scope
2019-11-20T17:58:02.6704185Z    --> src/libcore/fmt/float.rs:102:58
2019-11-20T17:58:02.6704393Z     |
2019-11-20T17:58:02.6704393Z     |
2019-11-20T17:58:02.6704666Z 102 | ...                   buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6705244Z     |                           ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 17]>`
2019-11-20T17:58:02.6705719Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6705924Z     |
2019-11-20T17:58:02.6706171Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6706502Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6706502Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6706709Z     |
2019-11-20T17:58:02.6706999Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6707336Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6707600Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6712110Z 
2019-11-20T17:58:02.6752010Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>` in the current scope
2019-11-20T17:58:02.6752291Z    --> src/libcore/fmt/float.rs:102:75
2019-11-20T17:58:02.6752506Z     |
2019-11-20T17:58:02.6752781Z 102 | ...                   buf.get_mut(), parts.get_mut());
2019-11-20T17:58:02.6753139Z     |                                            ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>`
2019-11-20T17:58:02.6753601Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6753790Z     |
2019-11-20T17:58:02.6754048Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6754333Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6754333Z     | ------------------------ method `get_mut` not found for this
2019-11-20T17:58:02.6754540Z     |
2019-11-20T17:58:02.6755318Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-20T17:58:02.6755686Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-20T17:58:02.6755971Z             candidate #1: `slice::SliceIndex`
2019-11-20T17:58:02.6760390Z 
2019-11-20T17:58:02.6885253Z error[E0599]: no function or associated item named `first_ptr` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.6885589Z    --> src/libcore/fmt/num.rs:87:26
2019-11-20T17:58:02.6886068Z 87  |             MaybeUninit::first_ptr(buf),
2019-11-20T17:58:02.6886068Z 87  |             MaybeUninit::first_ptr(buf),
2019-11-20T17:58:02.6886544Z     |                          ^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.6887314Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.6887756Z     |
2019-11-20T17:58:02.6888464Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6888464Z 221 | pub union MaybeUninit<T> {
2019-11-20T17:58:02.6888978Z     | ------------------------ function or associated item `first_ptr` not found for this
2019-11-20T17:58:02.6893834Z 
2019-11-20T17:58:02.7179970Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.7180266Z    --> src/libcore/fmt/num.rs:196:40
2019-11-20T17:58:02.7180732Z 191 | / macro_rules! impl_Display {
2019-11-20T17:58:02.7180732Z 191 | / macro_rules! impl_Display {
2019-11-20T17:58:02.7181024Z 192 | |     ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
2019-11-20T17:58:02.7181355Z 193 | |         fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
2019-11-20T17:58:02.7181665Z 194 | |             let mut buf = [MaybeUninit::<u8>::uninit(); 39];
2019-11-20T17:58:02.7181934Z 195 | |             let mut curr = buf.len() as isize;
2019-11-20T17:58:02.7182247Z 196 | |             let buf_ptr = MaybeUninit::first_ptr_mut(&mut buf);
2019-11-20T17:58:02.7182606Z     | |                                        ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.7183093Z 259 | |     };
2019-11-20T17:58:02.7183327Z 260 | | }
2019-11-20T17:58:02.7183327Z 260 | | }
2019-11-20T17:58:02.7183608Z     | |_- in this expansion of `impl_Display!`
2019-11-20T17:58:02.7184157Z 267 | /     impl_Display!(
2019-11-20T17:58:02.7184157Z 267 | /     impl_Display!(
2019-11-20T17:58:02.7184701Z 268 | |         i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
2019-11-20T17:58:02.7185223Z 269 | |             as u64 via to_u64 named fmt_u64
2019-11-20T17:58:02.7185766Z     | |______- in this macro invocation
2019-11-20T17:58:02.7185952Z     | 
2019-11-20T17:58:02.7186195Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.7186380Z     |
2019-11-20T17:58:02.7186380Z     |
2019-11-20T17:58:02.7186632Z 221 |   pub union MaybeUninit<T> {
2019-11-20T17:58:02.7186970Z     |   ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.7191488Z 
2019-11-20T17:58:02.7356284Z error[E0599]: no function or associated item named `first_ptr_mut` found for type `mem::maybe_uninit::MaybeUninit<_>` in the current scope
2019-11-20T17:58:02.7356577Z    --> src/libcore/fmt/num.rs:196:40
2019-11-20T17:58:02.7357001Z 191 | / macro_rules! impl_Display {
2019-11-20T17:58:02.7357001Z 191 | / macro_rules! impl_Display {
2019-11-20T17:58:02.7357326Z 192 | |     ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
2019-11-20T17:58:02.7357638Z 193 | |         fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
2019-11-20T17:58:02.7357947Z 194 | |             let mut buf = [MaybeUninit::<u8>::uninit(); 39];
2019-11-20T17:58:02.7358215Z 195 | |             let mut curr = buf.len() as isize;
2019-11-20T17:58:02.7358493Z 196 | |             let buf_ptr = MaybeUninit::first_ptr_mut(&mut buf);
2019-11-20T17:58:02.7358869Z     | |                                        ^^^^^^^^^^^^^ function or associated item not found in `mem::maybe_uninit::MaybeUninit<_>`
2019-11-20T17:58:02.7359339Z 259 | |     };
2019-11-20T17:58:02.7359581Z 260 | | }
2019-11-20T17:58:02.7359581Z 260 | | }
2019-11-20T17:58:02.7359833Z     | |_- in this expansion of `impl_Display!`
2019-11-20T17:58:02.7360024Z ...
2019-11-20T17:58:02.7360282Z 280 |   impl_Display!(i128, u128 as u128 via to_u128 named fmt_u128);
2019-11-20T17:58:02.7360991Z     | 
2019-11-20T17:58:02.7361206Z    ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:02.7361409Z     |
2019-11-20T17:58:02.7361639Z 221 |   pub union MaybeUninit<T> {
2019-11-20T17:58:02.7361639Z 221 |   pub union MaybeUninit<T> {
2019-11-20T17:58:02.7361969Z     |   ------------------------ function or associated item `first_ptr_mut` not found for this
2019-11-20T17:58:02.9867856Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-11-20T17:58:02.9867856Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-11-20T17:58:03.1318639Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128>` in the current scope
2019-11-20T17:58:03.1319046Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/sse.rs:1867:42
2019-11-20T17:58:03.1319275Z      |
2019-11-20T17:58:03.1319746Z 1867 |     mem::MaybeUninit::<__m128>::uninit().assume_init()
2019-11-20T17:58:03.1320115Z      |                                          ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128>`
2019-11-20T17:58:03.1320839Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.1321214Z      |
2019-11-20T17:58:03.1321479Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.1321801Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.1321801Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.1321841Z 
2019-11-20T17:58:03.3638890Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128d>` in the current scope
2019-11-20T17:58:03.3639249Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/sse2.rs:2862:43
2019-11-20T17:58:03.3639537Z      |
2019-11-20T17:58:03.3639981Z 2862 |     mem::MaybeUninit::<__m128d>::uninit().assume_init()
2019-11-20T17:58:03.3640318Z      |                                           ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128d>`
2019-11-20T17:58:03.3640767Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.3640976Z      |
2019-11-20T17:58:03.3641197Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.3641474Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.3641474Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.3645521Z 
2019-11-20T17:58:03.3667094Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128i>` in the current scope
2019-11-20T17:58:03.3667380Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/sse2.rs:2873:43
2019-11-20T17:58:03.3667601Z      |
2019-11-20T17:58:03.3667845Z 2873 |     mem::MaybeUninit::<__m128i>::uninit().assume_init()
2019-11-20T17:58:03.3668356Z      |                                           ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m128i>`
2019-11-20T17:58:03.3668835Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.3669034Z      |
2019-11-20T17:58:03.3669246Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.3669513Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.3669513Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.3673219Z 
2019-11-20T17:58:03.3832643Z    Compiling cmake v0.1.38
2019-11-20T17:58:03.7474498Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256>` in the current scope
2019-11-20T17:58:03.7475011Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/avx.rs:2960:42
2019-11-20T17:58:03.7475467Z      |
2019-11-20T17:58:03.7475778Z 2960 |     mem::MaybeUninit::<__m256>::uninit().assume_init()
2019-11-20T17:58:03.7476559Z      |                                          ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256>`
2019-11-20T17:58:03.7477041Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.7477239Z      |
2019-11-20T17:58:03.7477504Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.7478206Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.7478206Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.7482857Z 
2019-11-20T17:58:03.7511211Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256d>` in the current scope
2019-11-20T17:58:03.7511730Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/avx.rs:2972:43
2019-11-20T17:58:03.7511970Z      |
2019-11-20T17:58:03.7512254Z 2972 |     mem::MaybeUninit::<__m256d>::uninit().assume_init()
2019-11-20T17:58:03.7512646Z      |                                           ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256d>`
2019-11-20T17:58:03.7513132Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.7513333Z      |
2019-11-20T17:58:03.7513605Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.7513925Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.7513925Z      | ------------------------ method `assume_init` not found for this
2019-11-20T17:58:03.7513964Z 
2019-11-20T17:58:03.7550070Z error[E0599]: no method named `assume_init` found for type `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256i>` in the current scope
2019-11-20T17:58:03.7550378Z     --> src/libcore/../stdarch/crates/core_arch/src/x86/avx.rs:2984:43
2019-11-20T17:58:03.7550600Z      |
2019-11-20T17:58:03.7551457Z 2984 |     mem::MaybeUninit::<__m256i>::uninit().assume_init()
2019-11-20T17:58:03.7551889Z      |                                           ^^^^^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<core_arch::x86::__m256i>`
2019-11-20T17:58:03.7552392Z     ::: src/libcore/mem/maybe_uninit.rs:221:1
2019-11-20T17:58:03.7552616Z      |
2019-11-20T17:58:03.7552866Z 221  | pub union MaybeUninit<T> {
2019-11-20T17:58:03.7553171Z      | ------------------------ method `assume_init` not found for this
---
2019-11-20T17:58:05.3393938Z   local time: Wed Nov 20 17:58:05 UTC 2019
2019-11-20T17:58:05.6412901Z   network time: Wed, 20 Nov 2019 17:58:05 GMT
2019-11-20T17:58:05.6417696Z == end clock drift check ==
2019-11-20T17:58:07.2387168Z 
2019-11-20T17:58:07.2521577Z ##[error]Bash exited with code '1'.
2019-11-20T17:58:07.2544653Z ##[section]Starting: Checkout
2019-11-20T17:58:07.2546058Z ==============================================================================
2019-11-20T17:58:07.2546120Z Task         : Get sources
2019-11-20T17:58:07.2546156Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
