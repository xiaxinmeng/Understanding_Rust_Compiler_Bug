plain
2019-09-14T23:19:28.1652882Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-14T23:19:28.1838499Z ##[command]git config gc.auto 0
2019-09-14T23:19:28.1910623Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-14T23:19:28.1964477Z ##[command]git config --get-all http.proxy
2019-09-14T23:19:28.2126560Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61812/merge:refs/remotes/pull/61812/merge
---
2019-09-15T00:22:44.6430019Z running 9035 tests
2019-09-15T00:23:01.5203287Z .................................................................................................... 100/9035
2019-09-15T00:23:12.5687359Z .................................................................................................... 200/9035
2019-09-15T00:23:20.6124217Z .................................................................................................... 300/9035
2019-09-15T00:23:32.1121827Z ......................................FF.F.....F..FF................................................ 400/9035
2019-09-15T00:23:39.7708016Z ..................................................................................................i. 500/9035
2019-09-15T00:24:01.6817098Z .................................................................................................... 700/9035
2019-09-15T00:24:06.4241828Z .................................................................................................... 800/9035
2019-09-15T00:24:10.9369932Z .................................................................................i.................. 900/9035
2019-09-15T00:24:20.7950912Z .................................................................................................... 1000/9035
---
2019-09-15T00:24:55.6726209Z .................................................................................................... 1500/9035
2019-09-15T00:25:02.5687222Z .................................................................................................... 1600/9035
2019-09-15T00:25:13.3965341Z ..............................................................................i...............i..... 1700/9035
2019-09-15T00:25:21.3862133Z .................................................................................................... 1800/9035
2019-09-15T00:25:29.1083840Z .....................................................................iiiii.......................... 1900/9035
2019-09-15T00:25:51.3245784Z .................................................................................................... 2100/9035
2019-09-15T00:25:53.8943151Z .................................................................................................... 2200/9035
2019-09-15T00:25:57.0895327Z .................................................................................................... 2300/9035
2019-09-15T00:26:06.0916728Z .................................................................................................... 2400/9035
---
2019-09-15T00:29:11.8446844Z .........................................................i..............i........................... 4700/9035
2019-09-15T00:29:21.3419311Z .................................................................................................... 4800/9035
2019-09-15T00:29:30.3179214Z .................................................................................................... 4900/9035
2019-09-15T00:29:38.0951800Z .................................................................................................... 5000/9035
2019-09-15T00:29:48.2972292Z ........................................ii.ii....................................................... 5100/9035
2019-09-15T00:29:58.5389042Z .................................................................................................... 5300/9035
2019-09-15T00:30:09.6488020Z .................................................................................................... 5400/9035
2019-09-15T00:30:17.5662699Z ....i............................................................................................... 5500/9035
2019-09-15T00:30:23.0530846Z .................................................................................................... 5600/9035
2019-09-15T00:30:23.0530846Z .................................................................................................... 5600/9035
2019-09-15T00:30:35.1089191Z ...................................................................................................i 5700/9035
2019-09-15T00:30:49.0702336Z i...i..ii...........i............................................................................... 5800/9035
2019-09-15T00:31:11.6508213Z .................................................................................................... 6000/9035
2019-09-15T00:31:18.5674652Z .................................................................................................... 6100/9035
2019-09-15T00:31:18.5674652Z .................................................................................................... 6100/9035
2019-09-15T00:31:33.5994025Z .i..ii.............................................................................................. 6200/9035
2019-09-15T00:31:53.2716657Z ............................................................i....................................... 6400/9035
2019-09-15T00:31:55.6025522Z .................................................................................................... 6500/9035
2019-09-15T00:31:58.3251104Z ................................i................................................................... 6600/9035
2019-09-15T00:32:02.7838853Z .................................................................................................... 6700/9035
---
2019-09-15T00:36:16.9125067Z 
2019-09-15T00:36:16.9125753Z ---- [ui] ui/associated-types/defaults-suitability.rs stdout ----
2019-09-15T00:36:16.9125841Z diff of stderr:
2019-09-15T00:36:16.9125871Z 
2019-09-15T00:36:16.9125919Z 1 error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9126202Z 2   --> $DIR/defaults-suitability.rs:17:14
2019-09-15T00:36:16.9126248Z 3    |
2019-09-15T00:36:16.9126290Z + LL | trait Tr {
2019-09-15T00:36:16.9126514Z +    | -------- required by `Tr`
2019-09-15T00:36:16.9126564Z 4 LL |     type Ty: Clone = NotClone;
2019-09-15T00:36:16.9126616Z 5    |              ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9127350Z - note: required by `Tr`
2019-09-15T00:36:16.9127564Z -   --> $DIR/defaults-suitability.rs:16:1
2019-09-15T00:36:16.9127741Z -    |
2019-09-15T00:36:16.9127741Z -    |
2019-09-15T00:36:16.9127950Z - LL | trait Tr {
2019-09-15T00:36:16.9128175Z 12 
2019-09-15T00:36:16.9128175Z 12 
2019-09-15T00:36:16.9128626Z 13 error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9128890Z 14   --> $DIR/defaults-suitability.rs:22:27
2019-09-15T00:36:16.9128965Z 15    |
2019-09-15T00:36:16.9128965Z 15    |
2019-09-15T00:36:16.9129024Z 16 LL | trait Tr2 where Self::Ty: Clone {
2019-09-15T00:36:16.9129296Z -    |                           ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9129699Z - note: required by `Tr2`
2019-09-15T00:36:16.9129910Z -   --> $DIR/defaults-suitability.rs:22:1
2019-09-15T00:36:16.9130084Z -    |
2019-09-15T00:36:16.9130084Z -    |
2019-09-15T00:36:16.9130319Z - LL | trait Tr2 where Self::Ty: Clone {
2019-09-15T00:36:16.9130732Z +    | --------------------------^^^^^
2019-09-15T00:36:16.9130796Z +    | |                         |
2019-09-15T00:36:16.9130796Z +    | |                         |
2019-09-15T00:36:16.9130847Z +    | |                         the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9130897Z +    | required by `Tr2`
2019-09-15T00:36:16.9131164Z 25 error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9131422Z 26   --> $DIR/defaults-suitability.rs:35:15
2019-09-15T00:36:16.9131456Z 
2019-09-15T00:36:16.9131510Z 27    |
2019-09-15T00:36:16.9131510Z 27    |
2019-09-15T00:36:16.9131552Z + LL | trait Foo<T> {
2019-09-15T00:36:16.9131761Z +    | ------------ required by `Foo`
2019-09-15T00:36:16.9131823Z 28 LL |     type Bar: Clone = Vec<T>;
2019-09-15T00:36:16.9131920Z 30    |
2019-09-15T00:36:16.9131956Z 
2019-09-15T00:36:16.9131956Z 
2019-09-15T00:36:16.9132019Z 31    = help: consider adding a `where T: std::clone::Clone` bound
2019-09-15T00:36:16.9132281Z - note: required by `Foo`
2019-09-15T00:36:16.9132506Z -   --> $DIR/defaults-suitability.rs:34:1
2019-09-15T00:36:16.9132696Z -    |
2019-09-15T00:36:16.9132696Z -    |
2019-09-15T00:36:16.9132988Z - LL | trait Foo<T> {
2019-09-15T00:36:16.9133222Z 38 
2019-09-15T00:36:16.9133222Z 38 
2019-09-15T00:36:16.9133267Z 39 error[E0277]: the trait bound `(): Foo<Self>` is not satisfied
2019-09-15T00:36:16.9133477Z 40   --> $DIR/defaults-suitability.rs:41:17
2019-09-15T00:36:16.9133562Z 41    |
2019-09-15T00:36:16.9133562Z 41    |
2019-09-15T00:36:16.9133601Z + LL | trait Bar: Sized {
2019-09-15T00:36:16.9133803Z +    | ---------------- required by `Bar`
2019-09-15T00:36:16.9133870Z + LL |     // `(): Foo<Self>` might hold for some possible impls but not all.
2019-09-15T00:36:16.9133931Z 42 LL |     type Assoc: Foo<Self> = ();
2019-09-15T00:36:16.9133984Z 43    |                 ^^^^^^^^^ the trait `Foo<Self>` is not implemented for `()`
2019-09-15T00:36:16.9134506Z - note: required by `Bar`
2019-09-15T00:36:16.9135025Z -   --> $DIR/defaults-suitability.rs:39:1
2019-09-15T00:36:16.9135241Z -    |
2019-09-15T00:36:16.9135241Z -    |
2019-09-15T00:36:16.9135444Z - LL | trait Bar: Sized {
2019-09-15T00:36:16.9135689Z 50 
2019-09-15T00:36:16.9135689Z 50 
2019-09-15T00:36:16.9135737Z 51 error[E0277]: the trait bound `NotClone: IsU8<NotClone>` is not satisfied
2019-09-15T00:36:16.9135957Z 52   --> $DIR/defaults-suitability.rs:61:18
2019-09-15T00:36:16.9136044Z 53    |
2019-09-15T00:36:16.9136044Z 53    |
2019-09-15T00:36:16.9137386Z - LL |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9137656Z -    |                  ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `NotClone`
2019-09-15T00:36:16.9138229Z - note: required by `D`
2019-09-15T00:36:16.9138489Z -   --> $DIR/defaults-suitability.rs:58:1
2019-09-15T00:36:16.9138665Z -    |
2019-09-15T00:36:16.9138665Z -    |
2019-09-15T00:36:16.9138768Z 60 LL | / trait D where
2019-09-15T00:36:16.9138811Z 61 LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9138897Z 
2019-09-15T00:36:16.9138897Z 
2019-09-15T00:36:16.9138941Z 63 LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9139005Z +    | |                  ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `NotClone`
2019-09-15T00:36:16.9139051Z 64 ...  |
2019-09-15T00:36:16.9139109Z 65 LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9139178Z 
2019-09-15T00:36:16.9139385Z -    | |_^
2019-09-15T00:36:16.9139385Z -    | |_^
2019-09-15T00:36:16.9139579Z +    | |_- required by `D`
2019-09-15T00:36:16.9139622Z 68 
2019-09-15T00:36:16.9139669Z 69 error[E0277]: the trait bound `bool: IsU8<NotClone>` is not satisfied
2019-09-15T00:36:16.9139902Z 70   --> $DIR/defaults-suitability.rs:63:11
2019-09-15T00:36:16.9139982Z 71    |
2019-09-15T00:36:16.9139982Z 71    |
2019-09-15T00:36:16.9140201Z - LL |     bool: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9140461Z -    |           ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `bool`
2019-09-15T00:36:16.9140827Z - note: required by `D`
2019-09-15T00:36:16.9141054Z -   --> $DIR/defaults-suitability.rs:58:1
2019-09-15T00:36:16.9141412Z -    |
2019-09-15T00:36:16.9141412Z -    |
2019-09-15T00:36:16.9141457Z 78 LL | / trait D where
2019-09-15T00:36:16.9141570Z 79 LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9141639Z 
2019-09-15T00:36:16.9141639Z 
2019-09-15T00:36:16.9141682Z 81 LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9141740Z + LL | |
2019-09-15T00:36:16.9141782Z + LL | |     bool: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9141832Z +    | |           ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `bool`
2019-09-15T00:36:16.9141894Z 82 ...  |
2019-09-15T00:36:16.9141945Z 83 LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9142015Z 
2019-09-15T00:36:16.9142217Z -    | |_^
2019-09-15T00:36:16.9142217Z -    | |_^
2019-09-15T00:36:16.9142409Z +    | |_- required by `D`
2019-09-15T00:36:16.9142451Z 86 
2019-09-15T00:36:16.9142514Z 87 error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9142732Z 88   --> $DIR/defaults-suitability.rs:59:23
2019-09-15T00:36:16.9142811Z 89    |
2019-09-15T00:36:16.9142811Z 89    |
2019-09-15T00:36:16.9143030Z - LL |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9144920Z -    |                       ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9145292Z -    |
2019-09-15T00:36:16.9145600Z -    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<NotClone>`
2019-09-15T00:36:16.9145806Z - note: required by `D`
2019-09-15T00:36:16.9146021Z -   --> $DIR/defaults-suitability.rs:58:1
2019-09-15T00:36:16.9146213Z -    |
2019-09-15T00:36:16.9146273Z 97 LL | / trait D where
2019-09-15T00:36:16.9146316Z 98 LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9146385Z +    | |                       ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9146433Z 99 LL | |
2019-09-15T00:36:16.9146475Z 100 LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9146570Z 
2019-09-15T00:36:16.9146570Z 
2019-09-15T00:36:16.9146612Z 102 LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9146843Z -    | |_^
2019-09-15T00:36:16.9146843Z -    | |_^
2019-09-15T00:36:16.9147055Z +    | |_- required by `D`
2019-09-15T00:36:16.9147097Z +    |
2019-09-15T00:36:16.9147149Z +    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<NotClone>`
2019-09-15T00:36:16.9147212Z 105 
2019-09-15T00:36:16.9147261Z 106 error[E0277]: the trait bound `<Self as Foo2<T>>::Baz: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9147483Z 107   --> $DIR/defaults-suitability.rs:74:15
2019-09-15T00:36:16.9147753Z 108    |
2019-09-15T00:36:16.9147753Z 108    |
2019-09-15T00:36:16.9147794Z + LL | trait Foo2<T> {
2019-09-15T00:36:16.9148048Z +    | ------------- required by `Foo2`
2019-09-15T00:36:16.9148116Z 109 LL |     type Bar: Clone = Vec<Self::Baz>;
2019-09-15T00:36:16.9148171Z 110    |               ^^^^^ the trait `std::clone::Clone` is not implemented for `<Self as Foo2<T>>::Baz`
2019-09-15T00:36:16.9148271Z 
2019-09-15T00:36:16.9148271Z 
2019-09-15T00:36:16.9148319Z 112    = help: consider adding a `where <Self as Foo2<T>>::Baz: std::clone::Clone` bound
2019-09-15T00:36:16.9148380Z 113    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<<Self as Foo2<T>>::Baz>`
2019-09-15T00:36:16.9148620Z - note: required by `Foo2`
2019-09-15T00:36:16.9148841Z -   --> $DIR/defaults-suitability.rs:73:1
2019-09-15T00:36:16.9149018Z -    |
2019-09-15T00:36:16.9149222Z - LL | trait Foo2<T> {
2019-09-15T00:36:16.9149468Z 119 
2019-09-15T00:36:16.9149468Z 119 
2019-09-15T00:36:16.9149517Z 120 error[E0277]: the trait bound `<Self as Foo25<T>>::Baz: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9149759Z 121   --> $DIR/defaults-suitability.rs:83:15
2019-09-15T00:36:16.9149830Z 122    |
2019-09-15T00:36:16.9149830Z 122    |
2019-09-15T00:36:16.9149886Z + LL | trait Foo25<T: Clone> {
2019-09-15T00:36:16.9150110Z +    | --------------------- required by `Foo25`
2019-09-15T00:36:16.9150261Z 123 LL |     type Bar: Clone = Vec<Self::Baz>;
2019-09-15T00:36:16.9150331Z 124    |               ^^^^^ the trait `std::clone::Clone` is not implemented for `<Self as Foo25<T>>::Baz`
2019-09-15T00:36:16.9150406Z 
2019-09-15T00:36:16.9150406Z 
2019-09-15T00:36:16.9150455Z 126    = help: consider adding a `where <Self as Foo25<T>>::Baz: std::clone::Clone` bound
2019-09-15T00:36:16.9150532Z 127    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<<Self as Foo25<T>>::Baz>`
2019-09-15T00:36:16.9150784Z - note: required by `Foo25`
2019-09-15T00:36:16.9151001Z -   --> $DIR/defaults-suitability.rs:82:1
2019-09-15T00:36:16.9151299Z -    |
2019-09-15T00:36:16.9151490Z - LL | trait Foo25<T: Clone> {
2019-09-15T00:36:16.9151738Z 133 
2019-09-15T00:36:16.9151784Z 134 error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9152009Z 135   --> $DIR/defaults-suitability.rs:92:16
2019-09-15T00:36:16.9152041Z 
2019-09-15T00:36:16.9152041Z 
2019-09-15T00:36:16.9152095Z 136    |
2019-09-15T00:36:16.9152287Z - LL |     Self::Baz: Clone,
2019-09-15T00:36:16.9152715Z -    |
2019-09-15T00:36:16.9152715Z -    |
2019-09-15T00:36:16.9152940Z -    = help: consider adding a `where T: std::clone::Clone` bound
2019-09-15T00:36:16.9153130Z - note: required by `Foo3`
2019-09-15T00:36:16.9153334Z -   --> $DIR/defaults-suitability.rs:90:1
2019-09-15T00:36:16.9153650Z -    |
2019-09-15T00:36:16.9153704Z 144 LL | / trait Foo3<T> where
2019-09-15T00:36:16.9153747Z 145 LL | |     Self::Bar: Clone,
2019-09-15T00:36:16.9153808Z 146 LL | |     Self::Baz: Clone,
2019-09-15T00:36:16.9153884Z +    | |                ^^^^^ the trait `std::clone::Clone` is not implemented for `T`
2019-09-15T00:36:16.9153930Z 147 LL | |
2019-09-15T00:36:16.9153985Z 148 ...  |
2019-09-15T00:36:16.9153985Z 148 ...  |
2019-09-15T00:36:16.9154033Z 149 LL | |     type Baz = T;
2019-09-15T00:36:16.9154115Z 150 LL | | }
2019-09-15T00:36:16.9154304Z -    | |_^
2019-09-15T00:36:16.9154304Z -    | |_^
2019-09-15T00:36:16.9154500Z +    | |_- required by `Foo3`
2019-09-15T00:36:16.9154544Z +    |
2019-09-15T00:36:16.9154879Z +    = help: consider adding a `where T: std::clone::Clone` bound
2019-09-15T00:36:16.9154929Z 152 
2019-09-15T00:36:16.9154977Z 153 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
2019-09-15T00:36:16.9155276Z 154   --> $DIR/defaults-suitability.rs:29:5
2019-09-15T00:36:16.9155334Z 
2019-09-15T00:36:16.9155513Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9155513Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9155897Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/defaults-suitability.stderr
2019-09-15T00:36:16.9156146Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9156412Z To only update this specific test, also pass `--test-args associated-types/defaults-suitability.rs`
2019-09-15T00:36:16.9156522Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9156565Z status: exit code: 1
2019-09-15T00:36:16.9156565Z status: exit code: 1
2019-09-15T00:36:16.9157342Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-suitability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/auxiliary" "-A" "unused"
2019-09-15T00:36:16.9157675Z ------------------------------------------
2019-09-15T00:36:16.9157711Z 
2019-09-15T00:36:16.9158060Z ------------------------------------------
2019-09-15T00:36:16.9158105Z stderr:
2019-09-15T00:36:16.9158105Z stderr:
2019-09-15T00:36:16.9158332Z ------------------------------------------
2019-09-15T00:36:16.9158384Z error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9158635Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:17:14
2019-09-15T00:36:16.9158701Z    |
2019-09-15T00:36:16.9158742Z LL | trait Tr {
2019-09-15T00:36:16.9159052Z    | -------- required by `Tr`
2019-09-15T00:36:16.9159113Z LL |     type Ty: Clone = NotClone;
2019-09-15T00:36:16.9159172Z    |              ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9159204Z 
2019-09-15T00:36:16.9159250Z error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9159511Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:22:27
2019-09-15T00:36:16.9159556Z    |
2019-09-15T00:36:16.9159597Z LL | trait Tr2 where Self::Ty: Clone {
2019-09-15T00:36:16.9159865Z    | |                         |
2019-09-15T00:36:16.9159865Z    | |                         |
2019-09-15T00:36:16.9159914Z    | |                         the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9159975Z    | required by `Tr2`
2019-09-15T00:36:16.9160046Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9160286Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:35:15
2019-09-15T00:36:16.9160345Z    |
2019-09-15T00:36:16.9160345Z    |
2019-09-15T00:36:16.9160384Z LL | trait Foo<T> {
2019-09-15T00:36:16.9160587Z    | ------------ required by `Foo`
2019-09-15T00:36:16.9160649Z LL |     type Bar: Clone = Vec<T>;
2019-09-15T00:36:16.9160740Z    |
2019-09-15T00:36:16.9160740Z    |
2019-09-15T00:36:16.9160799Z    = help: consider adding a `where T: std::clone::Clone` bound
2019-09-15T00:36:16.9160897Z 
2019-09-15T00:36:16.9160897Z 
2019-09-15T00:36:16.9160959Z error[E0277]: the trait bound `(): Foo<Self>` is not satisfied
2019-09-15T00:36:16.9161206Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:41:17
2019-09-15T00:36:16.9161251Z    |
2019-09-15T00:36:16.9161304Z LL | trait Bar: Sized {
2019-09-15T00:36:16.9161508Z    | ---------------- required by `Bar`
2019-09-15T00:36:16.9161611Z LL |     // `(): Foo<Self>` might hold for some possible impls but not all.
2019-09-15T00:36:16.9161768Z LL |     type Assoc: Foo<Self> = ();
2019-09-15T00:36:16.9161829Z    |                 ^^^^^^^^^ the trait `Foo<Self>` is not implemented for `()`
2019-09-15T00:36:16.9161863Z 
2019-09-15T00:36:16.9161910Z error[E0277]: the trait bound `NotClone: IsU8<NotClone>` is not satisfied
2019-09-15T00:36:16.9162224Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:61:18
2019-09-15T00:36:16.9162284Z    |
2019-09-15T00:36:16.9162325Z LL | / trait D where
2019-09-15T00:36:16.9162386Z LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9162437Z LL | |     //~^ ERROR the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9162486Z LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9162553Z    | |                  ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `NotClone`
2019-09-15T00:36:16.9162601Z ...  |
2019-09-15T00:36:16.9162644Z LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9162687Z LL | | }
2019-09-15T00:36:16.9162926Z    | |_- required by `D`
2019-09-15T00:36:16.9162958Z 
2019-09-15T00:36:16.9163005Z error[E0277]: the trait bound `bool: IsU8<NotClone>` is not satisfied
2019-09-15T00:36:16.9163278Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:63:11
2019-09-15T00:36:16.9163326Z    |
2019-09-15T00:36:16.9163367Z LL | / trait D where
2019-09-15T00:36:16.9163411Z LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9163571Z LL | |     //~^ ERROR the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9163621Z LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9163671Z LL | |     //~^ ERROR the trait bound `NotClone: IsU8<NotClone>` is not satisfied
2019-09-15T00:36:16.9163736Z LL | |     bool: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9163788Z    | |           ^^^^^^^^^^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `bool`
2019-09-15T00:36:16.9163833Z ...  |
2019-09-15T00:36:16.9163892Z LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9163935Z LL | | }
2019-09-15T00:36:16.9164177Z    | |_- required by `D`
2019-09-15T00:36:16.9164209Z 
2019-09-15T00:36:16.9164386Z error[E0277]: the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9164888Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:59:23
2019-09-15T00:36:16.9164946Z    |
2019-09-15T00:36:16.9165006Z LL | / trait D where
2019-09-15T00:36:16.9165060Z LL | |     Vec<Self::Assoc>: Clone,
2019-09-15T00:36:16.9165111Z    | |                       ^^^^^ the trait `std::clone::Clone` is not implemented for `NotClone`
2019-09-15T00:36:16.9165182Z LL | |     //~^ ERROR the trait bound `NotClone: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9165232Z LL | |     Self::Assoc: IsU8<Self::Assoc>,
2019-09-15T00:36:16.9165273Z ...  |
2019-09-15T00:36:16.9165330Z LL | |     type Assoc = NotClone;
2019-09-15T00:36:16.9165372Z LL | | }
2019-09-15T00:36:16.9165608Z    | |_- required by `D`
2019-09-15T00:36:16.9165652Z    |
2019-09-15T00:36:16.9165727Z    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<NotClone>`
2019-09-15T00:36:16.9165764Z 
2019-09-15T00:36:16.9165812Z error[E0277]: the trait bound `<Self as Foo2<T>>::Baz: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9166081Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:74:15
2019-09-15T00:36:16.9166130Z    |
2019-09-15T00:36:16.9166177Z LL | trait Foo2<T> {
2019-09-15T00:36:16.9166402Z    | ------------- required by `Foo2`
2019-09-15T00:36:16.9166450Z LL |     type Bar: Clone = Vec<Self::Baz>;
2019-09-15T00:36:16.9166504Z    |               ^^^^^ the trait `std::clone::Clone` is not implemented for `<Self as Foo2<T>>::Baz`
2019-09-15T00:36:16.9166569Z    |
2019-09-15T00:36:16.9166617Z    = help: consider adding a `where <Self as Foo2<T>>::Baz: std::clone::Clone` bound
2019-09-15T00:36:16.9166676Z    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<<Self as Foo2<T>>::Baz>`
2019-09-15T00:36:16.9166837Z 
2019-09-15T00:36:16.9166915Z error[E0277]: the trait bound `<Self as Foo25<T>>::Baz: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9167202Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:83:15
2019-09-15T00:36:16.9167250Z    |
2019-09-15T00:36:16.9167309Z LL | trait Foo25<T: Clone> {
2019-09-15T00:36:16.9167528Z    | --------------------- required by `Foo25`
2019-09-15T00:36:16.9167589Z LL |     type Bar: Clone = Vec<Self::Baz>;
2019-09-15T00:36:16.9167659Z    |               ^^^^^ the trait `std::clone::Clone` is not implemented for `<Self as Foo25<T>>::Baz`
2019-09-15T00:36:16.9167706Z    |
2019-09-15T00:36:16.9167754Z    = help: consider adding a `where <Self as Foo25<T>>::Baz: std::clone::Clone` bound
2019-09-15T00:36:16.9167830Z    = note: required because of the requirements on the impl of `std::clone::Clone` for `std::vec::Vec<<Self as Foo25<T>>::Baz>`
2019-09-15T00:36:16.9167920Z error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9168172Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:92:16
2019-09-15T00:36:16.9168234Z    |
2019-09-15T00:36:16.9168234Z    |
2019-09-15T00:36:16.9168274Z LL | / trait Foo3<T> where
2019-09-15T00:36:16.9168317Z LL | |     Self::Bar: Clone,
2019-09-15T00:36:16.9168376Z LL | |     Self::Baz: Clone,
2019-09-15T00:36:16.9168426Z    | |                ^^^^^ the trait `std::clone::Clone` is not implemented for `T`
2019-09-15T00:36:16.9168574Z LL | |     //~^ ERROR the trait bound `T: std::clone::Clone` is not satisfied
2019-09-15T00:36:16.9168636Z ...  |
2019-09-15T00:36:16.9168677Z LL | |     type Baz = T;
2019-09-15T00:36:16.9168717Z LL | | }
2019-09-15T00:36:16.9168945Z    | |_- required by `Foo3`
2019-09-15T00:36:16.9169006Z    |
2019-09-15T00:36:16.9169050Z    = help: consider adding a `where T: std::clone::Clone` bound
2019-09-15T00:36:16.9169082Z 
2019-09-15T00:36:16.9169147Z error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
2019-09-15T00:36:16.9169405Z   --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:29:5
2019-09-15T00:36:16.9169453Z    |
2019-09-15T00:36:16.9169502Z LL |     type Ty = Vec<[u8]>;
2019-09-15T00:36:16.9169744Z    |     ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-15T00:36:16.9188537Z    |
2019-09-15T00:36:16.9188602Z    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
2019-09-15T00:36:16.9190575Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-15T00:36:16.9190686Z    = note: required by `std::vec::Vec`
2019-09-15T00:36:16.9190766Z error: aborting due to 11 previous errors
2019-09-15T00:36:16.9190808Z 
2019-09-15T00:36:16.9191166Z For more information about this error, try `rustc --explain E0277`.
2019-09-15T00:36:16.9191201Z 
---
2019-09-15T00:36:16.9191808Z 
2019-09-15T00:36:16.9191863Z 1 error[E0277]: the trait bound `Self: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9192082Z 2   --> $DIR/defaults-unsound-62211-1.rs:23:18
2019-09-15T00:36:16.9193915Z 3    |
2019-09-15T00:36:16.9193991Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9194483Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9194534Z + ...
2019-09-15T00:36:16.9194585Z 4 LL |     type Output: Copy
2019-09-15T00:36:16.9195082Z 6    |
2019-09-15T00:36:16.9195109Z 
2019-09-15T00:36:16.9195109Z 
2019-09-15T00:36:16.9195163Z 7    = help: consider adding a `where Self: std::marker::Copy` bound
2019-09-15T00:36:16.9195442Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9195860Z -   --> $DIR/defaults-unsound-62211-1.rs:20:1
2019-09-15T00:36:16.9196116Z -    |
2019-09-15T00:36:16.9196316Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9196577Z 13 
2019-09-15T00:36:16.9196577Z 13 
2019-09-15T00:36:16.9196804Z 14 error[E0277]: cannot add-assign `&'static str` to `Self`
2019-09-15T00:36:16.9197023Z 15   --> $DIR/defaults-unsound-62211-1.rs:27:7
2019-09-15T00:36:16.9197118Z 16    |
2019-09-15T00:36:16.9197118Z 16    |
2019-09-15T00:36:16.9197160Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9197389Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9197444Z + ...
2019-09-15T00:36:16.9197645Z 17 LL |     + AddAssign<&'static str>
2019-09-15T00:36:16.9197890Z 18    |       ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Self += &'static str`
2019-09-15T00:36:16.9197974Z 
2019-09-15T00:36:16.9198345Z 20    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9198345Z 20    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9198594Z 21    = help: consider adding a `where Self: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9199021Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9199235Z -   --> $DIR/defaults-unsound-62211-1.rs:20:1
2019-09-15T00:36:16.9199527Z -    |
2019-09-15T00:36:16.9199733Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9200126Z 27 
2019-09-15T00:36:16.9200172Z 28 error[E0277]: the trait bound `Self: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9200727Z 29   --> $DIR/defaults-unsound-62211-1.rs:25:7
2019-09-15T00:36:16.9200766Z 
2019-09-15T00:36:16.9200766Z 
2019-09-15T00:36:16.9200807Z 30    |
2019-09-15T00:36:16.9200866Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9201129Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9201175Z + ...
2019-09-15T00:36:16.9201215Z 31 LL |     + Deref<Target = str>
2019-09-15T00:36:16.9201341Z 33    |
2019-09-15T00:36:16.9201366Z 
2019-09-15T00:36:16.9201366Z 
2019-09-15T00:36:16.9202186Z 34    = help: consider adding a `where Self: std::ops::Deref` bound
2019-09-15T00:36:16.9202577Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9202801Z -   --> $DIR/defaults-unsound-62211-1.rs:20:1
2019-09-15T00:36:16.9202996Z -    |
2019-09-15T00:36:16.9203263Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9203504Z 40 
2019-09-15T00:36:16.9203747Z 41 error[E0277]: `Self` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9203967Z 42   --> $DIR/defaults-unsound-62211-1.rs:30:7
2019-09-15T00:36:16.9204000Z 
2019-09-15T00:36:16.9204000Z 
2019-09-15T00:36:16.9204039Z 43    |
2019-09-15T00:36:16.9204091Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9204320Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9204367Z + ...
2019-09-15T00:36:16.9204425Z 44 LL |     + Display = Self;
2019-09-15T00:36:16.9204474Z 45    |       ^^^^^^^ `Self` cannot be formatted with the default formatter
2019-09-15T00:36:16.9204543Z 
2019-09-15T00:36:16.9204894Z 47    = help: the trait `std::fmt::Display` is not implemented for `Self`
2019-09-15T00:36:16.9205226Z 48    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9205226Z 48    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9205295Z 49    = help: consider adding a `where Self: std::fmt::Display` bound
2019-09-15T00:36:16.9220201Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9220707Z -   --> $DIR/defaults-unsound-62211-1.rs:20:1
2019-09-15T00:36:16.9220927Z -    |
2019-09-15T00:36:16.9221196Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9222417Z 55 
2019-09-15T00:36:16.9222715Z 56 error[E0277]: `T` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9222974Z 57   --> $DIR/defaults-unsound-62211-1.rs:43:9
2019-09-15T00:36:16.9223232Z 
2019-09-15T00:36:16.9223232Z 
2019-09-15T00:36:16.9223275Z 
2019-09-15T00:36:16.9223350Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9223773Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/defaults-unsound-62211-1.stderr
2019-09-15T00:36:16.9224050Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9224392Z To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-1.rs`
2019-09-15T00:36:16.9224483Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9224532Z status: exit code: 1
2019-09-15T00:36:16.9224532Z status: exit code: 1
2019-09-15T00:36:16.9226050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/auxiliary" "-A" "unused"
2019-09-15T00:36:16.9226655Z ------------------------------------------
2019-09-15T00:36:16.9226697Z 
2019-09-15T00:36:16.9226942Z ------------------------------------------
2019-09-15T00:36:16.9227007Z stderr:
2019-09-15T00:36:16.9227007Z stderr:
2019-09-15T00:36:16.9227241Z ------------------------------------------
2019-09-15T00:36:16.9227301Z error[E0277]: the trait bound `Self: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9227600Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:23:18
2019-09-15T00:36:16.9227658Z    |
2019-09-15T00:36:16.9227707Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9227985Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9228085Z LL |     type Output: Copy
2019-09-15T00:36:16.9228159Z    |                  ^^^^ the trait `std::marker::Copy` is not implemented for `Self`
2019-09-15T00:36:16.9228211Z    |
2019-09-15T00:36:16.9228211Z    |
2019-09-15T00:36:16.9228263Z    = help: consider adding a `where Self: std::marker::Copy` bound
2019-09-15T00:36:16.9228299Z 
2019-09-15T00:36:16.9228579Z error[E0277]: cannot add-assign `&'static str` to `Self`
2019-09-15T00:36:16.9228858Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:27:7
2019-09-15T00:36:16.9228911Z    |
2019-09-15T00:36:16.9228972Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9229227Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9229278Z ...
2019-09-15T00:36:16.9229514Z LL |     + AddAssign<&'static str>
2019-09-15T00:36:16.9229791Z    |       ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Self += &'static str`
2019-09-15T00:36:16.9231070Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9231070Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9231392Z    = help: consider adding a `where Self: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9231486Z error[E0277]: the trait bound `Self: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9231781Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:25:7
2019-09-15T00:36:16.9231843Z    |
2019-09-15T00:36:16.9231843Z    |
2019-09-15T00:36:16.9231889Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9232158Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9232211Z ...
2019-09-15T00:36:16.9232257Z LL |     + Deref<Target = str>
2019-09-15T00:36:16.9232376Z    |
2019-09-15T00:36:16.9232376Z    |
2019-09-15T00:36:16.9232429Z    = help: consider adding a `where Self: std::ops::Deref` bound
2019-09-15T00:36:16.9232885Z error[E0277]: `Self` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9233225Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:30:7
2019-09-15T00:36:16.9233279Z    |
2019-09-15T00:36:16.9233279Z    |
2019-09-15T00:36:16.9233337Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9233594Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9233645Z ...
2019-09-15T00:36:16.9233701Z LL |     + Display = Self;
2019-09-15T00:36:16.9233771Z    |       ^^^^^^^ `Self` cannot be formatted with the default formatter
2019-09-15T00:36:16.9233870Z    = help: the trait `std::fmt::Display` is not implemented for `Self`
2019-09-15T00:36:16.9234183Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9234183Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9234251Z    = help: consider adding a `where Self: std::fmt::Display` bound
2019-09-15T00:36:16.9234557Z error[E0277]: `T` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9235361Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:43:9
2019-09-15T00:36:16.9235419Z    |
2019-09-15T00:36:16.9235419Z    |
2019-09-15T00:36:16.9235481Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9235536Z    |         ^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
2019-09-15T00:36:16.9235634Z    = help: the trait `std::fmt::Display` is not implemented for `T`
2019-09-15T00:36:16.9236132Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9236132Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9236196Z    = help: consider adding a `where T: std::fmt::Display` bound
2019-09-15T00:36:16.9236296Z error[E0277]: the trait bound `T: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9236582Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:43:9
2019-09-15T00:36:16.9236634Z    |
2019-09-15T00:36:16.9236634Z    |
2019-09-15T00:36:16.9236694Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9236812Z    |
2019-09-15T00:36:16.9236812Z    |
2019-09-15T00:36:16.9236877Z    = help: consider adding a `where T: std::ops::Deref` bound
2019-09-15T00:36:16.9236912Z 
2019-09-15T00:36:16.9237166Z error[E0277]: cannot add-assign `&'static str` to `T`
2019-09-15T00:36:16.9237444Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:43:9
2019-09-15T00:36:16.9237522Z    |
2019-09-15T00:36:16.9237568Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9237835Z    |         ^^^^^^^^^^^^^ no implementation for `T += &'static str`
2019-09-15T00:36:16.9238178Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `T`
2019-09-15T00:36:16.9238178Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `T`
2019-09-15T00:36:16.9238456Z    = help: consider adding a `where T: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9238562Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9238849Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:43:9
2019-09-15T00:36:16.9238903Z    |
2019-09-15T00:36:16.9238903Z    |
2019-09-15T00:36:16.9238964Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9239070Z    |
2019-09-15T00:36:16.9239070Z    |
2019-09-15T00:36:16.9239132Z    = help: consider adding a `where T: std::marker::Copy` bound
2019-09-15T00:36:16.9239222Z error: aborting due to 8 previous errors
2019-09-15T00:36:16.9239253Z 
2019-09-15T00:36:16.9239537Z For more information about this error, try `rustc --explain E0277`.
2019-09-15T00:36:16.9239575Z 
---
2019-09-15T00:36:16.9240239Z 
2019-09-15T00:36:16.9240398Z 1 error[E0277]: the trait bound `Self: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9240693Z 2   --> $DIR/defaults-unsound-62211-2.rs:23:18
2019-09-15T00:36:16.9240745Z 3    |
2019-09-15T00:36:16.9240793Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9241064Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9241116Z + ...
2019-09-15T00:36:16.9241176Z 4 LL |     type Output: Copy
2019-09-15T00:36:16.9241298Z 6    |
2019-09-15T00:36:16.9241329Z 
2019-09-15T00:36:16.9241329Z 
2019-09-15T00:36:16.9241381Z 7    = help: consider adding a `where Self: std::marker::Copy` bound
2019-09-15T00:36:16.9241628Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9241873Z -   --> $DIR/defaults-unsound-62211-2.rs:20:1
2019-09-15T00:36:16.9242073Z -    |
2019-09-15T00:36:16.9242317Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9242598Z 13 
2019-09-15T00:36:16.9242598Z 13 
2019-09-15T00:36:16.9242870Z 14 error[E0277]: cannot add-assign `&'static str` to `Self`
2019-09-15T00:36:16.9243121Z 15   --> $DIR/defaults-unsound-62211-2.rs:27:7
2019-09-15T00:36:16.9243201Z 16    |
2019-09-15T00:36:16.9243201Z 16    |
2019-09-15T00:36:16.9243263Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9243521Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9243666Z + ...
2019-09-15T00:36:16.9243939Z 17 LL |     + AddAssign<&'static str>
2019-09-15T00:36:16.9244221Z 18    |       ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Self += &'static str`
2019-09-15T00:36:16.9244599Z 
2019-09-15T00:36:16.9244957Z 20    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9244957Z 20    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9245245Z 21    = help: consider adding a `where Self: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9245478Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9245753Z -   --> $DIR/defaults-unsound-62211-2.rs:20:1
2019-09-15T00:36:16.9245954Z -    |
2019-09-15T00:36:16.9246177Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9246461Z 27 
2019-09-15T00:36:16.9246513Z 28 error[E0277]: the trait bound `Self: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9246775Z 29   --> $DIR/defaults-unsound-62211-2.rs:25:7
2019-09-15T00:36:16.9246821Z 
2019-09-15T00:36:16.9246821Z 
2019-09-15T00:36:16.9246865Z 30    |
2019-09-15T00:36:16.9246911Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9247182Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9247234Z + ...
2019-09-15T00:36:16.9247300Z 31 LL |     + Deref<Target = str>
2019-09-15T00:36:16.9247408Z 33    |
2019-09-15T00:36:16.9247437Z 
2019-09-15T00:36:16.9247437Z 
2019-09-15T00:36:16.9247507Z 34    = help: consider adding a `where Self: std::ops::Deref` bound
2019-09-15T00:36:16.9247751Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9247999Z -   --> $DIR/defaults-unsound-62211-2.rs:20:1
2019-09-15T00:36:16.9248214Z -    |
2019-09-15T00:36:16.9248443Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9248715Z 40 
2019-09-15T00:36:16.9248986Z 41 error[E0277]: `Self` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9249245Z 42   --> $DIR/defaults-unsound-62211-2.rs:30:7
2019-09-15T00:36:16.9249281Z 
2019-09-15T00:36:16.9249281Z 
2019-09-15T00:36:16.9249340Z 43    |
2019-09-15T00:36:16.9249386Z + LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9249644Z +    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9249711Z + ...
2019-09-15T00:36:16.9249758Z 44 LL |     + Display = Self;
2019-09-15T00:36:16.9249812Z 45    |       ^^^^^^^ `Self` cannot be formatted with the default formatter
2019-09-15T00:36:16.9249906Z 
2019-09-15T00:36:16.9250091Z 47    = help: the trait `std::fmt::Display` is not implemented for `Self`
2019-09-15T00:36:16.9250442Z 48    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9250442Z 48    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9250526Z 49    = help: consider adding a `where Self: std::fmt::Display` bound
2019-09-15T00:36:16.9250763Z - note: required by `UncheckedCopy`
2019-09-15T00:36:16.9251022Z -   --> $DIR/defaults-unsound-62211-2.rs:20:1
2019-09-15T00:36:16.9251236Z -    |
2019-09-15T00:36:16.9251462Z - LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9251733Z 55 
2019-09-15T00:36:16.9252001Z 56 error[E0277]: `T` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9252247Z 57   --> $DIR/defaults-unsound-62211-2.rs:43:9
2019-09-15T00:36:16.9252283Z 
2019-09-15T00:36:16.9252283Z 
2019-09-15T00:36:16.9252312Z 
2019-09-15T00:36:16.9252378Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9252749Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/defaults-unsound-62211-2.stderr
2019-09-15T00:36:16.9253031Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9253370Z To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-2.rs`
2019-09-15T00:36:16.9253460Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9253629Z status: exit code: 1
2019-09-15T00:36:16.9253629Z status: exit code: 1
2019-09-15T00:36:16.9254528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/auxiliary" "-A" "unused"
2019-09-15T00:36:16.9255317Z ------------------------------------------
2019-09-15T00:36:16.9255356Z 
2019-09-15T00:36:16.9255614Z ------------------------------------------
2019-09-15T00:36:16.9255665Z stderr:
2019-09-15T00:36:16.9255665Z stderr:
2019-09-15T00:36:16.9255897Z ------------------------------------------
2019-09-15T00:36:16.9255987Z error[E0277]: the trait bound `Self: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9257518Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:23:18
2019-09-15T00:36:16.9257587Z    |
2019-09-15T00:36:16.9257662Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9257928Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9258026Z LL |     type Output: Copy
2019-09-15T00:36:16.9258123Z    |                  ^^^^ the trait `std::marker::Copy` is not implemented for `Self`
2019-09-15T00:36:16.9258189Z    |
2019-09-15T00:36:16.9258189Z    |
2019-09-15T00:36:16.9258240Z    = help: consider adding a `where Self: std::marker::Copy` bound
2019-09-15T00:36:16.9258291Z 
2019-09-15T00:36:16.9258960Z error[E0277]: cannot add-assign `&'static str` to `Self`
2019-09-15T00:36:16.9259534Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:27:7
2019-09-15T00:36:16.9259612Z    |
2019-09-15T00:36:16.9259670Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9259972Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9260024Z ...
2019-09-15T00:36:16.9260273Z LL |     + AddAssign<&'static str>
2019-09-15T00:36:16.9260549Z    |       ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Self += &'static str`
2019-09-15T00:36:16.9260902Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9260902Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `Self`
2019-09-15T00:36:16.9261187Z    = help: consider adding a `where Self: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9261477Z error[E0277]: the trait bound `Self: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9261807Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:25:7
2019-09-15T00:36:16.9261861Z    |
2019-09-15T00:36:16.9261861Z    |
2019-09-15T00:36:16.9261907Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9262181Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9262245Z ...
2019-09-15T00:36:16.9262291Z LL |     + Deref<Target = str>
2019-09-15T00:36:16.9262417Z    |
2019-09-15T00:36:16.9262417Z    |
2019-09-15T00:36:16.9262469Z    = help: consider adding a `where Self: std::ops::Deref` bound
2019-09-15T00:36:16.9262783Z error[E0277]: `Self` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9263066Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:30:7
2019-09-15T00:36:16.9263118Z    |
2019-09-15T00:36:16.9263118Z    |
2019-09-15T00:36:16.9263189Z LL | trait UncheckedCopy: Sized {
2019-09-15T00:36:16.9263445Z    | -------------------------- required by `UncheckedCopy`
2019-09-15T00:36:16.9263495Z ...
2019-09-15T00:36:16.9263556Z LL |     + Display = Self;
2019-09-15T00:36:16.9263609Z    |       ^^^^^^^ `Self` cannot be formatted with the default formatter
2019-09-15T00:36:16.9263724Z    = help: the trait `std::fmt::Display` is not implemented for `Self`
2019-09-15T00:36:16.9264164Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9264164Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9264231Z    = help: consider adding a `where Self: std::fmt::Display` bound
2019-09-15T00:36:16.9264775Z error[E0277]: `T` doesn't implement `std::fmt::Display`
2019-09-15T00:36:16.9265148Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:43:9
2019-09-15T00:36:16.9265203Z    |
2019-09-15T00:36:16.9265203Z    |
2019-09-15T00:36:16.9265276Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9265345Z    |         ^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
2019-09-15T00:36:16.9265460Z    = help: the trait `std::fmt::Display` is not implemented for `T`
2019-09-15T00:36:16.9265768Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9265768Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2019-09-15T00:36:16.9265831Z    = help: consider adding a `where T: std::fmt::Display` bound
2019-09-15T00:36:16.9265943Z error[E0277]: the trait bound `T: std::ops::Deref` is not satisfied
2019-09-15T00:36:16.9266228Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:43:9
2019-09-15T00:36:16.9266295Z    |
2019-09-15T00:36:16.9266295Z    |
2019-09-15T00:36:16.9266342Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9266447Z    |
2019-09-15T00:36:16.9266447Z    |
2019-09-15T00:36:16.9266513Z    = help: consider adding a `where T: std::ops::Deref` bound
2019-09-15T00:36:16.9266548Z 
2019-09-15T00:36:16.9266811Z error[E0277]: cannot add-assign `&'static str` to `T`
2019-09-15T00:36:16.9267112Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:43:9
2019-09-15T00:36:16.9267164Z    |
2019-09-15T00:36:16.9267209Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9267478Z    |         ^^^^^^^^^^^^^ no implementation for `T += &'static str`
2019-09-15T00:36:16.9267831Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `T`
2019-09-15T00:36:16.9267831Z    = help: the trait `std::ops::AddAssign<&'static str>` is not implemented for `T`
2019-09-15T00:36:16.9268113Z    = help: consider adding a `where T: std::ops::AddAssign<&'static str>` bound
2019-09-15T00:36:16.9268218Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2019-09-15T00:36:16.9268502Z   --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:43:9
2019-09-15T00:36:16.9268569Z    |
2019-09-15T00:36:16.9268569Z    |
2019-09-15T00:36:16.9268617Z LL | impl<T> UncheckedCopy for T {}
2019-09-15T00:36:16.9268893Z    |
2019-09-15T00:36:16.9268893Z    |
2019-09-15T00:36:16.9268944Z    = help: consider adding a `where T: std::marker::Copy` bound
2019-09-15T00:36:16.9269028Z error: aborting due to 8 previous errors
2019-09-15T00:36:16.9269075Z 
2019-09-15T00:36:16.9269380Z For more information about this error, try `rustc --explain E0277`.
2019-09-15T00:36:16.9269429Z 
2019-09-15T00:36:16.9269429Z 
2019-09-15T00:36:16.9269669Z ------------------------------------------
2019-09-15T00:36:16.9269720Z 
2019-09-15T00:36:16.9269750Z 
2019-09-15T00:36:16.9270007Z ---- [ui] ui/associated-types/issue-43924.rs stdout ----
2019-09-15T00:36:16.9270060Z diff of stderr:
2019-09-15T00:36:16.9270091Z 
2019-09-15T00:36:16.9270422Z 1 error[E0277]: the trait bound `(dyn std::string::ToString + 'static): std::default::Default` is not satisfied
2019-09-15T00:36:16.9270664Z 2   --> $DIR/issue-43924.rs:7:15
2019-09-15T00:36:16.9270715Z 3    |
2019-09-15T00:36:16.9270788Z + LL | trait Foo<T: Default + ToString> {
2019-09-15T00:36:16.9271048Z +    | -------------------------------- required by `Foo`
2019-09-15T00:36:16.9271108Z 4 LL |     type Out: Default + ToString + ?Sized = dyn ToString;
2019-09-15T00:36:16.9271454Z 5    |               ^^^^^^^ the trait `std::default::Default` is not implemented for `(dyn std::string::ToString + 'static)`
2019-09-15T00:36:16.9272028Z - note: required by `Foo`
2019-09-15T00:36:16.9272278Z -   --> $DIR/issue-43924.rs:6:1
2019-09-15T00:36:16.9272477Z -    |
2019-09-15T00:36:16.9272477Z -    |
2019-09-15T00:36:16.9272714Z - LL | trait Foo<T: Default + ToString> {
2019-09-15T00:36:16.9273016Z 12 
2019-09-15T00:36:16.9273016Z 12 
2019-09-15T00:36:16.9273330Z 13 error[E0277]: the trait bound `(dyn std::string::ToString + 'static): std::default::Default` is not satisfied
2019-09-15T00:36:16.9273588Z 14   --> $DIR/issue-43924.rs:10:6
2019-09-15T00:36:16.9273654Z 
2019-09-15T00:36:16.9273714Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9274069Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924/issue-43924.stderr
2019-09-15T00:36:16.9274069Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924/issue-43924.stderr
2019-09-15T00:36:16.9274345Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9274883Z To only update this specific test, also pass `--test-args associated-types/issue-43924.rs`
2019-09-15T00:36:16.9275014Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9275064Z status: exit code: 1
2019-09-15T00:36:16.9275064Z status: exit code: 1
2019-09-15T00:36:16.9275950Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-43924.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43924/auxiliary" "-A" "unused"
2019-09-15T00:36:16.9276312Z ------------------------------------------
2019-09-15T00:36:16.9276367Z 
2019-09-15T00:36:16.9276609Z ------------------------------------------
2019-09-15T00:36:16.9276668Z stderr:
2019-09-15T00:36:16.9276668Z stderr:
2019-09-15T00:36:16.9276923Z ------------------------------------------
2019-09-15T00:36:16.9277238Z error[E0277]: the trait bound `(dyn std::string::ToString + 'static): std::default::Default` is not satisfied
2019-09-15T00:36:16.9278049Z   --> /checkout/src/test/ui/associated-types/issue-43924.rs:7:15
2019-09-15T00:36:16.9278149Z    |
2019-09-15T00:36:16.9278197Z LL | trait Foo<T: Default + ToString> {
2019-09-15T00:36:16.9278455Z    | -------------------------------- required by `Foo`
2019-09-15T00:36:16.9278533Z LL |     type Out: Default + ToString + ?Sized = dyn ToString;  //~ error: not satisfied
2019-09-15T00:36:16.9279022Z    |               ^^^^^^^ the trait `std::default::Default` is not implemented for `(dyn std::string::ToString + 'static)`
2019-09-15T00:36:16.9279082Z 
2019-09-15T00:36:16.9279519Z error[E0277]: the trait bound `(dyn std::string::ToString + 'static): std::default::Default` is not satisfied
2019-09-15T00:36:16.9279801Z   --> /checkout/src/test/ui/associated-types/issue-43924.rs:10:6
2019-09-15T00:36:16.9279863Z    |
2019-09-15T00:36:16.9279912Z LL | impl Foo<u32> for () {}  //~ error: not satisfied
2019-09-15T00:36:16.9280249Z    |      ^^^^^^^^ the trait `std::default::Default` is not implemented for `(dyn std::string::ToString + 'static)`
2019-09-15T00:36:16.9280293Z 
2019-09-15T00:36:16.9280601Z error[E0277]: the trait bound `(dyn std::string::ToString + 'static): std::default::Default` is not satisfied
2019-09-15T00:36:16.9280894Z   --> /checkout/src/test/ui/associated-types/issue-43924.rs:11:6
2019-09-15T00:36:16.9280946Z    |
2019-09-15T00:36:16.9281003Z LL | impl Foo<u64> for () {}  //~ error: not satisfied
2019-09-15T00:36:16.9281336Z    |      ^^^^^^^^ the trait `std::default::Default` is not implemented for `(dyn std::string::ToString + 'static)`
2019-09-15T00:36:16.9281426Z error: aborting due to 3 previous errors
2019-09-15T00:36:16.9281458Z 
2019-09-15T00:36:16.9281739Z For more information about this error, try `rustc --explain E0277`.
2019-09-15T00:36:16.9281900Z 
---
2019-09-15T00:36:16.9282592Z 
2019-09-15T00:36:16.9282662Z 5 error[E0277]: the size for values of type `Self` cannot be known at compilation time
2019-09-15T00:36:16.9282904Z 6   --> $DIR/issue-63593.rs:9:5
2019-09-15T00:36:16.9282953Z 7    |
2019-09-15T00:36:16.9283015Z + LL | trait MyTrait {
2019-09-15T00:36:16.9283263Z +    | ------------- required by `MyTrait`
2019-09-15T00:36:16.9283317Z 8 LL |     type This = Self;
2019-09-15T00:36:16.9283586Z 9    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-15T00:36:16.9283686Z 
2019-09-15T00:36:16.9283740Z 11    = help: the trait `std::marker::Sized` is not implemented for `Self`
2019-09-15T00:36:16.9283740Z 11    = help: the trait `std::marker::Sized` is not implemented for `Self`
2019-09-15T00:36:16.9284129Z 12    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-15T00:36:16.9284203Z 13    = help: consider adding a `where Self: std::marker::Sized` bound
2019-09-15T00:36:16.9284439Z - note: required by `MyTrait`
2019-09-15T00:36:16.9285021Z -   --> $DIR/issue-63593.rs:8:1
2019-09-15T00:36:16.9285231Z -    |
2019-09-15T00:36:16.9285445Z - LL | trait MyTrait {
2019-09-15T00:36:16.9285725Z 19 
2019-09-15T00:36:16.9285774Z 20 error: aborting due to 2 previous errors
2019-09-15T00:36:16.9285834Z 21 
2019-09-15T00:36:16.9285881Z 
2019-09-15T00:36:16.9285881Z 
2019-09-15T00:36:16.9285909Z 
2019-09-15T00:36:16.9285959Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9286299Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-63593/issue-63593.stderr
2019-09-15T00:36:16.9286590Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9286957Z To only update this specific test, also pass `--test-args associated-types/issue-63593.rs`
2019-09-15T00:36:16.9287060Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9287108Z status: exit code: 1
2019-09-15T00:36:16.9287108Z status: exit code: 1
2019-09-15T00:36:16.9288145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-63593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-63593" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-63593/auxiliary" "-A" "unused"
2019-09-15T00:36:16.9288593Z ------------------------------------------
2019-09-15T00:36:16.9288659Z 
2019-09-15T00:36:16.9288909Z ------------------------------------------
2019-09-15T00:36:16.9288958Z stderr:
2019-09-15T00:36:16.9288958Z stderr:
2019-09-15T00:36:16.9289190Z ------------------------------------------
2019-09-15T00:36:16.9289263Z error[E0601]: `main` function not found in crate `issue_63593`
2019-09-15T00:36:16.9289621Z    = note: consider adding a `main` function to `/checkout/src/test/ui/associated-types/issue-63593.rs`
2019-09-15T00:36:16.9289679Z 
2019-09-15T00:36:16.9289736Z error[E0277]: the size for values of type `Self` cannot be known at compilation time
2019-09-15T00:36:16.9290020Z   --> /checkout/src/test/ui/associated-types/issue-63593.rs:9:5
2019-09-15T00:36:16.9290020Z   --> /checkout/src/test/ui/associated-types/issue-63593.rs:9:5
2019-09-15T00:36:16.9290089Z    |
2019-09-15T00:36:16.9290133Z LL | trait MyTrait {
2019-09-15T00:36:16.9290372Z    | ------------- required by `MyTrait`
2019-09-15T00:36:16.9290447Z LL |     type This = Self;  //~ error: size for values of type `Self` cannot be known
2019-09-15T00:36:16.9290719Z    |     ^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-09-15T00:36:16.9290945Z    = help: the trait `std::marker::Sized` is not implemented for `Self`
2019-09-15T00:36:16.9290945Z    = help: the trait `std::marker::Sized` is not implemented for `Self`
2019-09-15T00:36:16.9291327Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-09-15T00:36:16.9291399Z    = help: consider adding a `where Self: std::marker::Sized` bound
2019-09-15T00:36:16.9291504Z error: aborting due to 2 previous errors
2019-09-15T00:36:16.9291535Z 
2019-09-15T00:36:16.9291594Z Some errors have detailed explanations: E0277, E0601.
2019-09-15T00:36:16.9292113Z For more information about an error, try `rustc --explain E0277`.
---
2019-09-15T00:36:16.9292947Z normalized stderr:
2019-09-15T00:36:16.9293010Z warning: unused variable: `r`
2019-09-15T00:36:16.9293265Z   --> $DIR/issue-54182-1.rs:24:9
2019-09-15T00:36:16.9293314Z    |
2019-09-15T00:36:16.9293361Z LL |     let r: () = overload!(42, true);
2019-09-15T00:36:16.9293430Z    |         ^ help: consider prefixing with an underscore: `_r`
2019-09-15T00:36:16.9293527Z    = note: `#[warn(unused_variables)]` on by default
2019-09-15T00:36:16.9293561Z 
2019-09-15T00:36:16.9293624Z warning: unused variable: `r`
2019-09-15T00:36:16.9293863Z   --> $DIR/issue-54182-1.rs:29:9
2019-09-15T00:36:16.9293863Z   --> $DIR/issue-54182-1.rs:29:9
2019-09-15T00:36:16.9293911Z    |
2019-09-15T00:36:16.9293984Z LL |     let r: () = overload!(42, true, 42.5);
2019-09-15T00:36:16.9294039Z    |         ^ help: consider prefixing with an underscore: `_r`
2019-09-15T00:36:16.9294101Z 
2019-09-15T00:36:16.9294129Z 
2019-09-15T00:36:16.9294173Z 
2019-09-15T00:36:16.9294222Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9294222Z The actual stderr differed from the expected stderr.
2019-09-15T00:36:16.9294844Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54182-1/issue-54182-1.stderr
2019-09-15T00:36:16.9295194Z To update references, rerun the tests and pass the `--bless` flag
2019-09-15T00:36:16.9295832Z To only update this specific test, also pass `--test-args associated-types/issue-54182-1.rs`
2019-09-15T00:36:16.9295944Z error: 1 errors occurred comparing output.
2019-09-15T00:36:16.9295994Z status: exit code: 0
2019-09-15T00:36:16.9295994Z status: exit code: 0
2019-09-15T00:36:16.9297284Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-54182-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54182-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54182-1/auxiliary"
2019-09-15T00:36:16.9297739Z ------------------------------------------
2019-09-15T00:36:16.9297779Z 
2019-09-15T00:36:16.9298146Z ------------------------------------------
2019-09-15T00:36:16.9298197Z stderr:
2019-09-15T00:36:16.9298197Z stderr:
2019-09-15T00:36:16.9298578Z ------------------------------------------
2019-09-15T00:36:16.9298647Z warning: unused variable: `r`
2019-09-15T00:36:16.9298946Z   --> /checkout/src/test/ui/associated-types/issue-54182-1.rs:24:9
2019-09-15T00:36:16.9299001Z    |
2019-09-15T00:36:16.9299079Z LL |     let r: () = overload!(42, true);
2019-09-15T00:36:16.9299133Z    |         ^ help: consider prefixing with an underscore: `_r`
2019-09-15T00:36:16.9299247Z    = note: `#[warn(unused_variables)]` on by default
2019-09-15T00:36:16.9299281Z 
2019-09-15T00:36:16.9299328Z warning: unused variable: `r`
2019-09-15T00:36:16.9299606Z   --> /checkout/src/test/ui/associated-types/issue-54182-1.rs:29:9
2019-09-15T00:36:16.9299606Z   --> /checkout/src/test/ui/associated-types/issue-54182-1.rs:29:9
2019-09-15T00:36:16.9299792Z    |
2019-09-15T00:36:16.9299841Z LL |     let r: () = overload!(42, true, 42.5);
2019-09-15T00:36:16.9299896Z    |         ^ help: consider prefixing with an underscore: `_r`
2019-09-15T00:36:16.9299976Z 
2019-09-15T00:36:16.9300248Z ------------------------------------------
2019-09-15T00:36:16.9300284Z 
2019-09-15T00:36:16.9300313Z 
---
2019-09-15T00:36:16.9302288Z test result: FAILED. 8991 passed; 6 failed; 38 ignored; 0 measured; 0 filtered out
2019-09-15T00:36:16.9302331Z 
2019-09-15T00:36:16.9302381Z 
2019-09-15T00:36:16.9302409Z 
2019-09-15T00:36:16.9304036Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-15T00:36:16.9304328Z 
2019-09-15T00:36:16.9304361Z 
2019-09-15T00:36:16.9304521Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-15T00:36:16.9304906Z Build completed unsuccessfully in 1:09:20
2019-09-15T00:36:16.9304906Z Build completed unsuccessfully in 1:09:20
2019-09-15T00:36:16.9305309Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-15T00:36:16.9305376Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-15T00:36:16.9305538Z == clock drift check ==
2019-09-15T00:36:16.9305588Z   local time: Sun Sep 15 00:36:16 UTC 2019
2019-09-15T00:36:17.0732791Z   network time: Sun, 15 Sep 2019 00:36:17 GMT
2019-09-15T00:36:17.0742533Z == end clock drift check ==
2019-09-15T00:36:17.8376361Z ##[error]Bash exited with code '1'.
2019-09-15T00:36:17.8413042Z ##[section]Starting: Checkout
2019-09-15T00:36:17.8415441Z ==============================================================================
2019-09-15T00:36:17.8415516Z Task         : Get sources
2019-09-15T00:36:17.8415563Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
