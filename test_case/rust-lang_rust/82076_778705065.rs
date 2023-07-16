plain
    Checking cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
warning: field is never read: `update_symbols`
   |
33 |     update_symbols: bool,
   |     ^^^^^^^^^^^^^^^^^^^^
   |
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking core v0.0.0 (/checkout/library/core)
error: trailing semicolon in macro used in expression position
  --> /checkout/library/std/src/../../stdarch/crates/std_detect/src/detect/macros.rs:21:65
   |
14 | /         macro_rules! $macro_name {
15 | |             $(
16 | |                 ($feature_lit) => {
17 | |                     $crate::detect::__is_feature_detected::$feature()
...  |
21 | |                 ($bind_feature) => { $macro_name!($feature_impl); };
...  |
43 | |             };
44 | |         }
44 | |         }
   | |_________- in this expansion of `is_x86_feature_detected!`
  ::: library/std/tests/run-time-detect.rs:60:27
   |
   |
60 |       println!("abm: {:?}", is_x86_feature_detected!("abm")); // this is a synonym for lzcnt but we test it anyways
   |
   |
   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error: aborting due to previous error


error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1821 |       m!(v[..], [N(0), ref sub @ .., N(4)] => c!(sub, &[N], n![1, 2, 3]));
     |
     |
     = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1822 |       m!(v[..], [N(0), ref sub @ ..] => c!(sub, &[N], n![1, 2, 3, 4]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1823 |       m!(v[..], [ref sub @ .., N(4)] => c!(sub, &[N], n![0, 1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1824 |       m!(v[..], [ref sub @ .., _, _, _, _, _] => c!(sub, &[N], &n![] as &[N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1825 |       m!(v[..], [_, _, _, _, _, ref sub @ ..] => c!(sub, &[N], &n![] as &[N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1826 |       m!(vc[..], [x, .., y] => c!((x, y), (u8, u8), (0, 4)));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1830 |       m!(v[..], [N(0), ref mut sub @ .., N(4)] => c!(sub, &mut [N], n![1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1831 |       m!(v[..], [N(0), ref mut sub @ ..] => c!(sub, &mut [N], n![1, 2, 3, 4]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1832 |       m!(v[..], [ref mut sub @ .., N(4)] => c!(sub, &mut [N], n![0, 1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1833 |       m!(v[..], [ref mut sub @ .., _, _, _, _, _] => c!(sub, &mut [N], &mut n![] as &mut [N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1834 |       m!(v[..], [_, _, _, _, _, ref mut sub @ ..] => c!(sub, &mut [N], &mut n![] as &mut [N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1835 |       m!(vc[..], [x, .., y] => c!((x, y), (u8, u8), (0, 4)));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1839 |       m!(&v[..], [N(0), sub @ .., N(4)] => c!(sub, &[N], n![1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1840 |       m!(&v[..], [N(0), sub @ ..] => c!(sub, &[N], n![1, 2, 3, 4]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1841 |       m!(&v[..], [sub @ .., N(4)] => c!(sub, &[N], n![0, 1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1842 |       m!(&v[..], [sub @ .., _, _, _, _, _] => c!(sub, &[N], &n![] as &[N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1843 |       m!(&v[..], [_, _, _, _, _, sub @ ..] => c!(sub, &[N], &n![] as &[N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1844 |       m!(&vc[..], [x, .., y] => c!((x, y), (&u8, &u8), (&0, &4)));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1848 |       m!(&mut v[..], [N(0), sub @ .., N(4)] => c!(sub, &mut [N], n![1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1849 |       m!(&mut v[..], [N(0), sub @ ..] => c!(sub, &mut [N], n![1, 2, 3, 4]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1850 |       m!(&mut v[..], [sub @ .., N(4)] => c!(sub, &mut [N], n![0, 1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1851 |       m!(&mut v[..], [sub @ .., _, _, _, _, _] => c!(sub, &mut [N], &mut n![] as &mut [N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1852 |       m!(&mut v[..], [_, _, _, _, _, sub @ ..] => c!(sub, &mut [N], &mut n![] as &mut [N]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1853 |       m!(&mut vc[..], [x, .., y] => c!((x, y), (&mut u8, &mut u8), (&mut 0, &mut 4)));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1860 |       m!(v.clone(), [N(0), sub @ .., N(4)] => c!(sub, [N; 3], n![1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1861 |       m!(v.clone(), [N(0), sub @ ..] => c!(sub, [N; 4], n![1, 2, 3, 4]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
...
...
1862 |       m!(v.clone(), [sub @ .., N(4)] => c!(sub, [N; 4], n![0, 1, 2, 3]));
     |
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>


error: trailing semicolon in macro used in expression position
    --> library/alloc/tests/slice.rs:1801:53
     |
1799 | /     macro_rules! c {
1800 | |         ($inp:expr, $typ:ty, $out:expr $(,)?) => {
1801 | |             assert_eq!($out, identity::<$typ>($inp));
1802 | |         };
1803 | |     }
     | |_____- in this expansion of `c!`
