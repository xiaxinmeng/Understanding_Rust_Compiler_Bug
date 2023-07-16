plain
[TIMING] StdLink { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target_compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.001
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
Build completed successfully in 0:17:18
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:66:1
66 | #![feature(rustc_allow_const_fn_unstable)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:67:1
67 | #![feature(allow_internal_unstable)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:68:1
68 | #![feature(arbitrary_self_types)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:69:1
69 | #![feature(asm)]
   | ^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:70:1
   |
70 | #![feature(cfg_target_has_atomic)]


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:71:29
   |
71 | #![cfg_attr(not(bootstrap), feature(const_heap))]


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:72:1
72 | #![feature(const_alloc_layout)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:73:1
73 | #![feature(const_assert_type)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:74:1
74 | #![feature(const_discriminant)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:75:1
75 | #![feature(const_cell_into_inner)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:76:1
76 | #![feature(const_checked_int_methods)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:77:1
   |
77 | #![feature(const_euclidean_int_methods)]


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:78:1
78 | #![feature(const_float_classify)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:79:1
79 | #![feature(const_float_bits_conv)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:80:1
80 | #![feature(const_overflowing_int_methods)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:81:1
81 | #![feature(const_int_unchecked_arith)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:82:1
82 | #![feature(const_mut_refs)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:83:1
83 | #![feature(const_cttz)]
   | ^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:84:1
84 | #![feature(const_panic)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:85:1
85 | #![feature(const_pin)]
   | ^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:86:1
86 | #![feature(const_fn)]
   | ^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:87:1
87 | #![feature(const_fn_union)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:88:1
88 | #![feature(const_impl_trait)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:89:1
89 | #![feature(const_fn_floating_point_arithmetic)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:90:1
90 | #![feature(const_fn_fn_ptr_basics)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:91:1
91 | #![feature(const_generics)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:92:1
92 | #![feature(const_option)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:93:1
93 | #![feature(const_precise_live_drops)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:94:1
94 | #![feature(const_ptr_offset)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:95:1
95 | #![feature(const_ptr_offset_from)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:96:1
96 | #![feature(const_raw_ptr_comparison)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:97:1
97 | #![feature(const_raw_ptr_deref)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:98:1
98 | #![feature(const_slice_from_raw_parts)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
  --> ../library/core/src/lib.rs:99:1
99 | #![feature(const_slice_ptr_len)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:100:1
100 | #![feature(const_size_of_val)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:101:1
101 | #![feature(const_align_of_val)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:102:1
102 | #![feature(const_type_id)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:103:1
103 | #![feature(const_type_name)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:104:1
104 | #![feature(const_likely)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:105:1
105 | #![feature(const_unreachable_unchecked)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:106:1
106 | #![feature(const_maybe_uninit_assume_init)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:107:1
107 | #![feature(const_maybe_uninit_as_ptr)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:108:1
    |
108 | #![feature(custom_inner_attributes)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:109:1
    |
109 | #![feature(decl_macro)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:110:1
    |
110 | #![feature(doc_cfg)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:111:1
    |
111 | #![feature(doc_spotlight)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:112:1
    |
112 | #![feature(duration_consts_2)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:113:1
    |
113 | #![feature(duration_saturating_ops)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:114:1
114 | #![feature(extern_types)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:115:1
115 | #![feature(fundamental)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:116:1
116 | #![feature(intrinsics)]
    | ^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:117:1
117 | #![feature(lang_items)]
    | ^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:118:1
118 | #![feature(link_llvm_intrinsics)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:119:1
119 | #![feature(llvm_asm)]
    | ^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:120:1
    |
120 | #![feature(negative_impls)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:121:1
    |
121 | #![feature(never_type)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:122:1
122 | #![feature(nll)]
    | ^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:123:1
123 | #![feature(exhaustive_patterns)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:124:1
    |
124 | #![feature(no_core)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:126:29
    |
126 | #![cfg_attr(not(bootstrap), feature(auto_traits))]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:127:1
127 | #![feature(or_patterns)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:128:1
128 | #![feature(prelude_import)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:129:1
    |
129 | #![feature(repr_simd, platform_intrinsics)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:130:1
    |
130 | #![feature(rustc_attrs)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:131:1
    |
131 | #![feature(simd_ffi)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:132:1
132 | #![feature(min_specialization)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:133:1
133 | #![feature(staged_api)]
    | ^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:134:1
    |
134 | #![feature(std_internals)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:135:1
    |
135 | #![feature(stmt_expr_attributes)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:136:1
    |
136 | #![feature(str_split_as_str)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:137:1
    |
137 | #![feature(str_split_inclusive_as_str)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:138:1
138 | #![feature(transparent_unions)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:139:1
139 | #![feature(try_blocks)]
    | ^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:140:1
140 | #![feature(unboxed_closures)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:141:1
141 | #![feature(unsized_fn_params)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:142:1
    |
142 | #![feature(unwind_attributes)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:143:1
143 | #![feature(variant_count)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:144:1
    |
144 | #![feature(tbm_target_feature)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:145:1
    |
145 | #![feature(sse4a_target_feature)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:146:1
    |
146 | #![feature(arm_target_feature)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:147:1
    |
147 | #![feature(powerpc_target_feature)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:148:1
    |
148 | #![feature(mips_target_feature)]


error[E0554]: `#![feature]` may not be used on the beta release channel
   --> ../library/core/src/lib.rs:149:1
149 | #![feature(aarch64_target_feature)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
