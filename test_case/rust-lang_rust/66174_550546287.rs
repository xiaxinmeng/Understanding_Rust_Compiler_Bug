plain
2019-11-06T23:14:26.9570064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T23:14:26.9798492Z ##[command]git config gc.auto 0
2019-11-06T23:14:26.9869103Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T23:14:26.9916873Z ##[command]git config --get-all http.proxy
2019-11-06T23:14:27.0055080Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66174/merge:refs/remotes/pull/66174/merge
---
2019-11-06T23:21:13.5923498Z    Compiling autocfg v0.1.6
2019-11-06T23:21:14.3262475Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 1024]>` in the current scope
2019-11-06T23:21:14.3262949Z    --> src/libcore/fmt/float.rs:22:64
2019-11-06T23:21:14.3287725Z     |
2019-11-06T23:21:14.3288372Z 22  | ...                   false, buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3288670Z     |                                  ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 1024]>`
2019-11-06T23:21:14.3289279Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3289464Z     |
2019-11-06T23:21:14.3289683Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3289985Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3289985Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3290172Z     |
2019-11-06T23:21:14.3290411Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3290653Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3291071Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3291243Z 
2019-11-06T23:21:14.3321578Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>` in the current scope
2019-11-06T23:21:14.3322762Z    --> src/libcore/fmt/float.rs:22:81
2019-11-06T23:21:14.3323875Z     |
2019-11-06T23:21:14.3325978Z 22  | ...                   false, buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3327375Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>`
2019-11-06T23:21:14.3329780Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3330698Z     |
2019-11-06T23:21:14.3331941Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3333563Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3333563Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3335443Z     |
2019-11-06T23:21:14.3337055Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3338679Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3339892Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3407008Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 17]>` in the current scope
2019-11-06T23:21:14.3408989Z    --> src/libcore/fmt/float.rs:40:78
2019-11-06T23:21:14.3410348Z     |
2019-11-06T23:21:14.3410348Z     |
2019-11-06T23:21:14.3411786Z 40  | ...                   sign, precision, false, buf.get_mut(),
2019-11-06T23:21:14.3413341Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 17]>`
2019-11-06T23:21:14.3416626Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3418569Z     |
2019-11-06T23:21:14.3419731Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3420922Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3420922Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3422474Z     |
2019-11-06T23:21:14.3423734Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3425832Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3427601Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3428931Z 
2019-11-06T23:21:14.3483438Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>` in the current scope
2019-11-06T23:21:14.3483998Z    --> src/libcore/fmt/float.rs:41:56
2019-11-06T23:21:14.3485351Z 41  | ...                   parts.get_mut());
2019-11-06T23:21:14.3485351Z 41  | ...                   parts.get_mut());
2019-11-06T23:21:14.3485978Z     |                             ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 4]>`
2019-11-06T23:21:14.3486868Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3487316Z     |
2019-11-06T23:21:14.3487958Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3488558Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3488558Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3488903Z     |
2019-11-06T23:21:14.3489459Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3489871Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3490273Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3558505Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 1024]>` in the current scope
2019-11-06T23:21:14.3559331Z    --> src/libcore/fmt/float.rs:80:62
2019-11-06T23:21:14.3559685Z     |
2019-11-06T23:21:14.3559685Z     |
2019-11-06T23:21:14.3560089Z 80  | ...                   upper, buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3560543Z     |                                  ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 1024]>`
2019-11-06T23:21:14.3561261Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3561723Z     |
2019-11-06T23:21:14.3562318Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3562715Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3562715Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3563024Z     |
2019-11-06T23:21:14.3563412Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3563794Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3564162Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3564923Z 
2019-11-06T23:21:14.3615595Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>` in the current scope
2019-11-06T23:21:14.3616211Z    --> src/libcore/fmt/float.rs:80:79
2019-11-06T23:21:14.3616639Z     |
2019-11-06T23:21:14.3617127Z 80  | ...                   upper, buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3617874Z     |                                                   ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>`
2019-11-06T23:21:14.3618523Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3618844Z     |
2019-11-06T23:21:14.3619207Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3619616Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3619616Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3619925Z     |
2019-11-06T23:21:14.3620302Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3620681Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3621204Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3679709Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[u8; 17]>` in the current scope
2019-11-06T23:21:14.3680360Z    --> src/libcore/fmt/float.rs:100:58
2019-11-06T23:21:14.3682026Z     |
2019-11-06T23:21:14.3682026Z     |
2019-11-06T23:21:14.3682513Z 100 | ...                   buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3682996Z     |                           ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[u8; 17]>`
2019-11-06T23:21:14.3683697Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3684497Z     |
2019-11-06T23:21:14.3685019Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3685526Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3685526Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3685974Z     |
2019-11-06T23:21:14.3686649Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3687259Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3688105Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.3688517Z 
2019-11-06T23:21:14.3740943Z error[E0599]: no method named `get_mut` found for type `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>` in the current scope
2019-11-06T23:21:14.3741782Z    --> src/libcore/fmt/float.rs:100:75
2019-11-06T23:21:14.3742153Z     |
2019-11-06T23:21:14.3742516Z 100 | ...                   buf.get_mut(), parts.get_mut());
2019-11-06T23:21:14.3743661Z     |                                            ^^^^^^^ method not found in `mem::maybe_uninit::MaybeUninit<[num::flt2dec::Part<'_>; 6]>`
2019-11-06T23:21:14.3744967Z    ::: src/libcore/mem/maybe_uninit.rs:219:1
2019-11-06T23:21:14.3745375Z     |
2019-11-06T23:21:14.3745844Z 219 | pub union MaybeUninit<T> {
2019-11-06T23:21:14.3746372Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3746372Z     | ------------------------ method `get_mut` not found for this
2019-11-06T23:21:14.3746971Z     |
2019-11-06T23:21:14.3747506Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-11-06T23:21:14.3748135Z     = note: the following trait defines an item `get_mut`, perhaps you need to implement it:
2019-11-06T23:21:14.3748539Z             candidate #1: `slice::SliceIndex`
2019-11-06T23:21:14.7016504Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-11-06T23:21:15.1091213Z    Compiling cmake v0.1.38
2019-11-06T23:21:16.7843204Z error: aborting due to 8 previous errors
2019-11-06T23:21:16.7844440Z 
---
2019-11-06T23:21:17.3132251Z   local time: Wed Nov  6 23:21:17 UTC 2019
2019-11-06T23:21:17.4618091Z   network time: Wed, 06 Nov 2019 23:21:17 GMT
2019-11-06T23:21:17.4618173Z == end clock drift check ==
2019-11-06T23:21:19.0671634Z 
2019-11-06T23:21:19.0788063Z ##[error]Bash exited with code '1'.
2019-11-06T23:21:19.0814802Z ##[section]Starting: Checkout
2019-11-06T23:21:19.0816559Z ==============================================================================
2019-11-06T23:21:19.0816614Z Task         : Get sources
2019-11-06T23:21:19.0816683Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
