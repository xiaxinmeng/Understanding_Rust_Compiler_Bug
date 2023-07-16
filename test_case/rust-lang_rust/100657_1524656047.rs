
PS C:\Users\jyn\src\rust> rg supported_split_debuginfo .\compiler\rustc_target\src\spec -A3 -g '!compiler/rustc_target/src/spec/mod.rs'
.\compiler\rustc_target\src\spec\windows_gnullvm_base.rs
44:        supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
45-        ..Default::default()
46-    }
47-}

.\compiler\rustc_target\src\spec\windows_gnu_base.rs
104:        supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
105-        ..Default::default()
106-    }
107-}

.\compiler\rustc_target\src\spec\msvc_base.rs
21:        supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Packed]),
22-        debuginfo_kind: DebuginfoKind::Pdb,
23-
24-        ..Default::default()

.\compiler\rustc_target\src\spec\linux_base.rs
14:        supported_split_debuginfo: Cow::Borrowed(&[
15-            SplitDebuginfo::Packed,
16-            SplitDebuginfo::Unpacked,
17-            SplitDebuginfo::Off,

.\compiler\rustc_target\src\spec\apple_base.rs
156:        supported_split_debuginfo: Cow::Borrowed(&[
157-            SplitDebuginfo::Packed,
158-            SplitDebuginfo::Unpacked,
159-            SplitDebuginfo::Off,

PS C:\Users\jyn\src\rust> rg -B1 \[SplitDebuginfo .\compiler\rustc_target\src\spec\mod.rs
1711-    /// Which kinds of split debuginfo are supported by the target?
1712:    pub supported_split_debuginfo: StaticCow<[SplitDebuginfo]>,
--
1958-            // `Off` is supported by default, but targets can remove this manually, e.g. Windows.
1959:            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
