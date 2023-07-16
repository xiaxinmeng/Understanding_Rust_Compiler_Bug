plain
2019-07-16T03:56:00.3099360Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-16T03:56:00.3100668Z 
2019-07-16T03:56:00.3102690Z   git checkout -b <new-branch-name>
2019-07-16T03:56:00.3103758Z 
2019-07-16T03:56:00.3104673Z HEAD is now at 40d495671 Auto merge of #61946 - BaoshanPang:vxworks, r=alexcrichton
2019-07-16T03:56:00.3222714Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-16T03:56:00.3225183Z ==============================================================================
2019-07-16T03:56:00.3225269Z Task         : Bash
2019-07-16T03:56:00.3225322Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-16T03:56:00.8326250Z 408 ./src/test/run-pass/macros
2019-07-16T03:56:00.8327650Z 400 ./src/liballoc/collections
2019-07-16T03:56:00.8327871Z 384 ./src/libstd/os
2019-07-16T03:56:00.8328631Z 380 ./src/test/run-pass/binding
2019-07-16T03:56:00.8328864Z 380 ./src/libstd/sys/vxworks
2019-07-16T03:56:00.8329315Z 364 ./src/librustc_apfloat
2019-07-16T03:56:00.8329766Z 352 ./src/test/ui/nll/user-annotations
2019-07-16T03:56:00.8330223Z 352 ./src/test/run-pass/issues/auxiliary
2019-07-16T03:56:00.8330437Z 344 ./src/librustc_mir/interpret
---
2019-07-16T05:00:51.1418389Z [RUSTC-TIMING] panic_unwind test:false 0.096
2019-07-16T05:00:51.8146230Z [RUSTC-TIMING] hashbrown test:false 0.740
2019-07-16T05:00:51.9963230Z warning: dropping unsupported crate type `dylib` for target `wasm32-wasi`
2019-07-16T05:00:51.9963805Z 
2019-07-16T05:00:52.3762027Z error[E0433]: failed to resolve: could not find `wasi` in `os`
2019-07-16T05:00:52.3762657Z  --> src/libstd/sys/wasi/args.rs:6:16
2019-07-16T05:00:52.3763073Z   |
2019-07-16T05:00:52.3763377Z 6 | use crate::os::wasi::ffi::OsStringExt;
2019-07-16T05:00:52.3763698Z   |                ^^^^ could not find `wasi` in `os`
2019-07-16T05:00:52.3763763Z 
2019-07-16T05:00:52.3764015Z error[E0433]: failed to resolve: could not find `wasi` in `os`
2019-07-16T05:00:52.3764244Z  --> src/libstd/sys/wasi/fs.rs:6:16
2019-07-16T05:00:52.3764452Z   |
2019-07-16T05:00:52.3764699Z 6 | use crate::os::wasi::ffi::{OsStrExt, OsStringExt};
2019-07-16T05:00:52.3765009Z   |                ^^^^ could not find `wasi` in `os`
2019-07-16T05:00:52.3765075Z 
2019-07-16T05:00:52.3765334Z error[E0433]: failed to resolve: could not find `wasi` in `os`
2019-07-16T05:00:52.3765565Z  --> src/libstd/sys/wasi/os.rs:7:16
2019-07-16T05:00:52.3765772Z   |
2019-07-16T05:00:52.3766007Z 7 | use crate::os::wasi::prelude::*;
2019-07-16T05:00:52.3766312Z   |                ^^^^ could not find `wasi` in `os`
2019-07-16T05:00:52.3766369Z 
2019-07-16T05:00:52.3766615Z error[E0433]: failed to resolve: could not find `wasi` in `os`
2019-07-16T05:00:52.3766849Z  --> src/libstd/sys/wasi/ext/fs.rs:7:16
2019-07-16T05:00:52.3767069Z   |
2019-07-16T05:00:52.3767304Z 7 | use crate::os::wasi::ffi::OsStrExt;
2019-07-16T05:00:52.3767618Z   |                ^^^^ could not find `wasi` in `os`
2019-07-16T05:00:52.3767677Z 
2019-07-16T05:00:52.4367235Z error[E0433]: failed to resolve: use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4367549Z    --> src/libstd/sys/wasi/os.rs:129:13
2019-07-16T05:00:52.4367765Z     |
2019-07-16T05:00:52.4368026Z 129 |             OsStringExt::from_vec(input[..p].to_vec()),
2019-07-16T05:00:52.4368326Z     |             ^^^^^^^^^^^ use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4368564Z 
2019-07-16T05:00:52.4368884Z error[E0433]: failed to resolve: use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4369161Z    --> src/libstd/sys/wasi/os.rs:130:13
2019-07-16T05:00:52.4369837Z     |
2019-07-16T05:00:52.4370206Z 130 |             OsStringExt::from_vec(input[p+1..].to_vec()),
2019-07-16T05:00:52.4370615Z     |             ^^^^^^^^^^^ use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4370714Z 
2019-07-16T05:00:52.4371035Z error[E0433]: failed to resolve: use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4371359Z    --> src/libstd/sys/wasi/os.rs:143:18
2019-07-16T05:00:52.4371608Z     |
2019-07-16T05:00:52.4372003Z 143 |             Some(OsStringExt::from_vec(CStr::from_ptr(s).to_bytes().to_vec()))
2019-07-16T05:00:52.4372600Z     |                  ^^^^^^^^^^^ use of undeclared type or module `OsStringExt`
2019-07-16T05:00:52.4372869Z 
2019-07-16T05:00:52.5991974Z error: unused import: `crate::os::wasi::ffi::OsStringExt`
2019-07-16T05:00:52.5992558Z  --> src/libstd/sys/wasi/args.rs:6:5
2019-07-16T05:00:52.5992772Z   |
2019-07-16T05:00:52.5993014Z 6 | use crate::os::wasi::ffi::OsStringExt;
2019-07-16T05:00:52.5993505Z   |
2019-07-16T05:00:52.5993505Z   |
2019-07-16T05:00:52.5993802Z   = note: `-D unused-imports` implied by `-D warnings`
2019-07-16T05:00:52.5993853Z 
2019-07-16T05:00:52.5994101Z error: unused imports: `OsStrExt`, `OsStringExt`
2019-07-16T05:00:52.5994334Z  --> src/libstd/sys/wasi/fs.rs:6:28
2019-07-16T05:00:52.5994540Z   |
2019-07-16T05:00:52.5994789Z 6 | use crate::os::wasi::ffi::{OsStrExt, OsStringExt};
2019-07-16T05:00:52.5995103Z   |                            ^^^^^^^^  ^^^^^^^^^^^
2019-07-16T05:00:52.5995163Z 
2019-07-16T05:00:52.5995408Z error: unused import: `crate::os::wasi::prelude::*`
2019-07-16T05:00:52.5995787Z  --> src/libstd/sys/wasi/os.rs:7:5
2019-07-16T05:00:52.5995997Z   |
2019-07-16T05:00:52.5996234Z 7 | use crate::os::wasi::prelude::*;
2019-07-16T05:00:52.5996523Z 
2019-07-16T05:00:52.5996523Z 
2019-07-16T05:00:52.5996824Z error: unused import: `crate::os::wasi::ffi::OsStrExt`
2019-07-16T05:00:52.5997046Z  --> src/libstd/sys/wasi/ext/fs.rs:7:5
2019-07-16T05:00:52.5997244Z   |
2019-07-16T05:00:52.5997481Z 7 | use crate::os::wasi::ffi::OsStrExt;
2019-07-16T05:00:52.5997767Z 
2019-07-16T05:00:52.5997767Z 
2019-07-16T05:00:54.1527220Z error[E0599]: no function or associated item named `from_vec` found for type `ffi::os_str::OsString` in the current scope
2019-07-16T05:00:54.1528486Z   --> src/libstd/sys/wasi/args.rs:41:36
2019-07-16T05:00:54.1528904Z    |
2019-07-16T05:00:54.1529367Z 41 |             .map(|bytes| OsString::from_vec(bytes))
2019-07-16T05:00:54.1530312Z    |                                    ^^^^^^^^ function or associated item not found in `ffi::os_str::OsString`
2019-07-16T05:00:54.1531251Z   ::: src/libstd/ffi/os_str.rs:80:1
2019-07-16T05:00:54.1531686Z    |
2019-07-16T05:00:54.1532156Z 80 | pub struct OsString {
2019-07-16T05:00:54.1532156Z 80 | pub struct OsString {
2019-07-16T05:00:54.1532747Z    | ------------------- function or associated item `from_vec` not found for this
2019-07-16T05:00:54.1533325Z    |
2019-07-16T05:00:54.1534800Z    = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.1535579Z    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.1536087Z            `use crate::sys_common::os_str_bytes::OsStringExt;`
2019-07-16T05:00:54.1536302Z 
2019-07-16T05:00:54.1868681Z error[E0599]: no function or associated item named `from_bytes` found for type `ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.1874439Z    --> src/libstd/sys/wasi/fs.rs:222:27
2019-07-16T05:00:54.1874916Z     |
2019-07-16T05:00:54.1875419Z 222 |         let name = OsStr::from_bytes(&self.name);
2019-07-16T05:00:54.1875958Z     |                           ^^^^^^^^^^ function or associated item not found in `ffi::os_str::OsStr`
2019-07-16T05:00:54.1880753Z    ::: src/libstd/ffi/os_str.rs:100:1
2019-07-16T05:00:54.1881175Z     |
2019-07-16T05:00:54.1881960Z 100 | pub struct OsStr {
2019-07-16T05:00:54.1881960Z 100 | pub struct OsStr {
2019-07-16T05:00:54.1882585Z     | ---------------- function or associated item `from_bytes` not found for this
2019-07-16T05:00:54.1883022Z     |
2019-07-16T05:00:54.1883842Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.1884372Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.1884799Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.1884955Z 
2019-07-16T05:00:54.1885654Z error[E0599]: no function or associated item named `from_vec` found for type `ffi::os_str::OsString` in the current scope
2019-07-16T05:00:54.1886237Z    --> src/libstd/sys/wasi/fs.rs:227:19
2019-07-16T05:00:54.1887506Z     |
2019-07-16T05:00:54.1887987Z 227 |         OsString::from_vec(self.name.clone())
2019-07-16T05:00:54.1888354Z     |                   ^^^^^^^^ function or associated item not found in `ffi::os_str::OsString`
2019-07-16T05:00:54.1889230Z    ::: src/libstd/ffi/os_str.rs:80:1
2019-07-16T05:00:54.1889651Z     |
2019-07-16T05:00:54.1889970Z 80  | pub struct OsString {
2019-07-16T05:00:54.1889970Z 80  | pub struct OsString {
2019-07-16T05:00:54.1890379Z     | ------------------- function or associated item `from_vec` not found for this
2019-07-16T05:00:54.1890668Z     |
2019-07-16T05:00:54.1891005Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.1891410Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.1891730Z             `use crate::sys_common::os_str_bytes::OsStringExt;`
2019-07-16T05:00:54.1891807Z 
2019-07-16T05:00:54.1892285Z error[E0599]: no function or associated item named `from_bytes` found for type `ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.1892870Z    --> src/libstd/sys/wasi/fs.rs:234:20
2019-07-16T05:00:54.1893241Z     |
2019-07-16T05:00:54.1893897Z 234 |             OsStr::from_bytes(&self.name).as_ref(),
2019-07-16T05:00:54.1894447Z     |                    ^^^^^^^^^^ function or associated item not found in `ffi::os_str::OsStr`
2019-07-16T05:00:54.1894988Z    ::: src/libstd/ffi/os_str.rs:100:1
2019-07-16T05:00:54.1895221Z     |
2019-07-16T05:00:54.1895499Z 100 | pub struct OsStr {
2019-07-16T05:00:54.1895499Z 100 | pub struct OsStr {
2019-07-16T05:00:54.1895858Z     | ---------------- function or associated item `from_bytes` not found for this
2019-07-16T05:00:54.1896112Z     |
2019-07-16T05:00:54.1896408Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.1896759Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.1897178Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.1897247Z 
2019-07-16T05:00:54.1980632Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.1981071Z    --> src/libstd/sys/wasi/fs.rs:481:47
2019-07-16T05:00:54.1981331Z     |
2019-07-16T05:00:54.1981707Z 481 |         dir.create_directory(file.as_os_str().as_bytes())
2019-07-16T05:00:54.1982348Z     |
2019-07-16T05:00:54.1982348Z     |
2019-07-16T05:00:54.1982860Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.1983516Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.1983789Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.1983838Z 
2019-07-16T05:00:54.2004109Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2004612Z    --> src/libstd/sys/wasi/fs.rs:512:38
2019-07-16T05:00:54.2004848Z     |
2019-07-16T05:00:54.2005125Z 512 |     dir.unlink_file(file.as_os_str().as_bytes())
2019-07-16T05:00:54.2005634Z     |
2019-07-16T05:00:54.2005634Z     |
2019-07-16T05:00:54.2005927Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2006238Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2006507Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2006564Z 
2019-07-16T05:00:54.2017862Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2018233Z    --> src/libstd/sys/wasi/fs.rs:519:30
2019-07-16T05:00:54.2018455Z     |
2019-07-16T05:00:54.2018743Z 519 |         old_file.as_os_str().as_bytes(),
2019-07-16T05:00:54.2019261Z     |
2019-07-16T05:00:54.2019261Z     |
2019-07-16T05:00:54.2019635Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2020340Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2020664Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2020741Z 
2019-07-16T05:00:54.2021095Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2021429Z    --> src/libstd/sys/wasi/fs.rs:521:30
2019-07-16T05:00:54.2021677Z     |
2019-07-16T05:00:54.2022013Z 521 |         new_file.as_os_str().as_bytes(),
2019-07-16T05:00:54.2022622Z     |
2019-07-16T05:00:54.2022622Z     |
2019-07-16T05:00:54.2022954Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2023492Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2023945Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2023994Z 
2019-07-16T05:00:54.2028343Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2028655Z    --> src/libstd/sys/wasi/fs.rs:533:43
2019-07-16T05:00:54.2028882Z     |
2019-07-16T05:00:54.2029160Z 533 |     dir.remove_directory(file.as_os_str().as_bytes())
2019-07-16T05:00:54.2030002Z     |
2019-07-16T05:00:54.2030002Z     |
2019-07-16T05:00:54.2030350Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2030926Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2031258Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2031315Z 
2019-07-16T05:00:54.2045689Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2045990Z    --> src/libstd/sys/wasi/fs.rs:558:33
2019-07-16T05:00:54.2046222Z     |
2019-07-16T05:00:54.2046482Z 558 |     let file = file.as_os_str().as_bytes();
2019-07-16T05:00:54.2047559Z     |
2019-07-16T05:00:54.2047559Z     |
2019-07-16T05:00:54.2047853Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2048319Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2048618Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2048681Z 
2019-07-16T05:00:54.2056262Z error[E0599]: no function or associated item named `from_vec` found for type `ffi::os_str::OsString` in the current scope
2019-07-16T05:00:54.2056666Z    --> src/libstd/sys/wasi/fs.rs:565:47
2019-07-16T05:00:54.2056931Z     |
2019-07-16T05:00:54.2057308Z 565 |             return Ok(PathBuf::from(OsString::from_vec(destination)));
2019-07-16T05:00:54.2057820Z     |                                               ^^^^^^^^ function or associated item not found in `ffi::os_str::OsString`
2019-07-16T05:00:54.2059094Z    ::: src/libstd/ffi/os_str.rs:80:1
2019-07-16T05:00:54.2060235Z     |
2019-07-16T05:00:54.2060571Z 80  | pub struct OsString {
2019-07-16T05:00:54.2060571Z 80  | pub struct OsString {
2019-07-16T05:00:54.2060982Z     | ------------------- function or associated item `from_vec` not found for this
2019-07-16T05:00:54.2061276Z     |
2019-07-16T05:00:54.2061617Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2062022Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2062887Z             `use crate::sys_common::os_str_bytes::OsStringExt;`
2019-07-16T05:00:54.2063042Z 
2019-07-16T05:00:54.2072895Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2073194Z    --> src/libstd/sys/wasi/fs.rs:574:33
2019-07-16T05:00:54.2073422Z     |
2019-07-16T05:00:54.2073714Z 574 |     dst.symlink(src.as_os_str().as_bytes(), dst_file.as_os_str().as_bytes())
2019-07-16T05:00:54.2074770Z     |
2019-07-16T05:00:54.2074770Z     |
2019-07-16T05:00:54.2075060Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2075576Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2075850Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2075896Z 
2019-07-16T05:00:54.2076189Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2077004Z    --> src/libstd/sys/wasi/fs.rs:574:66
2019-07-16T05:00:54.2077239Z     |
2019-07-16T05:00:54.2077525Z 574 |     dst.symlink(src.as_os_str().as_bytes(), dst_file.as_os_str().as_bytes())
2019-07-16T05:00:54.2078088Z     |
2019-07-16T05:00:54.2078088Z     |
2019-07-16T05:00:54.2078445Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2079320Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2080174Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2080255Z 
2019-07-16T05:00:54.2087240Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2088065Z    --> src/libstd/sys/wasi/fs.rs:582:30
2019-07-16T05:00:54.2088302Z     |
2019-07-16T05:00:54.2088583Z 582 |         src_file.as_os_str().as_bytes(),
2019-07-16T05:00:54.2089095Z     |
2019-07-16T05:00:54.2089095Z     |
2019-07-16T05:00:54.2089821Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2090941Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2091317Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2091380Z 
2019-07-16T05:00:54.2091727Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2092063Z    --> src/libstd/sys/wasi/fs.rs:584:30
2019-07-16T05:00:54.2092331Z     |
2019-07-16T05:00:54.2092655Z 584 |         dst_file.as_os_str().as_bytes(),
2019-07-16T05:00:54.2093548Z     |
2019-07-16T05:00:54.2093548Z     |
2019-07-16T05:00:54.2094399Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2094892Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2095159Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2095205Z 
2019-07-16T05:00:54.2102538Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2102898Z    --> src/libstd/sys/wasi/fs.rs:604:50
2019-07-16T05:00:54.2104465Z     |
2019-07-16T05:00:54.2104774Z 604 |     fd.path_filestat_get(flags, path.as_os_str().as_bytes(), &mut ret.meta)?;
2019-07-16T05:00:54.2105300Z     |
2019-07-16T05:00:54.2105300Z     |
2019-07-16T05:00:54.2105588Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2105909Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2106750Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2111965Z 
2019-07-16T05:00:54.2113699Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2114220Z    --> src/libstd/sys/wasi/fs.rs:617:26
2019-07-16T05:00:54.2115068Z     |
2019-07-16T05:00:54.2115392Z 617 |         path.as_os_str().as_bytes(),
2019-07-16T05:00:54.2115881Z     |
2019-07-16T05:00:54.2115881Z     |
2019-07-16T05:00:54.2116692Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2121243Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2122352Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2122444Z 
2019-07-16T05:00:54.2123650Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2123961Z    --> src/libstd/sys/wasi/fs.rs:657:40
2019-07-16T05:00:54.2124165Z     |
2019-07-16T05:00:54.2124454Z 657 |     let p = CString::new(p.as_os_str().as_bytes())?;
2019-07-16T05:00:54.2125515Z     |
2019-07-16T05:00:54.2125515Z     |
2019-07-16T05:00:54.2125801Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2126107Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2126368Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2126423Z 
2019-07-16T05:00:54.2138027Z error[E0599]: no function or associated item named `from_bytes` found for type `ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2138919Z    --> src/libstd/sys/wasi/fs.rs:669:37
2019-07-16T05:00:54.2139157Z     |
2019-07-16T05:00:54.2139452Z 669 |         let path = Path::new(OsStr::from_bytes(CStr::from_ptr(ret).to_bytes()));
2019-07-16T05:00:54.2140403Z     |                                     ^^^^^^^^^^ function or associated item not found in `ffi::os_str::OsStr`
2019-07-16T05:00:54.2141013Z    ::: src/libstd/ffi/os_str.rs:100:1
2019-07-16T05:00:54.2141999Z     |
2019-07-16T05:00:54.2142326Z 100 | pub struct OsStr {
2019-07-16T05:00:54.2142326Z 100 | pub struct OsStr {
2019-07-16T05:00:54.2142757Z     | ---------------- function or associated item `from_bytes` not found for this
2019-07-16T05:00:54.2143034Z     |
2019-07-16T05:00:54.2143659Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2144127Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2144388Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2144434Z 
2019-07-16T05:00:54.2304119Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2304441Z    --> src/libstd/sys/wasi/os.rs:136:28
2019-07-16T05:00:54.2304672Z     |
2019-07-16T05:00:54.2305077Z 136 |     let k = CString::new(k.as_bytes())?;
2019-07-16T05:00:54.2305642Z     |
2019-07-16T05:00:54.2305642Z     |
2019-07-16T05:00:54.2305914Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2306236Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2306482Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2306543Z 
2019-07-16T05:00:54.2317777Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2318135Z    --> src/libstd/sys/wasi/os.rs:150:28
2019-07-16T05:00:54.2318354Z     |
2019-07-16T05:00:54.2323270Z 150 |     let k = CString::new(k.as_bytes())?;
2019-07-16T05:00:54.2324122Z     |
2019-07-16T05:00:54.2324122Z     |
2019-07-16T05:00:54.2324594Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2325115Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2325552Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2325618Z 
2019-07-16T05:00:54.2326791Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2327085Z    --> src/libstd/sys/wasi/os.rs:151:28
2019-07-16T05:00:54.2327304Z     |
2019-07-16T05:00:54.2327564Z 151 |     let v = CString::new(v.as_bytes())?;
2019-07-16T05:00:54.2328056Z     |
2019-07-16T05:00:54.2328056Z     |
2019-07-16T05:00:54.2328340Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2328647Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2329576Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2329638Z 
2019-07-16T05:00:54.2336489Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2336780Z    --> src/libstd/sys/wasi/os.rs:160:31
2019-07-16T05:00:54.2337004Z     |
2019-07-16T05:00:54.2337264Z 160 |     let nbuf = CString::new(n.as_bytes())?;
2019-07-16T05:00:54.2337758Z     |
2019-07-16T05:00:54.2337758Z     |
2019-07-16T05:00:54.2338048Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2338374Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2338631Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2338807Z 
2019-07-16T05:00:54.2584849Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2585192Z    --> src/libstd/sys/wasi/ext/fs.rs:139:56
2019-07-16T05:00:54.2585567Z     |
2019-07-16T05:00:54.2585869Z 139 |             .create_directory(dir.as_ref().as_os_str().as_bytes())
2019-07-16T05:00:54.2586400Z     |
2019-07-16T05:00:54.2586400Z     |
2019-07-16T05:00:54.2586852Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2587392Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2587654Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2587726Z 
2019-07-16T05:00:54.2602440Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2602855Z    --> src/libstd/sys/wasi/ext/fs.rs:154:52
2019-07-16T05:00:54.2603404Z     |
2019-07-16T05:00:54.2606805Z 154 |             .unlink_file(path.as_ref().as_os_str().as_bytes())
2019-07-16T05:00:54.2607587Z     |
2019-07-16T05:00:54.2607587Z     |
2019-07-16T05:00:54.2608071Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2608412Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2608698Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2608750Z 
2019-07-16T05:00:54.2610201Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2610564Z    --> src/libstd/sys/wasi/ext/fs.rs:160:57
2019-07-16T05:00:54.2610839Z     |
2019-07-16T05:00:54.2611191Z 160 |             .remove_directory(path.as_ref().as_os_str().as_bytes())
2019-07-16T05:00:54.2612077Z     |
2019-07-16T05:00:54.2612077Z     |
2019-07-16T05:00:54.2612440Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2612843Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2613446Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2613494Z 
2019-07-16T05:00:54.2661532Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2665655Z    --> src/libstd/sys/wasi/ext/fs.rs:378:39
2019-07-16T05:00:54.2665939Z     |
2019-07-16T05:00:54.2666605Z 378 |         old_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2667466Z     |
2019-07-16T05:00:54.2667466Z     |
2019-07-16T05:00:54.2667736Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2668062Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2668481Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2672241Z 
2019-07-16T05:00:54.2673447Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2673752Z    --> src/libstd/sys/wasi/ext/fs.rs:380:39
2019-07-16T05:00:54.2673999Z     |
2019-07-16T05:00:54.2677326Z 380 |         new_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2677909Z     |
2019-07-16T05:00:54.2677909Z     |
2019-07-16T05:00:54.2678200Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2678518Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2678788Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2678835Z 
2019-07-16T05:00:54.2679129Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2683516Z    --> src/libstd/sys/wasi/ext/fs.rs:394:39
2019-07-16T05:00:54.2687795Z     |
2019-07-16T05:00:54.2688288Z 394 |         old_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2714928Z     |
2019-07-16T05:00:54.2714928Z     |
2019-07-16T05:00:54.2715427Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2715767Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2716014Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2716068Z 
2019-07-16T05:00:54.2717128Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2717427Z    --> src/libstd/sys/wasi/ext/fs.rs:396:39
2019-07-16T05:00:54.2717636Z     |
2019-07-16T05:00:54.2717914Z 396 |         new_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2718409Z     |
2019-07-16T05:00:54.2718409Z     |
2019-07-16T05:00:54.2718683Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2718988Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2719713Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2719788Z 
2019-07-16T05:00:54.2720277Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2720651Z    --> src/libstd/sys/wasi/ext/fs.rs:409:39
2019-07-16T05:00:54.2720920Z     |
2019-07-16T05:00:54.2721253Z 409 |         old_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2722046Z     |
2019-07-16T05:00:54.2722046Z     |
2019-07-16T05:00:54.2722391Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2722918Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2723183Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.2723234Z 
2019-07-16T05:00:54.2723514Z error[E0599]: no method named `as_bytes` found for type `&ffi::os_str::OsStr` in the current scope
2019-07-16T05:00:54.2723775Z    --> src/libstd/sys/wasi/ext/fs.rs:410:39
2019-07-16T05:00:54.2724159Z     |
2019-07-16T05:00:54.2724426Z 410 |         new_path.as_ref().as_os_str().as_bytes(),
2019-07-16T05:00:54.2725119Z     |
2019-07-16T05:00:54.2725119Z     |
2019-07-16T05:00:54.2725404Z     = help: items from traits can only be used if the trait is in scope
2019-07-16T05:00:54.2725749Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-07-16T05:00:54.2726013Z             `use crate::sys_common::os_str_bytes::OsStrExt;`
2019-07-16T05:00:54.3983842Z error: aborting due to 43 previous errors
2019-07-16T05:00:54.3983966Z 
2019-07-16T05:00:54.3984255Z Some errors have detailed explanations: E0433, E0599.
2019-07-16T05:00:54.3984686Z For more information about an error, try `rustc --explain E0433`.
2019-07-16T05:00:54.3984686Z For more information about an error, try `rustc --explain E0433`.
2019-07-16T05:00:54.4614860Z [RUSTC-TIMING] std test:false 2.643
2019-07-16T05:00:54.4657796Z error: Could not compile `std`.
2019-07-16T05:00:54.4657867Z 
2019-07-16T05:00:54.4658157Z To learn more, run the command again with --verbose.
2019-07-16T05:00:54.4673885Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-wasi" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-16T05:00:54.4681951Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda
2019-07-16T05:00:54.4682468Z Build completed unsuccessfully in 0:58:28
2019-07-16T05:00:54.4682468Z Build completed unsuccessfully in 0:58:28
2019-07-16T05:00:56.1624835Z ##[error]Bash exited with code '1'.
2019-07-16T05:00:56.1658666Z ##[section]Starting: Upload CPU usage statistics
2019-07-16T05:00:56.1668248Z ==============================================================================
2019-07-16T05:00:56.1668362Z Task         : Bash
2019-07-16T05:00:56.1668436Z Description  : Run a Bash script on macOS, Linux, or Windows
