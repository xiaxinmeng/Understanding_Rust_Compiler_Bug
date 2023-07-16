plain
2019-08-09T15:56:03.3741410Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T15:56:03.3741619Z 
2019-08-09T15:56:03.3741988Z   git checkout -b <new-branch-name>
2019-08-09T15:56:03.3742154Z 
2019-08-09T15:56:03.3742607Z HEAD is now at f514b9d29 Auto merge of #63412 - Centril:rollup-5lvzexh, r=Centril
2019-08-09T15:56:03.3890421Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T15:56:03.3893189Z ==============================================================================
2019-08-09T15:56:03.3893288Z Task         : Bash
2019-08-09T15:56:03.3893351Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T17:42:11.3691386Z [RUSTC-TIMING] mdbook_linkcheck test:false 5.659
2019-08-09T17:42:11.3696863Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-08-09T17:42:16.0070923Z [RUSTC-TIMING] rustbook test:false 4.367
2019-08-09T17:42:16.0072549Z     Finished release [optimized] target(s) in 8m 10s
2019-08-09T17:42:16.0073258Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["linkcheck"] } -- 490.572
2019-08-09T17:43:57.9346027Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-09T17:43:57.9346469Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-08-09T17:43:57.9392880Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2019-08-09T17:43:58.2786309Z    Compiling proc-macro2 v0.4.30
---
2019-08-09T18:03:47.8670755Z    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8670830Z    |
2019-08-09T18:03:47.8671190Z    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
2019-08-09T18:03:47.8671263Z 
2019-08-09T18:03:47.8671524Z error: the type `&T` does not permit zero-initialization
2019-08-09T18:03:47.8671834Z    |
2019-08-09T18:03:47.8671900Z LL |     let ref_zero: &T = std::mem::zeroed(); // warning
2019-08-09T18:03:47.8671990Z    |                        ^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8672053Z    |
2019-08-09T18:03:47.8672053Z    |
2019-08-09T18:03:47.8672312Z    = note: `-D invalid-value` implied by `-D warnings`
2019-08-09T18:03:47.8672398Z    = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8672531Z 
2019-08-09T18:03:47.8672531Z 
2019-08-09T18:03:47.8672898Z error: the type `&T` does not permit zero-initialization
2019-08-09T18:03:47.8673208Z    |
2019-08-09T18:03:47.8673271Z LL |     let ref_zero: &T = core::mem::zeroed(); // warning
2019-08-09T18:03:47.8673455Z    |                        ^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8673528Z    |
2019-08-09T18:03:47.8673528Z    |
2019-08-09T18:03:47.8673609Z    = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8673683Z    = help: use `MaybeUninit` instead
2019-08-09T18:03:47.8673723Z 
2019-08-09T18:03:47.8673801Z error: the type `&T` does not permit being left uninitialized
2019-08-09T18:03:47.8674144Z    |
2019-08-09T18:03:47.8674208Z LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
2019-08-09T18:03:47.8674299Z    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8674361Z    |
2019-08-09T18:03:47.8674361Z    |
2019-08-09T18:03:47.8674441Z    = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8674512Z    = help: use `MaybeUninit` instead
2019-08-09T18:03:47.8674577Z 
2019-08-09T18:03:47.8674639Z error: the type `&T` does not permit being left uninitialized
2019-08-09T18:03:47.8674957Z    |
2019-08-09T18:03:47.8675037Z LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
2019-08-09T18:03:47.8675113Z    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8675190Z    |
---
2019-08-09T18:03:47.8686598Z     |
2019-08-09T18:03:47.8686922Z     = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
2019-08-09T18:03:47.8687025Z  
2019-08-09T18:03:47.8687250Z -error: aborting due to 5 previous errors
2019-08-09T18:03:47.8687511Z +error: the type `&T` does not permit zero-initialization
2019-08-09T18:03:47.8687821Z +   |
2019-08-09T18:03:47.8687884Z +LL |     let ref_zero: &T = std::mem::zeroed(); // warning
2019-08-09T18:03:47.8688045Z +   |                        ^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8688116Z +   |
2019-08-09T18:03:47.8688116Z +   |
2019-08-09T18:03:47.8688391Z +   = note: `-D invalid-value` implied by `-D warnings`
2019-08-09T18:03:47.8688471Z +   = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8688619Z +
2019-08-09T18:03:47.8688619Z +
2019-08-09T18:03:47.8688994Z +error: the type `&T` does not permit zero-initialization
2019-08-09T18:03:47.8689419Z +   |
2019-08-09T18:03:47.8689484Z +LL |     let ref_zero: &T = core::mem::zeroed(); // warning
2019-08-09T18:03:47.8689571Z +   |                        ^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8689635Z +   |
2019-08-09T18:03:47.8689635Z +   |
2019-08-09T18:03:47.8689716Z +   = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8689799Z +   = help: use `MaybeUninit` instead
2019-08-09T18:03:47.8689873Z +
2019-08-09T18:03:47.8689945Z +error: the type `&T` does not permit being left uninitialized
2019-08-09T18:03:47.8690275Z +   |
2019-08-09T18:03:47.8690356Z +LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
2019-08-09T18:03:47.8690448Z +   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8690514Z +   |
2019-08-09T18:03:47.8690514Z +   |
2019-08-09T18:03:47.8690595Z +   = note: this means that this code causes undefined behavior when executed
2019-08-09T18:03:47.8690669Z +   = help: use `MaybeUninit` instead
2019-08-09T18:03:47.8690744Z +
2019-08-09T18:03:47.8690810Z +error: the type `&T` does not permit being left uninitialized
2019-08-09T18:03:47.8691131Z +   |
2019-08-09T18:03:47.8691210Z +LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
2019-08-09T18:03:47.8691290Z +   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-09T18:03:47.8691462Z +   |
---
2019-08-09T18:03:47.8700498Z {"message":"reference to zeroed memory","code":{"code":"clippy::invalid_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":598,"byte_end":617,"line_start":27,"line_end":27,"column_start":24,"column_end":43,"is_primary":true,"text":[{"text":"    let ref_zero: &T = core::mem::zeroed(); // warning","highlight_start":24,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: reference to zeroed memory\n  --> tests/ui/invalid_ref.rs:27:24\n   |\nLL |     let ref_zero: &T = core::mem::zeroed(); // warning\n   |                        ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html\n\n"}
2019-08-09T18:03:47.8702403Z {"message":"reference to zeroed memory","code":{"code":"clippy::invalid_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":705,"byte_end":728,"line_start":31,"line_end":31,"column_start":24,"column_end":47,"is_primary":true,"text":[{"text":"    let ref_zero: &T = std::intrinsics::init(); // warning","highlight_start":24,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: reference to zeroed memory\n  --> tests/ui/invalid_ref.rs:31:24\n   |\nLL |     let ref_zero: &T = std::intrinsics::init(); // warning\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html\n\n"}
2019-08-09T18:03:47.8704390Z {"message":"reference to uninitialized memory","code":{"code":"clippy::invalid_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":817,"byte_end":842,"line_start":35,"line_end":35,"column_start":26,"column_end":51,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = std::mem::uninitialized(); // warning","highlight_start":26,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: reference to uninitialized memory\n  --> tests/ui/invalid_ref.rs:35:26\n   |\nLL |     let ref_uninit: &T = std::mem::uninitialized(); // warning\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html\n\n"}
2019-08-09T18:03:47.8706361Z {"message":"reference to uninitialized memory","code":{"code":"clippy::invalid_ref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":932,"byte_end":958,"line_start":39,"line_end":39,"column_start":26,"column_end":52,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = core::mem::uninitialized(); // warning","highlight_start":26,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: reference to uninitialized memory\n  --> tests/ui/invalid_ref.rs:39:26\n   |\nLL |     let ref_uninit: &T = core::mem::uninitialized(); // warning\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html\n\n"}
2019-08-09T18:03:47.8708852Z {"message":"the type `&T` does not permit zero-initialization","code":{"code":"invalid_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":492,"byte_end":510,"line_start":23,"line_end":23,"column_start":24,"column_end":42,"is_primary":true,"text":[{"text":"    let ref_zero: &T = std::mem::zeroed(); // warning","highlight_start":24,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D invalid-value` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this means that this code causes undefined behavior when executed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `MaybeUninit` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: the type `&T` does not permit zero-initialization\n  --> tests/ui/invalid_ref.rs:23:24\n   |\nLL |     let ref_zero: &T = std::mem::zeroed(); // warning\n   |                        ^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D invalid-value` implied by `-D warnings`\n   = note: this means that this code causes undefined behavior when executed\n   = help: use `MaybeUninit` instead\n\n"}
2019-08-09T18:03:47.8711489Z {"message":"the type `&T` does not permit zero-initialization","code":{"code":"invalid_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":598,"byte_end":617,"line_start":27,"line_end":27,"column_start":24,"column_end":43,"is_primary":true,"text":[{"text":"    let ref_zero: &T = core::mem::zeroed(); // warning","highlight_start":24,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this means that this code causes undefined behavior when executed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `MaybeUninit` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: the type `&T` does not permit zero-initialization\n  --> tests/ui/invalid_ref.rs:27:24\n   |\nLL |     let ref_zero: &T = core::mem::zeroed(); // warning\n   |                        ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this means that this code causes undefined behavior when executed\n   = help: use `MaybeUninit` instead\n\n"}
2019-08-09T18:03:47.8713850Z {"message":"the type `&T` does not permit being left uninitialized","code":{"code":"invalid_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":817,"byte_end":842,"line_start":35,"line_end":35,"column_start":26,"column_end":51,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = std::mem::uninitialized(); // warning","highlight_start":26,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this means that this code causes undefined behavior when executed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `MaybeUninit` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: the type `&T` does not permit being left uninitialized\n  --> tests/ui/invalid_ref.rs:35:26\n   |\nLL |     let ref_uninit: &T = std::mem::uninitialized(); // warning\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this means that this code causes undefined behavior when executed\n   = help: use `MaybeUninit` instead\n\n"}
2019-08-09T18:03:47.8715814Z {"message":"the type `&T` does not permit being left uninitialized","code":{"code":"invalid_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/invalid_ref.rs","byte_start":932,"byte_end":958,"line_start":39,"line_end":39,"column_start":26,"column_end":52,"is_primary":true,"text":[{"text":"    let ref_uninit: &T = core::mem::uninitialized(); // warning","highlight_start":26,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this means that this code causes undefined behavior when executed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `MaybeUninit` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: the type `&T` does not permit being left uninitialized\n  --> tests/ui/invalid_ref.rs:39:26\n   |\nLL |     let ref_uninit: &T = core::mem::uninitialized(); // warning\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this means that this code causes undefined behavior when executed\n   = help: use `MaybeUninit` instead\n\n"}
2019-08-09T18:03:47.8716557Z 
2019-08-09T18:03:47.8716846Z ------------------------------------------
2019-08-09T18:03:47.8716908Z 
2019-08-09T18:03:47.8717266Z thread '[ui] ui/invalid_ref.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
2019-08-09T18:40:45.1080820Z [RUSTC-TIMING] proc_macro test:false 21.013
2019-08-09T18:40:45.1086747Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-08-09T18:40:57.4278729Z [RUSTC-TIMING] test test:false 12.316
2019-08-09T18:40:57.4284214Z     Finished release [optimized] target(s) in 33.75s
2019-08-09T18:40:57.5544396Z [RUSTC-TIMING] miri_xargo test:false 0.048
2019-08-09T18:40:57.5568015Z A libstd for Miri is now available in `/home/user/.cache/miri/HOST`.
2019-08-09T18:40:58.4260372Z    Compiling compiletest_rs v0.3.22
2019-08-09T18:40:58.8283139Z [RUSTC-TIMING] build_script_build test:false 0.398
2019-08-09T18:40:58.8324981Z    Compiling lazy_static v0.2.11
2019-08-09T18:40:58.9480190Z [RUSTC-TIMING] lazy_static test:false 0.111
---
2019-08-09T18:43:05.1754314Z Verifying status of rustc-guide...
2019-08-09T18:43:05.1876012Z Cloning into 'rust-toolstate'...
2019-08-09T18:43:06.1255494Z The state of "clippy-driver" has changed from "test-pass" to "test-fail"
2019-08-09T18:43:06.1389656Z The state of "clippy-driver" has regressed from "test-pass" to "test-fail"
2019-08-09T18:43:06.7117377Z ##[error]Bash exited with code '1'.
2019-08-09T18:43:06.7160640Z ##[section]Starting: Upload CPU usage statistics
2019-08-09T18:43:06.7170332Z ==============================================================================
2019-08-09T18:43:06.7170441Z Task         : Bash
2019-08-09T18:43:06.7170510Z Description  : Run a Bash script on macOS, Linux, or Windows
