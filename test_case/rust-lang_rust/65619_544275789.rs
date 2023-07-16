plain
2019-10-20T17:53:36.0289568Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T17:53:36.0496936Z ##[command]git config gc.auto 0
2019-10-20T17:53:36.0588041Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T17:53:36.0643647Z ##[command]git config --get-all http.proxy
2019-10-20T17:53:36.8137913Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65619/merge:refs/remotes/pull/65619/merge
---
2019-10-20T17:59:10.1448409Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-10-20T17:59:16.4423615Z error[E0053]: method `partial_cmp` has an incompatible type for trait
2019-10-20T17:59:16.4424000Z     --> src/libcore/cmp.rs:1229:9
2019-10-20T17:59:16.4424281Z      |
2019-10-20T17:59:16.4424896Z 782  |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
2019-10-20T17:59:16.4425307Z      |     ------------------------------------------------------- type in trait
2019-10-20T17:59:16.4425526Z ...
2019-10-20T17:59:16.4425871Z 1229 |         fn partial_cmp(&self, other: &&B) -> Option<Ordering> {
2019-10-20T17:59:16.4426234Z      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-20T17:59:16.4426477Z      |
2019-10-20T17:59:16.4426814Z      = note: expected type `fn(&&A, &&mut B) -> option::Option<cmp::Ordering>`
2019-10-20T17:59:16.4427447Z                 found type `fn(&&A, &&B) -> option::Option<cmp::Ordering>`
2019-10-20T17:59:16.4427784Z help: consider change the type to match the mutability in trait
2019-10-20T17:59:16.4428007Z      |
2019-10-20T17:59:16.4428314Z 1229 |         fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>; {
2019-10-20T17:59:16.4428690Z 
2019-10-20T17:59:16.4428690Z 
2019-10-20T17:59:16.4544767Z error[E0053]: method `lt` has an incompatible type for trait
2019-10-20T17:59:16.4551015Z     --> src/libcore/cmp.rs:1233:9
2019-10-20T17:59:16.4551455Z      |
2019-10-20T17:59:16.4551830Z 798  | /     fn lt(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4552185Z 799  | |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4552504Z 800  | |             Some(Less) => true,
2019-10-20T17:59:16.4553120Z 802  | |         }
2019-10-20T17:59:16.4553443Z 803  | |     }
2019-10-20T17:59:16.4553749Z      | |_____- type in trait
2019-10-20T17:59:16.4554256Z ...
2019-10-20T17:59:16.4554256Z ...
2019-10-20T17:59:16.4554585Z 1233 |           fn lt(&self, other: &&B) -> bool { PartialOrd::lt(*self, *other) }
2019-10-20T17:59:16.4554949Z      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-20T17:59:16.4555175Z      |
2019-10-20T17:59:16.4555469Z      = note: expected type `fn(&&A, &&mut B) -> bool`
2019-10-20T17:59:16.4555743Z                 found type `fn(&&A, &&B) -> bool`
2019-10-20T17:59:16.4556012Z help: consider change the type to match the mutability in trait
2019-10-20T17:59:16.4556227Z      |
2019-10-20T17:59:16.4556702Z 1233 |         fn lt(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4557029Z 1234 |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4557673Z 1235 |             Some(Less) => true,
2019-10-20T17:59:16.4558250Z 1237 |         }
2019-10-20T17:59:16.4558250Z 1237 |         }
2019-10-20T17:59:16.4558532Z 1238 |     } { PartialOrd::lt(*self, *other) }
2019-10-20T17:59:16.4558806Z 
2019-10-20T17:59:16.4559070Z error[E0053]: method `le` has an incompatible type for trait
2019-10-20T17:59:16.4559337Z     --> src/libcore/cmp.rs:1235:9
2019-10-20T17:59:16.4559572Z      |
2019-10-20T17:59:16.4559572Z      |
2019-10-20T17:59:16.4559886Z 820  | /     fn le(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4560222Z 821  | |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4560546Z 822  | |             Some(Less) | Some(Equal) => true,
2019-10-20T17:59:16.4561157Z 824  | |         }
2019-10-20T17:59:16.4561439Z 825  | |     }
2019-10-20T17:59:16.4561745Z      | |_____- type in trait
2019-10-20T17:59:16.4561979Z ...
2019-10-20T17:59:16.4561979Z ...
2019-10-20T17:59:16.4562298Z 1235 |           fn le(&self, other: &&B) -> bool { PartialOrd::le(*self, *other) }
2019-10-20T17:59:16.4562659Z      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-20T17:59:16.4562884Z      |
2019-10-20T17:59:16.4563175Z      = note: expected type `fn(&&A, &&mut B) -> bool`
2019-10-20T17:59:16.4563446Z                 found type `fn(&&A, &&B) -> bool`
2019-10-20T17:59:16.4563717Z help: consider change the type to match the mutability in trait
2019-10-20T17:59:16.4564087Z      |
2019-10-20T17:59:16.4564400Z 1235 |         fn le(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4564687Z 1236 |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4564975Z 1237 |             Some(Less) | Some(Equal) => true,
2019-10-20T17:59:16.4565512Z 1239 |         }
2019-10-20T17:59:16.4565512Z 1239 |         }
2019-10-20T17:59:16.4565793Z 1240 |     } { PartialOrd::le(*self, *other) }
2019-10-20T17:59:16.4566069Z 
2019-10-20T17:59:16.4566329Z error[E0053]: method `ge` has an incompatible type for trait
2019-10-20T17:59:16.4566668Z     --> src/libcore/cmp.rs:1237:9
2019-10-20T17:59:16.4566937Z      |
2019-10-20T17:59:16.4566937Z      |
2019-10-20T17:59:16.4567443Z 863  | /     fn ge(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4567870Z 864  | |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4568198Z 865  | |             Some(Greater) | Some(Equal) => true,
2019-10-20T17:59:16.4568806Z 867  | |         }
2019-10-20T17:59:16.4569098Z 868  | |     }
2019-10-20T17:59:16.4569399Z      | |_____- type in trait
2019-10-20T17:59:16.4569627Z ...
2019-10-20T17:59:16.4569627Z ...
2019-10-20T17:59:16.4569947Z 1237 |           fn ge(&self, other: &&B) -> bool { PartialOrd::ge(*self, *other) }
2019-10-20T17:59:16.4570290Z      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-20T17:59:16.4570531Z      |
2019-10-20T17:59:16.4570823Z      = note: expected type `fn(&&A, &&mut B) -> bool`
2019-10-20T17:59:16.4571077Z                 found type `fn(&&A, &&B) -> bool`
2019-10-20T17:59:16.4571386Z help: consider change the type to match the mutability in trait
2019-10-20T17:59:16.4571601Z      |
2019-10-20T17:59:16.4571880Z 1237 |         fn ge(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4572343Z 1238 |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4572854Z 1239 |             Some(Greater) | Some(Equal) => true,
2019-10-20T17:59:16.4573397Z 1241 |         }
2019-10-20T17:59:16.4573397Z 1241 |         }
2019-10-20T17:59:16.4573678Z 1242 |     } { PartialOrd::ge(*self, *other) }
2019-10-20T17:59:16.4574071Z 
2019-10-20T17:59:16.4574368Z error[E0053]: method `gt` has an incompatible type for trait
2019-10-20T17:59:16.4574621Z     --> src/libcore/cmp.rs:1239:9
2019-10-20T17:59:16.4574856Z      |
2019-10-20T17:59:16.4574856Z      |
2019-10-20T17:59:16.4575168Z 841  | /     fn gt(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4575486Z 842  | |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4576058Z 843  | |             Some(Greater) => true,
2019-10-20T17:59:16.4617542Z 845  | |         }
2019-10-20T17:59:16.4617997Z 846  | |     }
2019-10-20T17:59:16.4618290Z      | |_____- type in trait
2019-10-20T17:59:16.4618525Z ...
2019-10-20T17:59:16.4618525Z ...
2019-10-20T17:59:16.4618845Z 1239 |           fn gt(&self, other: &&B) -> bool { PartialOrd::gt(*self, *other) }
2019-10-20T17:59:16.4619189Z      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ types differ in mutability
2019-10-20T17:59:16.4619435Z      |
2019-10-20T17:59:16.4619730Z      = note: expected type `fn(&&A, &&mut B) -> bool`
2019-10-20T17:59:16.4620003Z                 found type `fn(&&A, &&B) -> bool`
2019-10-20T17:59:16.4620296Z help: consider change the type to match the mutability in trait
2019-10-20T17:59:16.4620513Z      |
2019-10-20T17:59:16.4620793Z 1239 |         fn gt(&self, other: &Rhs) -> bool {
2019-10-20T17:59:16.4621100Z 1240 |         match self.partial_cmp(other) {
2019-10-20T17:59:16.4621375Z 1241 |             Some(Greater) => true,
2019-10-20T17:59:16.4621917Z 1243 |         }
2019-10-20T17:59:16.4621917Z 1243 |         }
2019-10-20T17:59:16.4622197Z 1244 |     } { PartialOrd::gt(*self, *other) }
2019-10-20T17:59:16.4622487Z 
2019-10-20T17:59:18.6959310Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-20T17:59:20.3814813Z    Compiling libc v0.2.64
2019-10-20T17:59:21.3383838Z    Compiling autocfg v0.1.6
---
2019-10-20T17:59:22.3946489Z   local time: Sun Oct 20 17:59:22 UTC 2019
2019-10-20T17:59:22.4919163Z   network time: Sun, 20 Oct 2019 17:59:22 GMT
2019-10-20T17:59:22.4922250Z == end clock drift check ==
2019-10-20T17:59:36.2523088Z 
2019-10-20T17:59:36.2627388Z ##[error]Bash exited with code '1'.
2019-10-20T17:59:36.2674051Z ##[section]Starting: Checkout
2019-10-20T17:59:36.2675819Z ==============================================================================
2019-10-20T17:59:36.2675892Z Task         : Get sources
2019-10-20T17:59:36.2675936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
