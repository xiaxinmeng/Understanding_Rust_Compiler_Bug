plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0a437b2ca081bc12425a3318cb66aade9824cbae and c4773dbf0286d398bd10556bf53afb5a5c73d95e
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: method `add` can be confused for the standard trait method `std::ops::Add::add`
    |
    |
 LL | /     pub fn add(self, other: T) -> T {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
    = note: `-D clippy::should-implement-trait` implied by `-D warnings`
-   = help: consider implementing the trait `std::ops::Add` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Add` or choosing _tcx less ambiguous method name
 
 
 error: method `as_mut` can be confused for the standard trait method `std::convert::AsMut::as_mut`
    |
    |
 LL | /     pub fn as_mut(&mut self) -> &mut T {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::convert::AsMut` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::convert::AsMut` or choosing _tcx less ambiguous method name
 
 error: method `as_ref` can be confused for the standard trait method `std::convert::AsRef::as_ref`
    |
    |
 LL | /     pub fn as_ref(&self) -> &T {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::convert::AsRef` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::convert::AsRef` or choosing _tcx less ambiguous method name
 
 error: method `bitand` can be confused for the standard trait method `std::ops::BitAnd::bitand`
    |
    |
 LL | /     pub fn bitand(self, rhs: T) -> T {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::BitAnd` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::BitAnd` or choosing _tcx less ambiguous method name
 
 error: method `bitor` can be confused for the standard trait method `std::ops::BitOr::bitor`
    |
    |
 LL | /     pub fn bitor(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::BitOr` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::BitOr` or choosing _tcx less ambiguous method name
 
 error: method `bitxor` can be confused for the standard trait method `std::ops::BitXor::bitxor`
    |
    |
 LL | /     pub fn bitxor(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::BitXor` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::BitXor` or choosing _tcx less ambiguous method name
 
 error: method `borrow` can be confused for the standard trait method `std::borrow::Borrow::borrow`
    |
    |
 LL | /     pub fn borrow(&self) -> &str {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::borrow::Borrow` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::borrow::Borrow` or choosing _tcx less ambiguous method name
 
 error: method `borrow_mut` can be confused for the standard trait method `std::borrow::BorrowMut::borrow_mut`
    |
 LL | /     pub fn borrow_mut(&mut self) -> &mut str {
 LL | |         unimplemented!()
 LL | |     }
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::borrow::BorrowMut` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::borrow::BorrowMut` or choosing _tcx less ambiguous method name
 error: method `clone` can be confused for the standard trait method `std::clone::Clone::clone`
   --> $DIR/method_list_1.rs:57:5
    |
 LL | /     pub fn clone(&self) -> Self {
 LL | /     pub fn clone(&self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::clone::Clone` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::clone::Clone` or choosing _tcx less ambiguous method name
 
 error: method `cmp` can be confused for the standard trait method `std::cmp::Ord::cmp`
    |
    |
 LL | /     pub fn cmp(&self, other: &Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::cmp::Ord` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::cmp::Ord` or choosing _tcx less ambiguous method name
 
 error: method `deref` can be confused for the standard trait method `std::ops::Deref::deref`
    |
    |
 LL | /     pub fn deref(&self) -> &Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Deref` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Deref` or choosing _tcx less ambiguous method name
 
 error: method `deref_mut` can be confused for the standard trait method `std::ops::DerefMut::deref_mut`
    |
 LL | /     pub fn deref_mut(&mut self) -> &mut Self {
 LL | |         unimplemented!()
 LL | |     }
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::ops::DerefMut` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::DerefMut` or choosing _tcx less ambiguous method name
 
 error: method `div` can be confused for the standard trait method `std::ops::Div::div`
    |
    |
 LL | /     pub fn div(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Div` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Div` or choosing _tcx less ambiguous method name
 
 error: method `drop` can be confused for the standard trait method `std::ops::Drop::drop`
    |
    |
 LL | /     pub fn drop(&mut self) {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Drop` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Drop` or choosing _tcx less ambiguous method name
 error: aborting due to 14 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_1.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args should_impl_trait/method_list_1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/should_impl_trait/method_list_1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_1.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-68514ea42833abf9.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_1.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"method `add` can be confused for the standard trait method `std::ops::Add::add`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":608,"byte_end":672,"line_start":25,"line_end":27,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn add(self, other: T) -> T {","highlight_start":5,"highlight_end":38},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::should-implement-trait` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider implementing the trait `std::ops::Add` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `add` can be confused for the standard trait method `std::ops::Add::add`\n  --> tests/ui/should_impl_trait/method_list_1.rs:25:5\n   |\nLL | /     pub fn add(self, other: T) -> T {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::should-implement-trait` implied by `-D warnings`\n   = help: consider implementing the trait `std::ops::Add` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `as_mut` can be confused for the standard trait method `std::convert::AsMut::as_mut`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":678,"byte_end":745,"line_start":29,"line_end":31,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn as_mut(&mut self) -> &mut T {","highlight_start":5,"highlight_end":41},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::convert::AsMut` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `as_mut` can be confused for the standard trait method `std::convert::AsMut::as_mut`\n  --> tests/ui/should_impl_trait/method_list_1.rs:29:5\n   |\nLL | /     pub fn as_mut(&mut self) -> &mut T {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::convert::AsMut` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `as_ref` can be confused for the standard trait method `std::convert::AsRef::as_ref`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":751,"byte_end":810,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn as_ref(&self) -> &T {","highlight_start":5,"highlight_end":33},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::convert::AsRef` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `as_ref` can be confused for the standard trait method `std::convert::AsRef::as_ref`\n  --> tests/ui/should_impl_trait/method_list_1.rs:33:5\n   |\nLL | /     pub fn as_ref(&self) -> &T {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::convert::AsRef` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `bitand` can be confused for the standard trait method `std::ops::BitAnd::bitand`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":816,"byte_end":881,"line_start":37,"line_end":39,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn bitand(self, rhs: T) -> T {","highlight_start":5,"highlight_end":39},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::BitAnd` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `bitand` can be confused for the standard trait method `std::ops::BitAnd::bitand`\n  --> tests/ui/should_impl_trait/method_list_1.rs:37:5\n   |\nLL | /     pub fn bitand(self, rhs: T) -> T {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::BitAnd` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `bitor` can be confused for the standard trait method `std::ops::BitOr::bitor`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":887,"byte_end":957,"line_start":41,"line_end":43,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn bitor(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":44},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::BitOr` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `bitor` can be confused for the standard trait method `std::ops::BitOr::bitor`\n  --> tests/ui/should_impl_trait/method_list_1.rs:41:5\n   |\nLL | /     pub fn bitor(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::BitOr` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `bitxor` can be confused for the standard trait method `std::ops::BitXor::bitxor`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":963,"byte_end":1034,"line_start":45,"line_end":47,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn bitxor(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":45},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::BitXor` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `bitxor` can be confused for the standard trait method `std::ops::BitXor::bitxor`\n  --> tests/ui/should_impl_trait/method_list_1.rs:45:5\n   |\nLL | /     pub fn bitxor(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::BitXor` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `borrow` can be confused for the standard trait method `std::borrow::Borrow::borrow`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1040,"byte_end":1101,"line_start":49,"line_end":51,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn borrow(&self) -> &str {","highlight_start":5,"highlight_end":35},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::borrow::Borrow` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `borrow` can be confused for the standard trait method `std::borrow::Borrow::borrow`\n  --> tests/ui/should_impl_trait/method_list_1.rs:49:5\n   |\nLL | /     pub fn borrow(&self) -> &str {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::borrow::Borrow` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `borrow_mut` can be confused for the standard trait method `std::borrow::BorrowMut::borrow_mut`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1107,"byte_end":1180,"line_start":53,"line_end":55,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn borrow_mut(&mut self) -> &mut str {","highlight_start":5,"highlight_end":47},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::borrow::BorrowMut` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `borrow_mut` can be confused for the standard trait method `std::borrow::BorrowMut::borrow_mut`\n  --> tests/ui/should_impl_trait/method_list_1.rs:53:5\n   |\nLL | /     pub fn borrow_mut(&mut self) -> &mut str {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::borrow::BorrowMut` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `clone` can be confused for the standard trait method `std::clone::Clone::clone`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1186,"byte_end":1246,"line_start":57,"line_end":59,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn clone(&self) -> Self {","highlight_start":5,"highlight_end":34},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::clone::Clone` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `clone` can be confused for the standard trait method `std::clone::Clone::clone`\n  --> tests/ui/should_impl_trait/method_list_1.rs:57:5\n   |\nLL | /     pub fn clone(&self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::clone::Clone` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `cmp` can be confused for the standard trait method `std::cmp::Ord::cmp`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1252,"byte_end":1324,"line_start":61,"line_end":63,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn cmp(&self, other: &Self) -> Self {","highlight_start":5,"highlight_end":46},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::cmp::Ord` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `cmp` can be confused for the standard trait method `std::cmp::Ord::cmp`\n  --> tests/ui/should_impl_trait/method_list_1.rs:61:5\n   |\nLL | /     pub fn cmp(&self, other: &Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::cmp::Ord` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `deref` can be confused for the standard trait method `std::ops::Deref::deref`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1393,"byte_end":1454,"line_start":69,"line_end":71,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn deref(&self) -> &Self {","highlight_start":5,"highlight_end":35},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Deref` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `deref` can be confused for the standard trait method `std::ops::Deref::deref`\n  --> tests/ui/should_impl_trait/method_list_1.rs:69:5\n   |\nLL | /     pub fn deref(&self) -> &Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Deref` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `deref_mut` can be confused for the standard trait method `std::ops::DerefMut::deref_mut`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1460,"byte_end":1533,"line_start":73,"line_end":75,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn deref_mut(&mut self) -> &mut Self {","highlight_start":5,"highlight_end":47},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::DerefMut` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `deref_mut` can be confused for the standard trait method `std::ops::DerefMut::deref_mut`\n  --> tests/ui/should_impl_trait/method_list_1.rs:73:5\n   |\nLL | /     pub fn deref_mut(&mut self) -> &mut Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::DerefMut` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `div` can be confused for the standard trait method `std::ops::Div::div`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1539,"byte_end":1607,"line_start":77,"line_end":79,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn div(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Div` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `div` can be confused for the standard trait method `std::ops::Div::div`\n  --> tests/ui/should_impl_trait/method_list_1.rs:77:5\n   |\nLL | /     pub fn div(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Div` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `drop` can be confused for the standard trait method `std::ops::Drop::drop`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_1.rs","byte_start":1613,"byte_end":1668,"line_start":81,"line_end":83,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn drop(&mut self) {","highlight_start":5,"highlight_end":29},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Drop` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `drop` can be confused for the standard trait method `std::ops::Drop::drop`\n  --> tests/ui/should_impl_trait/method_list_1.rs:81:5\n   |\nLL | /     pub fn drop(&mut self) {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Drop` or choosing _tcx less ambiguous method name\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: method `eq` can be confused for the standard trait method `std::cmp::PartialEq::eq`
    |
    |
 LL | /     pub fn eq(&self, other: &Self) -> bool {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
    = note: `-D clippy::should-implement-trait` implied by `-D warnings`
-   = help: consider implementing the trait `std::cmp::PartialEq` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::cmp::PartialEq` or choosing _tcx less ambiguous method name
 
 error: method `from_iter` can be confused for the standard trait method `std::iter::FromIterator::from_iter`
    |
    |
 LL | /     pub fn from_iter<T>(iter: T) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::iter::FromIterator` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::iter::FromIterator` or choosing _tcx less ambiguous method name
 
 error: method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`
    |
    |
 LL | /     pub fn from_str(s: &str) -> Result<Self, Self> {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::str::FromStr` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::str::FromStr` or choosing _tcx less ambiguous method name
 
 error: method `hash` can be confused for the standard trait method `std::hash::Hash::hash`
    |
    |
 LL | /     pub fn hash(&self, state: &mut T) {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::hash::Hash` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::hash::Hash` or choosing _tcx less ambiguous method name
 error: method `index` can be confused for the standard trait method `std::ops::Index::index`
   --> $DIR/method_list_2.rs:42:5
    |
    |
 LL | /     pub fn index(&self, index: usize) -> &Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Index` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Index` or choosing _tcx less ambiguous method name
 
 error: method `index_mut` can be confused for the standard trait method `std::ops::IndexMut::index_mut`
    |
 LL | /     pub fn index_mut(&mut self, index: usize) -> &mut Self {
 LL | |         unimplemented!()
 LL | |     }
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::ops::IndexMut` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::IndexMut` or choosing _tcx less ambiguous method name
 error: method `into_iter` can be confused for the standard trait method `std::iter::IntoIterator::into_iter`
   --> $DIR/method_list_2.rs:50:5
    |
 LL | /     pub fn into_iter(self) -> Self {
 LL | /     pub fn into_iter(self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::iter::IntoIterator` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::iter::IntoIterator` or choosing _tcx less ambiguous method name
 
 error: method `mul` can be confused for the standard trait method `std::ops::Mul::mul`
    |
    |
 LL | /     pub fn mul(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Mul` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Mul` or choosing _tcx less ambiguous method name
 
 error: method `neg` can be confused for the standard trait method `std::ops::Neg::neg`
    |
    |
 LL | /     pub fn neg(self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Neg` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Neg` or choosing _tcx less ambiguous method name
 
 error: method `next` can be confused for the standard trait method `std::iter::Iterator::next`
    |
    |
 LL | /     pub fn next(&mut self) -> Option<Self> {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::iter::Iterator` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::iter::Iterator` or choosing _tcx less ambiguous method name
 
 error: method `not` can be confused for the standard trait method `std::ops::Not::not`
    |
 LL | /     pub fn not(self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
 LL | |     }
    | |_____^
    |
-   = help: consider implementing the trait `std::ops::Not` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Not` or choosing _tcx less ambiguous method name
 
 error: method `rem` can be confused for the standard trait method `std::ops::Rem::rem`
    |
    |
 LL | /     pub fn rem(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Rem` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Rem` or choosing _tcx less ambiguous method name
 
 error: method `shl` can be confused for the standard trait method `std::ops::Shl::shl`
    |
    |
 LL | /     pub fn shl(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Shl` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Shl` or choosing _tcx less ambiguous method name
 
 error: method `shr` can be confused for the standard trait method `std::ops::Shr::shr`
    |
    |
 LL | /     pub fn shr(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Shr` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Shr` or choosing _tcx less ambiguous method name
 
 error: method `sub` can be confused for the standard trait method `std::ops::Sub::sub`
    |
    |
 LL | /     pub fn sub(self, rhs: Self) -> Self {
 LL | |         unimplemented!()
 LL | |     }
    |
    |
-   = help: consider implementing the trait `std::ops::Sub` or choosing a less ambiguous method name
+   = help: consider implementing the trait `std::ops::Sub` or choosing _tcx less ambiguous method name
 error: aborting due to 15 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_2.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args should_impl_trait/method_list_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/should_impl_trait/method_list_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-68514ea42833abf9.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/should_impl_trait/method_list_2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"method `eq` can be confused for the standard trait method `std::cmp::PartialEq::eq`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":609,"byte_end":680,"line_start":26,"line_end":28,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn eq(&self, other: &Self) -> bool {","highlight_start":5,"highlight_end":45},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::should-implement-trait` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider implementing the trait `std::cmp::PartialEq` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `eq` can be confused for the standard trait method `std::cmp::PartialEq::eq`\n  --> tests/ui/should_impl_trait/method_list_2.rs:26:5\n   |\nLL | /     pub fn eq(&self, other: &Self) -> bool {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::should-implement-trait` implied by `-D warnings`\n   = help: consider implementing the trait `std::cmp::PartialEq` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `from_iter` can be confused for the standard trait method `std::iter::FromIterator::from_iter`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":686,"byte_end":755,"line_start":30,"line_end":32,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn from_iter<T>(iter: T) -> Self {","highlight_start":5,"highlight_end":43},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::iter::FromIterator` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `from_iter` can be confused for the standard trait method `std::iter::FromIterator::from_iter`\n  --> tests/ui/should_impl_trait/method_list_2.rs:30:5\n   |\nLL | /     pub fn from_iter<T>(iter: T) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::iter::FromIterator` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":761,"byte_end":840,"line_start":34,"line_end":36,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn from_str(s: &str) -> Result<Self, Self> {","highlight_start":5,"highlight_end":53},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::str::FromStr` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `from_str` can be confused for the standard trait method `std::str::FromStr::from_str`\n  --> tests/ui/should_impl_trait/method_list_2.rs:34:5\n   |\nLL | /     pub fn from_str(s: &str) -> Result<Self, Self> {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::str::FromStr` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `hash` can be confused for the standard trait method `std::hash::Hash::hash`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":846,"byte_end":912,"line_start":38,"line_end":40,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn hash(&self, state: &mut T) {","highlight_start":5,"highlight_end":40},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::hash::Hash` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `hash` can be confused for the standard trait method `std::hash::Hash::hash`\n  --> tests/ui/should_impl_trait/method_list_2.rs:38:5\n   |\nLL | /     pub fn hash(&self, state: &mut T) {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::hash::Hash` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `index` can be confused for the standard trait method `std::ops::Index::index`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":918,"byte_end":993,"line_start":42,"line_end":44,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn index(&self, index: usize) -> &Self {","highlight_start":5,"highlight_end":49},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Index` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `index` can be confused for the standard trait method `std::ops::Index::index`\n  --> tests/ui/should_impl_trait/method_list_2.rs:42:5\n   |\nLL | /     pub fn index(&self, index: usize) -> &Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Index` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `index_mut` can be confused for the standard trait method `std::ops::IndexMut::index_mut`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":999,"byte_end":1086,"line_start":46,"line_end":48,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn index_mut(&mut self, index: usize) -> &mut Self {","highlight_start":5,"highlight_end":61},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::IndexMut` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `index_mut` can be confused for the standard trait method `std::ops::IndexMut::index_mut`\n  --> tests/ui/should_impl_trait/method_list_2.rs:46:5\n   |\nLL | /     pub fn index_mut(&mut self, index: usize) -> &mut Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::IndexMut` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `into_iter` can be confused for the standard trait method `std::iter::IntoIterator::into_iter`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1092,"byte_end":1155,"line_start":50,"line_end":52,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn into_iter(self) -> Self {","highlight_start":5,"highlight_end":37},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::iter::IntoIterator` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `into_iter` can be confused for the standard trait method `std::iter::IntoIterator::into_iter`\n  --> tests/ui/should_impl_trait/method_list_2.rs:50:5\n   |\nLL | /     pub fn into_iter(self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::iter::IntoIterator` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `mul` can be confused for the standard trait method `std::ops::Mul::mul`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1161,"byte_end":1229,"line_start":54,"line_end":56,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn mul(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Mul` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `mul` can be confused for the standard trait method `std::ops::Mul::mul`\n  --> tests/ui/should_impl_trait/method_list_2.rs:54:5\n   |\nLL | /     pub fn mul(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Mul` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `neg` can be confused for the standard trait method `std::ops::Neg::neg`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1235,"byte_end":1292,"line_start":58,"line_end":60,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn neg(self) -> Self {","highlight_start":5,"highlight_end":31},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Neg` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `neg` can be confused for the standard trait method `std::ops::Neg::neg`\n  --> tests/ui/should_impl_trait/method_list_2.rs:58:5\n   |\nLL | /     pub fn neg(self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Neg` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `next` can be confused for the standard trait method `std::iter::Iterator::next`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1298,"byte_end":1369,"line_start":62,"line_end":64,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn next(&mut self) -> Option<Self> {","highlight_start":5,"highlight_end":45},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::iter::Iterator` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `next` can be confused for the standard trait method `std::iter::Iterator::next`\n  --> tests/ui/should_impl_trait/method_list_2.rs:62:5\n   |\nLL | /     pub fn next(&mut self) -> Option<Self> {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::iter::Iterator` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `not` can be confused for the standard trait method `std::ops::Not::not`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1375,"byte_end":1432,"line_start":66,"line_end":68,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn not(self) -> Self {","highlight_start":5,"highlight_end":31},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Not` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `not` can be confused for the standard trait method `std::ops::Not::not`\n  --> tests/ui/should_impl_trait/method_list_2.rs:66:5\n   |\nLL | /     pub fn not(self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Not` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `rem` can be confused for the standard trait method `std::ops::Rem::rem`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1438,"byte_end":1506,"line_start":70,"line_end":72,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn rem(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Rem` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `rem` can be confused for the standard trait method `std::ops::Rem::rem`\n  --> tests/ui/should_impl_trait/method_list_2.rs:70:5\n   |\nLL | /     pub fn rem(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Rem` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `shl` can be confused for the standard trait method `std::ops::Shl::shl`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1512,"byte_end":1580,"line_start":74,"line_end":76,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn shl(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Shl` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `shl` can be confused for the standard trait method `std::ops::Shl::shl`\n  --> tests/ui/should_impl_trait/method_list_2.rs:74:5\n   |\nLL | /     pub fn shl(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Shl` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `shr` can be confused for the standard trait method `std::ops::Shr::shr`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1586,"byte_end":1654,"line_start":78,"line_end":80,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn shr(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Shr` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `shr` can be confused for the standard trait method `std::ops::Shr::shr`\n  --> tests/ui/should_impl_trait/method_list_2.rs:78:5\n   |\nLL | /     pub fn shr(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Shr` or choosing _tcx less ambiguous method name\n\n"}
{"message":"method `sub` can be confused for the standard trait method `std::ops::Sub::sub`","code":{"code":"clippy::should_implement_trait","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/should_impl_trait/method_list_2.rs","byte_start":1660,"byte_end":1728,"line_start":82,"line_end":84,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn sub(self, rhs: Self) -> Self {","highlight_start":5,"highlight_end":42},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider implementing the trait `std::ops::Sub` or choosing _tcx less ambiguous method name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: method `sub` can be confused for the standard trait method `std::ops::Sub::sub`\n  --> tests/ui/should_impl_trait/method_list_2.rs:82:5\n   |\nLL | /     pub fn sub(self, rhs: Self) -> Self {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\n   = help: consider implementing the trait `std::ops::Sub` or choosing _tcx less ambiguous method name\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
