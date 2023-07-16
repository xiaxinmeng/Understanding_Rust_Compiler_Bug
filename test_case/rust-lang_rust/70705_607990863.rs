plain
2020-04-02T17:15:20.8216652Z ========================== Starting Command Output ===========================
2020-04-02T17:15:20.8219670Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ffcf3acf-20e5-40e9-82a2-4e49eee71587.sh
2020-04-02T17:15:20.8220013Z 
2020-04-02T17:15:20.8223170Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T17:15:20.8238959Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T17:15:20.8241785Z Task         : Get sources
2020-04-02T17:15:20.8242050Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T17:15:20.8242294Z Version      : 1.0.0
2020-04-02T17:15:20.8242454Z Author       : Microsoft
---
2020-04-02T17:15:21.6757591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T17:15:21.6764272Z ##[command]git config gc.auto 0
2020-04-02T17:15:21.6769272Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T17:15:21.6773787Z ##[command]git config --get-all http.proxy
2020-04-02T17:15:21.6781208Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-02T17:17:25.8810627Z Looks like docker image is the same as before, not uploading
2020-04-02T17:17:33.5406614Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T17:17:33.5641100Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T17:17:33.5669824Z == clock drift check ==
2020-04-02T17:17:33.5679502Z   local time: Thu Apr  2 17:17:33 UTC 2020
2020-04-02T17:17:33.7309770Z   network time: Thu, 02 Apr 2020 17:17:33 GMT
2020-04-02T17:17:33.7328272Z Starting sccache server...
2020-04-02T17:17:33.8057543Z configure: processing command line
2020-04-02T17:17:33.8058379Z configure: 
2020-04-02T17:17:33.8059532Z configure: rust.dist-src        := False
---
2020-04-02T17:21:49.3138256Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T17:21:50.5004917Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T17:21:51.7769442Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T17:21:52.5152299Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T17:22:00.2182120Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T17:22:02.0787813Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T17:22:05.7848436Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T17:22:09.0707320Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T17:22:17.3293761Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T17:37:51.3206944Z    Compiling libc v0.2.66
2020-04-02T17:37:51.8649569Z    Compiling autocfg v0.1.7
2020-04-02T17:37:52.9508230Z    Compiling std v0.0.0 (/checkout/src/libstd)
2020-04-02T17:37:53.3100121Z    Compiling compiler_builtins v0.1.25
2020-04-02T17:37:53.7534907Z error[E0599]: no method named `partial_cmp` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7535796Z    --> src/libcore/ops/generator.rs:9:34
2020-04-02T17:37:53.7536222Z     |
2020-04-02T17:37:53.7536824Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7538086Z     |                                    |
2020-04-02T17:37:53.7538086Z     |                                    |
2020-04-02T17:37:53.7538975Z     |                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7540313Z     | 
2020-04-02T17:37:53.7540747Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:53.7541148Z     |
2020-04-02T17:37:53.7541148Z     |
2020-04-02T17:37:53.7541739Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:53.7542435Z 907 | |     /* compiler built-in */
2020-04-02T17:37:53.7544229Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:53.7544967Z     |
2020-04-02T17:37:53.7545805Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7545805Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7546774Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7547763Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7548721Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7549573Z note: `iter::traits::iterator::Iterator` defines an item `partial_cmp`, perhaps you need to implement it
2020-04-02T17:37:53.7550780Z     |
2020-04-02T17:37:53.7551292Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7551874Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7557909Z 
2020-04-02T17:37:53.7557909Z 
2020-04-02T17:37:53.7558818Z error[E0599]: no method named `lt` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7559563Z    --> src/libcore/ops/generator.rs:9:34
2020-04-02T17:37:53.7559990Z     |
2020-04-02T17:37:53.7560566Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7561843Z     |                                    |
2020-04-02T17:37:53.7561843Z     |                                    |
2020-04-02T17:37:53.7562714Z     |                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7564038Z     | 
2020-04-02T17:37:53.7564469Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:53.7564885Z     |
2020-04-02T17:37:53.7564885Z     |
2020-04-02T17:37:53.7565452Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:53.7566150Z 907 | |     /* compiler built-in */
2020-04-02T17:37:53.7567432Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:53.7567926Z     |
2020-04-02T17:37:53.7568530Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7568530Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7569562Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7570562Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7571646Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7572535Z note: `iter::traits::iterator::Iterator` defines an item `lt`, perhaps you need to implement it
2020-04-02T17:37:53.7573740Z     |
2020-04-02T17:37:53.7574250Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7574843Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7593335Z 
2020-04-02T17:37:53.7593335Z 
2020-04-02T17:37:53.7594625Z error[E0599]: no method named `le` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7595407Z    --> src/libcore/ops/generator.rs:9:34
2020-04-02T17:37:53.7595816Z     |
2020-04-02T17:37:53.7596413Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7597665Z     |                                    |
2020-04-02T17:37:53.7597665Z     |                                    |
2020-04-02T17:37:53.7598555Z     |                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7599893Z     | 
2020-04-02T17:37:53.7600327Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:53.7600731Z     |
2020-04-02T17:37:53.7600731Z     |
2020-04-02T17:37:53.7601320Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:53.7602018Z 907 | |     /* compiler built-in */
2020-04-02T17:37:53.7603654Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:53.7604285Z     |
2020-04-02T17:37:53.7604889Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7604889Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7605705Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7679072Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7680300Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7681153Z note: `iter::traits::iterator::Iterator` defines an item `le`, perhaps you need to implement it
2020-04-02T17:37:53.7682348Z     |
2020-04-02T17:37:53.7682862Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7683598Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7683841Z 
2020-04-02T17:37:53.7683841Z 
2020-04-02T17:37:53.7684648Z error[E0599]: no method named `gt` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7685554Z    --> src/libcore/ops/generator.rs:9:34
2020-04-02T17:37:53.7686149Z     |
2020-04-02T17:37:53.7686726Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7687983Z     |                                    |
2020-04-02T17:37:53.7687983Z     |                                    |
2020-04-02T17:37:53.7688848Z     |                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7690178Z     | 
2020-04-02T17:37:53.7690610Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:53.7691024Z     |
2020-04-02T17:37:53.7691024Z     |
2020-04-02T17:37:53.7691595Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:53.7692289Z 907 | |     /* compiler built-in */
2020-04-02T17:37:53.7693574Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:53.7694055Z     |
2020-04-02T17:37:53.7694675Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7694675Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7695472Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7696347Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7697166Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7697889Z note: `iter::traits::iterator::Iterator` defines an item `gt`, perhaps you need to implement it
2020-04-02T17:37:53.7698922Z     |
2020-04-02T17:37:53.7699543Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7700135Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7700360Z 
2020-04-02T17:37:53.7700360Z 
2020-04-02T17:37:53.7701229Z error[E0599]: no method named `ge` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7702072Z    --> src/libcore/ops/generator.rs:9:34
2020-04-02T17:37:53.7702531Z     |
2020-04-02T17:37:53.7703200Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7704703Z     |                                    |
2020-04-02T17:37:53.7704703Z     |                                    |
2020-04-02T17:37:53.7705761Z     |                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7707289Z     | 
2020-04-02T17:37:53.7707784Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:53.7708243Z     |
2020-04-02T17:37:53.7708243Z     |
2020-04-02T17:37:53.7708908Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:53.7709709Z 907 | |     /* compiler built-in */
2020-04-02T17:37:53.7711171Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:53.7711727Z     |
2020-04-02T17:37:53.7712421Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7712421Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7713479Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7714195Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7715103Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7715709Z note: `iter::traits::iterator::Iterator` defines an item `ge`, perhaps you need to implement it
2020-04-02T17:37:53.7716853Z     |
2020-04-02T17:37:53.7717284Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7717770Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7717980Z 
2020-04-02T17:37:53.7717980Z 
2020-04-02T17:37:53.7718905Z error[E0599]: no method named `cmp` found for associated type `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:53.7719747Z    --> src/libcore/ops/generator.rs:9:50
2020-04-02T17:37:53.7720221Z     |
2020-04-02T17:37:53.7720923Z 9   |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:53.7722439Z     |                                                    |
2020-04-02T17:37:53.7722439Z     |                                                    |
2020-04-02T17:37:53.7723508Z     |                                                    method not found in `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:53.7725206Z     | 
2020-04-02T17:37:53.7725698Z    ::: src/libcore/cmp.rs:665:1
2020-04-02T17:37:53.7726172Z     |
2020-04-02T17:37:53.7726172Z     |
2020-04-02T17:37:53.7726855Z 665 | / pub macro Ord($item:item) {
2020-04-02T17:37:53.7727645Z 666 | |     /* compiler built-in */
2020-04-02T17:37:53.7728461Z 667 | | }
2020-04-02T17:37:53.7729102Z     | |_- in this expansion of `#[derive(Ord)]`
2020-04-02T17:37:53.7730194Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7730194Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:53.7730997Z             `<ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7731870Z             which is required by `&mut <ops::generator::GeneratorState<Y, R> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:53.7733908Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:53.7734847Z note: `iter::traits::iterator::Iterator` defines an item `cmp`, perhaps you need to implement it
2020-04-02T17:37:53.7736128Z     |
2020-04-02T17:37:53.7736678Z 96  | pub trait Iterator {
2020-04-02T17:37:53.7737319Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:53.7737562Z 
2020-04-02T17:37:53.7737562Z 
2020-04-02T17:37:54.2056883Z error[E0599]: no method named `partial_cmp` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2058100Z     |
2020-04-02T17:37:54.2058100Z     |
2020-04-02T17:37:54.2058687Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2059905Z     |                             |
2020-04-02T17:37:54.2059905Z     |                             |
2020-04-02T17:37:54.2060717Z     |                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2061978Z     | 
2020-04-02T17:37:54.2062412Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2063068Z     |
2020-04-02T17:37:54.2063068Z     |
2020-04-02T17:37:54.2063741Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2064536Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2066163Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2066717Z     |
2020-04-02T17:37:54.2067512Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2067512Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2068421Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2069318Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2070232Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2071086Z note: `iter::traits::iterator::Iterator` defines an item `partial_cmp`, perhaps you need to implement it
2020-04-02T17:37:54.2072294Z     |
2020-04-02T17:37:54.2072804Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2073381Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2078486Z 
2020-04-02T17:37:54.2078486Z 
2020-04-02T17:37:54.2085416Z error[E0599]: no method named `lt` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2086825Z     |
2020-04-02T17:37:54.2086825Z     |
2020-04-02T17:37:54.2087519Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2089038Z     |                             |
2020-04-02T17:37:54.2089038Z     |                             |
2020-04-02T17:37:54.2090026Z     |                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2091579Z     | 
2020-04-02T17:37:54.2092112Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2092624Z     |
2020-04-02T17:37:54.2092624Z     |
2020-04-02T17:37:54.2093335Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2094205Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2095784Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2096397Z     |
2020-04-02T17:37:54.2097142Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2097142Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2098069Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2099055Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2100028Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2101085Z note: `iter::traits::iterator::Iterator` defines an item `lt`, perhaps you need to implement it
2020-04-02T17:37:54.2102416Z     |
2020-04-02T17:37:54.2102997Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2103614Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2103859Z 
2020-04-02T17:37:54.2103859Z 
2020-04-02T17:37:54.2104661Z error[E0599]: no method named `le` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2106016Z     |
2020-04-02T17:37:54.2106016Z     |
2020-04-02T17:37:54.2106724Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2108712Z     |                             |
2020-04-02T17:37:54.2108712Z     |                             |
2020-04-02T17:37:54.2109723Z     |                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2111258Z     | 
2020-04-02T17:37:54.2111810Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2112308Z     |
2020-04-02T17:37:54.2112308Z     |
2020-04-02T17:37:54.2113020Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2113891Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2115605Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2116201Z     |
2020-04-02T17:37:54.2116947Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2116947Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2117888Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2118855Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2119833Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2120830Z note: `iter::traits::iterator::Iterator` defines an item `le`, perhaps you need to implement it
2020-04-02T17:37:54.2122009Z     |
2020-04-02T17:37:54.2122522Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2123092Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2196427Z 
2020-04-02T17:37:54.2196427Z 
2020-04-02T17:37:54.2197485Z error[E0599]: no method named `gt` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2198949Z     |
2020-04-02T17:37:54.2198949Z     |
2020-04-02T17:37:54.2199659Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2201263Z     |                             |
2020-04-02T17:37:54.2201263Z     |                             |
2020-04-02T17:37:54.2202425Z     |                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2203869Z     | 
2020-04-02T17:37:54.2204366Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2204840Z     |
2020-04-02T17:37:54.2204840Z     |
2020-04-02T17:37:54.2205503Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2206301Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2207766Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2208334Z     |
2020-04-02T17:37:54.2209026Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2209026Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2209890Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2210808Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2211710Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2212537Z note: `iter::traits::iterator::Iterator` defines an item `gt`, perhaps you need to implement it
2020-04-02T17:37:54.2213723Z     |
2020-04-02T17:37:54.2214248Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2214826Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2215059Z 
2020-04-02T17:37:54.2215059Z 
2020-04-02T17:37:54.2215801Z error[E0599]: no method named `ge` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2217061Z     |
2020-04-02T17:37:54.2217061Z     |
2020-04-02T17:37:54.2217699Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2219084Z     |                             |
2020-04-02T17:37:54.2219084Z     |                             |
2020-04-02T17:37:54.2220215Z     |                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2221823Z     | 
2020-04-02T17:37:54.2222426Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2222936Z     |
2020-04-02T17:37:54.2222936Z     |
2020-04-02T17:37:54.2223643Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2224679Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2226140Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2226699Z     |
2020-04-02T17:37:54.2227394Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2227394Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2228274Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2229174Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2230078Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2230920Z note: `iter::traits::iterator::Iterator` defines an item `ge`, perhaps you need to implement it
2020-04-02T17:37:54.2232336Z     |
2020-04-02T17:37:54.2232902Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2233520Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2233765Z 
2020-04-02T17:37:54.2233765Z 
2020-04-02T17:37:54.2234711Z error[E0599]: no method named `cmp` found for associated type `<option::Option<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2236070Z     |
2020-04-02T17:37:54.2236070Z     |
2020-04-02T17:37:54.2236775Z 153 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2238359Z     |                                             |
2020-04-02T17:37:54.2238359Z     |                                             |
2020-04-02T17:37:54.2239423Z     |                                             method not found in `<option::Option<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2241084Z     | 
2020-04-02T17:37:54.2241614Z    ::: src/libcore/cmp.rs:665:1
2020-04-02T17:37:54.2242108Z     |
2020-04-02T17:37:54.2242108Z     |
2020-04-02T17:37:54.2242803Z 665 | / pub macro Ord($item:item) {
2020-04-02T17:37:54.2243645Z 666 | |     /* compiler built-in */
2020-04-02T17:37:54.2244624Z 667 | | }
2020-04-02T17:37:54.2245278Z     | |_- in this expansion of `#[derive(Ord)]`
2020-04-02T17:37:54.2246422Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2246422Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2247176Z             `<option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2247957Z             which is required by `&mut <option::Option<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2248757Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2249484Z note: `iter::traits::iterator::Iterator` defines an item `cmp`, perhaps you need to implement it
2020-04-02T17:37:54.2250513Z     |
2020-04-02T17:37:54.2250961Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2251462Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2251672Z 
2020-04-02T17:37:54.2251672Z 
2020-04-02T17:37:54.2586528Z error[E0599]: no method named `partial_cmp` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2590330Z     |
2020-04-02T17:37:54.2590330Z     |
2020-04-02T17:37:54.2591201Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2593032Z     |                             |
2020-04-02T17:37:54.2593032Z     |                             |
2020-04-02T17:37:54.2594173Z     |                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2596120Z     | 
2020-04-02T17:37:54.2596785Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2597424Z     |
2020-04-02T17:37:54.2597424Z     |
2020-04-02T17:37:54.2598248Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2599239Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2601144Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2602139Z     |
2020-04-02T17:37:54.2602995Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2602995Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2604025Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2605067Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2606397Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2607964Z note: `iter::traits::iterator::Iterator` defines an item `partial_cmp`, perhaps you need to implement it
2020-04-02T17:37:54.2609197Z     |
2020-04-02T17:37:54.2609711Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2610286Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2610529Z 
2020-04-02T17:37:54.2610529Z 
2020-04-02T17:37:54.2616355Z error[E0599]: no method named `lt` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2617671Z     |
2020-04-02T17:37:54.2617671Z     |
2020-04-02T17:37:54.2618322Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2619919Z     |                             |
2020-04-02T17:37:54.2619919Z     |                             |
2020-04-02T17:37:54.2620911Z     |                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2622479Z     | 
2020-04-02T17:37:54.2623015Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2623531Z     |
2020-04-02T17:37:54.2623531Z     |
2020-04-02T17:37:54.2624244Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2625161Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2626798Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2627369Z     |
2020-04-02T17:37:54.2628062Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2628062Z     = note: the method `lt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2628938Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2629864Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2630781Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2631611Z note: `iter::traits::iterator::Iterator` defines an item `lt`, perhaps you need to implement it
2020-04-02T17:37:54.2632795Z     |
2020-04-02T17:37:54.2633323Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2633898Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2654962Z 
2020-04-02T17:37:54.2654962Z 
2020-04-02T17:37:54.2656099Z error[E0599]: no method named `le` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2657627Z     |
2020-04-02T17:37:54.2657627Z     |
2020-04-02T17:37:54.2658349Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2659859Z     |                             |
2020-04-02T17:37:54.2659859Z     |                             |
2020-04-02T17:37:54.2660862Z     |                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2662432Z     | 
2020-04-02T17:37:54.2662976Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2663583Z     |
2020-04-02T17:37:54.2663583Z     |
2020-04-02T17:37:54.2664253Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2665051Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2666518Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2667077Z     |
2020-04-02T17:37:54.2667785Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2667785Z     = note: the method `le` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2668662Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2669575Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2670499Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2671329Z note: `iter::traits::iterator::Iterator` defines an item `le`, perhaps you need to implement it
2020-04-02T17:37:54.2716469Z     |
2020-04-02T17:37:54.2716989Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2717565Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2717810Z 
2020-04-02T17:37:54.2717810Z 
2020-04-02T17:37:54.2718576Z error[E0599]: no method named `gt` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2719853Z     |
2020-04-02T17:37:54.2719853Z     |
2020-04-02T17:37:54.2720499Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2722066Z     |                             |
2020-04-02T17:37:54.2722066Z     |                             |
2020-04-02T17:37:54.2723033Z     |                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2724563Z     | 
2020-04-02T17:37:54.2725058Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2725536Z     |
2020-04-02T17:37:54.2725536Z     |
2020-04-02T17:37:54.2726192Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2726985Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2728455Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2729013Z     |
2020-04-02T17:37:54.2729730Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2729730Z     = note: the method `gt` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2730604Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2731530Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2733280Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2734453Z note: `iter::traits::iterator::Iterator` defines an item `gt`, perhaps you need to implement it
2020-04-02T17:37:54.2735740Z     |
2020-04-02T17:37:54.2736297Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2736933Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2737179Z 
2020-04-02T17:37:54.2737179Z 
2020-04-02T17:37:54.2737988Z error[E0599]: no method named `ge` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2739623Z     |
2020-04-02T17:37:54.2739623Z     |
2020-04-02T17:37:54.2740318Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2741837Z     |                             |
2020-04-02T17:37:54.2741837Z     |                             |
2020-04-02T17:37:54.2743243Z     |                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2744802Z     | 
2020-04-02T17:37:54.2745351Z    ::: src/libcore/cmp.rs:906:1
2020-04-02T17:37:54.2745853Z     |
2020-04-02T17:37:54.2745853Z     |
2020-04-02T17:37:54.2746558Z 906 | / pub macro PartialOrd($item:item) {
2020-04-02T17:37:54.2747779Z 907 | |     /* compiler built-in */
2020-04-02T17:37:54.2749414Z     | |_- in this expansion of `#[derive(PartialOrd)]`
2020-04-02T17:37:54.2750048Z     |
2020-04-02T17:37:54.2750865Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2750865Z     = note: the method `ge` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2751749Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2752660Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2753575Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2754548Z note: `iter::traits::iterator::Iterator` defines an item `ge`, perhaps you need to implement it
2020-04-02T17:37:54.2755727Z     |
2020-04-02T17:37:54.2756260Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2756833Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2757234Z 
2020-04-02T17:37:54.2757234Z 
2020-04-02T17:37:54.2758878Z error[E0599]: no method named `cmp` found for associated type `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:54.2760248Z     |
2020-04-02T17:37:54.2760248Z     |
2020-04-02T17:37:54.2760958Z 243 |   #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2020-04-02T17:37:54.2762546Z     |                                             |
2020-04-02T17:37:54.2762546Z     |                                             |
2020-04-02T17:37:54.2763617Z     |                                             method not found in `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:54.2765292Z     | 
2020-04-02T17:37:54.2765831Z    ::: src/libcore/cmp.rs:665:1
2020-04-02T17:37:54.2766447Z     |
2020-04-02T17:37:54.2766447Z     |
2020-04-02T17:37:54.2767079Z 665 | / pub macro Ord($item:item) {
2020-04-02T17:37:54.2767864Z 666 | |     /* compiler built-in */
2020-04-02T17:37:54.2768581Z 667 | | }
2020-04-02T17:37:54.2769315Z     | |_- in this expansion of `#[derive(Ord)]`
2020-04-02T17:37:54.2770568Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2770568Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:54.2771573Z             `<result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2772500Z             which is required by `&mut <result::Result<T, E> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:54.2773467Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:54.2774317Z note: `iter::traits::iterator::Iterator` defines an item `cmp`, perhaps you need to implement it
2020-04-02T17:37:54.2775507Z     |
2020-04-02T17:37:54.2776022Z 96  | pub trait Iterator {
2020-04-02T17:37:54.2776607Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:54.2776837Z 
2020-04-02T17:37:54.2776837Z 
2020-04-02T17:37:54.7652667Z    Compiling backtrace-sys v0.1.35
2020-04-02T17:37:55.0424707Z error[E0599]: no method named `cmp` found for associated type `<task::poll::Poll<T> as marker::DiscriminantKind>::Discriminant` in the current scope
2020-04-02T17:37:55.0425630Z    --> src/libcore/task/poll.rs:9:45
2020-04-02T17:37:55.0426285Z     |
2020-04-02T17:37:55.0427025Z 9   |   #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
2020-04-02T17:37:55.0428740Z     |                                               |
2020-04-02T17:37:55.0428740Z     |                                               |
2020-04-02T17:37:55.0429744Z     |                                               method not found in `<task::poll::Poll<T> as marker::DiscriminantKind>::Discriminant`
2020-04-02T17:37:55.0431315Z     | 
2020-04-02T17:37:55.0431811Z    ::: src/libcore/cmp.rs:665:1
2020-04-02T17:37:55.0432271Z     |
2020-04-02T17:37:55.0432271Z     |
2020-04-02T17:37:55.0432923Z 665 | / pub macro Ord($item:item) {
2020-04-02T17:37:55.0433704Z 666 | |     /* compiler built-in */
2020-04-02T17:37:55.0434533Z 667 | | }
2020-04-02T17:37:55.0435283Z     | |_- in this expansion of `#[derive(Ord)]`
2020-04-02T17:37:55.0436543Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:55.0436543Z     = note: the method `cmp` exists but the following trait bounds were not satisfied:
2020-04-02T17:37:55.0437417Z             `<task::poll::Poll<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:55.0438396Z             which is required by `&mut <task::poll::Poll<T> as marker::DiscriminantKind>::Discriminant: iter::traits::iterator::Iterator`
2020-04-02T17:37:55.0439205Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T17:37:55.0439927Z note: `iter::traits::iterator::Iterator` defines an item `cmp`, perhaps you need to implement it
2020-04-02T17:37:55.0441145Z     |
2020-04-02T17:37:55.0441592Z 96  | pub trait Iterator {
2020-04-02T17:37:55.0442176Z     | ^^^^^^^^^^^^^^^^^^
2020-04-02T17:37:55.0442756Z 
2020-04-02T17:37:55.0442756Z 
2020-04-02T17:37:55.0460880Z error[E0599]: no method named `partial_cmp` found for associated type `<task::poll::Poll<T> as marker::DiscriminantKind>::Discriminant` in the current scope
---
2020-04-02T17:38:01.1449455Z expected success, got: exit code: 101
2020-04-02T17:38:01.1462361Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-02T17:38:01.1462786Z Build completed unsuccessfully in 0:19:07
2020-04-02T17:38:01.1516058Z == clock drift check ==
2020-04-02T17:38:01.1532319Z   local time: Thu Apr  2 17:38:01 UTC 2020
2020-04-02T17:38:01.3544206Z   network time: Thu, 02 Apr 2020 17:38:01 GMT
2020-04-02T17:38:02.9682371Z 
2020-04-02T17:38:02.9682371Z 
2020-04-02T17:38:02.9737905Z ##[error]Bash exited with code '1'.
2020-04-02T17:38:02.9748343Z ##[section]Finishing: Run build
2020-04-02T17:38:02.9796867Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T17:38:02.9801703Z Task         : Get sources
2020-04-02T17:38:02.9802037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T17:38:02.9802344Z Version      : 1.0.0
2020-04-02T17:38:02.9802578Z Author       : Microsoft
2020-04-02T17:38:02.9802578Z Author       : Microsoft
2020-04-02T17:38:02.9802926Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T17:38:02.9803321Z ==============================================================================
2020-04-02T17:38:03.2700570Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T17:38:03.2745903Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T17:38:03.2825196Z Cleaning up task key
2020-04-02T17:38:03.2826383Z Start cleaning up orphan processes.
2020-04-02T17:38:03.3001992Z Terminate orphan process: pid (3856) (python)
2020-04-02T17:38:03.3246491Z ##[section]Finishing: Finalize Job
