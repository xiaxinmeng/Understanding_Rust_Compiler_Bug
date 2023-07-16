sh
rust/src/libsyntax_pos$ git grep '\[feature'
lib.rs:26:#![feature(const_fn)]
lib.rs:27:#![feature(custom_attribute)]
lib.rs:28:#![feature(optin_builtin_traits)]
lib.rs:30:#![feature(rustc_private)]
lib.rs:31:#![feature(staged_api)]
lib.rs:32:#![feature(specialization)]

rust/src/librustc_error$ git grep '\[feature'
lib.rs:20:#![feature(custom_attribute)]
lib.rs:22:#![feature(rustc_private)]
lib.rs:23:#![feature(staged_api)]
lib.rs:24:#![feature(range_contains)]
lib.rs:25:#![feature(libc)]

rust/src/librustc_data_structures$ git grep '\[feature'
lib.rs:28:#![feature(shared)]
lib.rs:29:#![feature(collections_range)]
lib.rs:30:#![feature(nonzero)]
lib.rs:31:#![feature(rustc_private)]
lib.rs:32:#![feature(staged_api)]
lib.rs:33:#![feature(unboxed_closures)]
lib.rs:34:#![feature(fn_traits)]
lib.rs:35:#![feature(untagged_unions)]
lib.rs:36:#![feature(associated_consts)]
lib.rs:37:#![feature(unsize)]
lib.rs:38:#![feature(i128_type)]
lib.rs:39:#![feature(conservative_impl_trait)]
lib.rs:40:#![feature(discriminant_value)]
lib.rs:41:#![feature(specialization)]
lib.rs:42:#![feature(manually_drop)]
lib.rs:43:#![feature(struct_field_attributes)]

rust/src/libsyntax$ git grep rustc_data_structures
Cargo.toml:17:rustc_data_structures = { path = "../librustc_data_structures" }
ast.rs:26:use rustc_data_structures::indexed_vec;
lib.rs:44:extern crate rustc_data_structures;
ptr.rs:46:use rustc_data_structures::stable_hasher::{StableHasher, StableHasherResult,
util/rc_slice.rs:15:use rustc_data_structures::stable_hasher::{StableHasher, StableHasherResult,
util/small_vector.rs:11:use rustc_data_structures::small_vec::SmallVec;
