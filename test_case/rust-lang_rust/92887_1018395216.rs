plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking object v0.26.2
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/checkout/library/std)
error: duplicate lang item in crate `core` (which `unwind` depends on): `bool`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `char`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `str`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `array`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `slice`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `slice_u8`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `const_ptr`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `mut_ptr`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `const_slice_ptr`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `mut_slice_ptr`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `i8`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `i16`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `i32`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `i64`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `i128`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `isize`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `u8`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `u16`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `u32`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `u64`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `u128`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `usize`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `f32`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `f64`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `sized`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `unsize`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `structural_peq`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `structural_teq`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `copy`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `clone`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `sync`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `discriminant_kind`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `discriminant_type`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `pointee_trait`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `metadata_type`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `dyn_metadata`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `freeze`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `drop`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `coerce_unsized`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `dispatch_from_dyn`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `add`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `sub`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `mul`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `div`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `rem`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `neg`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `not`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitxor`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitand`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitor`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `shl`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `shr`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `add_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `sub_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `mul_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `div_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `rem_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitxor_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitand_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `bitor_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `shl_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `shr_assign`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `index`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `index_mut`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `unsafe_cell`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `va_list`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `deref`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `deref_mut`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `deref_target`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `receiver`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `fn`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `fn_mut`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `fn_once`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `fn_once_output`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `future_trait`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `generator_state`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `generator`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `generator_return`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `unpin`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `pin`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `eq`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `partial_ord`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
  = note: second definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-19a1c1f298a9cfda.rmeta

error: duplicate lang item in crate `core` (which `unwind` depends on): `panic`.
  |
  = note: the lang item is first defined in crate `core` (which `std` depends on)
  = note: first definition in `core` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta
---
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

error[E0277]: the size for values of type `ffi::os_str::OsString` cannot be known at compilation time
    |
    |
840 |     type Item = OsString;
    |
    = help: the trait `core::marker::Sized` is not implemented for `ffi::os_str::OsString`
note: required by a bound in `core::iter::Iterator::Item`
   --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
   --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
520 | impl<'a, T: Error + ?Sized> Error for &'a T {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
454 | impl<T: ?Sized + AsRef<OsStr>> From<&T> for OsString {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
     |
     |
1614 | impl<T: ?Sized + AsRef<OsStr>> From<&T> for PathBuf {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
541 | impl<T: Error + ?Sized> Error for Arc<T> {


error[E0277]: the size for values of type `&'a (dyn error::Error + 'static)` cannot be known at compilation time
    |
    |
779 |     type Item = &'a (dyn Error + 'static);
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'a (dyn error::Error + 'static)`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
   --> library/std/src/fs.rs:186:38
    |
186 | #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |                                      ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
53 | impl<W: Write + ?Sized> Write for &mut W {


error[E0277]: the size for values of type `core::result::Result<fs::DirEntry, io::error::Error>` cannot be known at compilation time
     |
1408 |     type Item = io::Result<DirEntry>;
     |                 ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<fs::DirEntry, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
     |
     |
1640 |         impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
     |
     |
1635 |         struct Adapter<'a, T: ?Sized + 'a> {


error[E0277]: the size for values of type `core::result::Result<u8, io::error::Error>` cannot be known at compilation time
     |
     |
2664 |     type Item = Result<u8>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<u8, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `core::result::Result<alloc_crate::vec::Vec<u8>, io::error::Error>` cannot be known at compilation time
     |
     |
2757 |     type Item = Result<Vec<u8>>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<alloc_crate::vec::Vec<u8>, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `core::result::Result<alloc_crate::string::String, io::error::Error>` cannot be known at compilation time
     |
2788 |     type Item = Result<String>;
     |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<alloc_crate::string::String, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
   --> library/std/src/path.rs:140:30
    |
140 | #[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
    |                              ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/path.rs:473:13
   --> library/std/src/path.rs:473:13
    |
473 |     fn hash<H: Hasher>(&self, h: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
   --> library/std/src/path.rs:500:55
    |
    |
500 | #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    |                                                       ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `&'a ffi::os_str::OsStr` cannot be known at compilation time
    |
    |
863 |     type Item = &'a OsStr;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'a ffi::os_str::OsStr`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `path::Component<'a>` cannot be known at compilation time
    |
    |
884 |     type Item = Component<'a>;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `path::Component<'a>`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a path::Path` cannot be known at compilation time
     |
     |
1090 |     type Item = &'a Path;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a path::Path`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

---

error[E0276]: impl has stricter requirements than trait
    --> library/std/src/path.rs:1669:18
     |
1669 |     fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> PathBuf {
     |                  ^ impl has extra requirement `I: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/path.rs:1678:15
     |
     |
1678 |     fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
     |               ^ impl has extra requirement `I: core::marker::Sized`
error[E0277]: the size for values of type `path::PathBuf` cannot be known at compilation time
    --> library/std/src/path.rs:1809:18
     |
1809 |     type Owned = PathBuf;
1809 |     type Owned = PathBuf;
     |                  ^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `core::marker::Sized` is not implemented for `path::PathBuf`
note: required by a bound in `alloc_crate::borrow::ToOwned::Owned`
    --> /checkout/library/alloc/src/borrow.rs:42:5
     |
42   |     type Owned: Borrow<Self>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `alloc_crate::borrow::ToOwned::Owned`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/path.rs:1830:13
     |
     |
1830 |     fn hash<H: Hasher>(&self, h: &mut H) {
     |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/path.rs:2900:13
     |
     |
2900 |     fn hash<H: Hasher>(&self, h: &mut H) {
     |             ^ impl has extra requirement `H: core::marker::Sized`

error[E0277]: the size for values of type `&'a ffi::os_str::OsStr` cannot be known at compilation time
     |
     |
3018 |     type Item = &'a OsStr;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a ffi::os_str::OsStr`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`

---
     = help: the trait `core::marker::Sized` is not implemented for `path::Iter<'a>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
    --> /checkout/library/core/src/iter/traits/collect.rs:205:5
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `&'a ffi::os_str::OsStr` cannot be known at compilation time
     |
     |
3028 |     type Item = &'a OsStr;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a ffi::os_str::OsStr`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`

---
     = help: the trait `core::marker::Sized` is not implemented for `path::Iter<'a>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
    --> /checkout/library/core/src/iter/traits/collect.rs:205:5
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `&'a ffi::os_str::OsStr` cannot be known at compilation time
     |
     |
1056 |     type Item = &'a OsStr;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a ffi::os_str::OsStr`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
   --> library/std/src/time.rs:128:55
    |
128 | #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                                       ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/time.rs:206:55
   --> library/std/src/time.rs:206:55
    |
206 | #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                                       ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `time::Instant` cannot be known at compilation time
   --> library/std/src/time.rs:402:19
---
    = help: the trait `core::marker::Sized` is not implemented for `time::Instant`
note: required by a bound in `core::ops::Add::Output`
   --> /checkout/library/core/src/ops/arith.rs:78:5
    |
78  |     type Output;
    |     ^^^^^^^^^^^^ required by this bound in `core::ops::Add::Output`
error[E0277]: the size for values of type `time::Instant` cannot be known at compilation time
   --> library/std/src/time.rs:422:19
    |
422 |     type Output = Instant;
422 |     type Output = Instant;
    |                   ^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `time::Instant`
note: required by a bound in `core::ops::Sub::Output`
   --> /checkout/library/core/src/ops/arith.rs:185:5
    |
185 |     type Output;
    |     ^^^^^^^^^^^^ required by this bound in `core::ops::Sub::Output`
error[E0277]: the size for values of type `core::time::Duration` cannot be known at compilation time
   --> library/std/src/time.rs:438:19
    |
438 |     type Output = Duration;
438 |     type Output = Duration;
    |                   ^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::time::Duration`
note: required by a bound in `core::ops::Sub::Output`
   --> /checkout/library/core/src/ops/arith.rs:185:5
    |
185 |     type Output;
    |     ^^^^^^^^^^^^ required by this bound in `core::ops::Sub::Output`
error[E0277]: the size for values of type `time::SystemTime` cannot be known at compilation time
   --> library/std/src/time.rs:568:19
    |
568 |     type Output = SystemTime;
568 |     type Output = SystemTime;
    |                   ^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `time::SystemTime`
note: required by a bound in `core::ops::Add::Output`
   --> /checkout/library/core/src/ops/arith.rs:78:5
    |
78  |     type Output;
    |     ^^^^^^^^^^^^ required by this bound in `core::ops::Add::Output`
error[E0277]: the size for values of type `time::SystemTime` cannot be known at compilation time
   --> library/std/src/time.rs:588:19
    |
588 |     type Output = SystemTime;
588 |     type Output = SystemTime;
    |                   ^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `time::SystemTime`
note: required by a bound in `core::ops::Sub::Output`
   --> /checkout/library/core/src/ops/arith.rs:185:5
    |
185 |     type Output;
    |     ^^^^^^^^^^^^ required by this bound in `core::ops::Sub::Output`

error: duplicate diagnostic item in crate `core`: `mem_drop`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `compiler_fence`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `assert_ne_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `cmp_max`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Display`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `gen_future`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `PartialOrd`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `ref_unwind_safe_trait`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Ordering`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `debug_assert_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `line_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `assert_eq_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_zeroed`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `TryFrom`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_forget`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `ptr_null`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `From`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `iter_repeat`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Hash`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `debug_assert_eq_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `cmp_min`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `DoubleEndedIterator`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Copy`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `deref_target`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `include_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `assert_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `todo_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `unimplemented_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Into`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_replace`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `ptr_null_mut`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `TryInto`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Send`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Sync`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `writeln_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `FromIterator`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `debug_assert_ne_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `unwind_safe_trait`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Duration`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `env_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `atomic_mod`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `matches_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `unreachable_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `noop_method_borrow`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `include_str_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Iterator`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `column_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `file_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_size_of`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `maybe_uninit_uninit`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `format_args_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `needs_drop`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Result`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `core_panic_2021_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Default`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `concat_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `noop_method_clone`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `include_bytes_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `option_env_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `IntoIterator`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `AsMut`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_size_of_val`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `transmute`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `maybe_uninit_zeroed`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `fence`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `core_panic_2015_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Deref`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Debug`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `write_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `assume_init`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `noop_method_deref`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Option`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `compile_error_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `AsRef`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_uninitialized`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Any`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_discriminant`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `core_panic_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Eq`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `stringify_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Pointer`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Clone`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `PartialEq`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Borrow`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `Ord`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `deref_method`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `pointer_trait_fmt`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `module_path_macro`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `AtomicBool`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `mem_variant_count`.
  = note: the diagnostic item is first defined in crate `core`.


error: duplicate diagnostic item in crate `core`: `cfg_macro`.
  = note: the diagnostic item is first defined in crate `core`.

error[E0277]: the size for values of type `()` cannot be known at compilation time
   --> library/std/src/alloc.rs:288:45
   --> library/std/src/alloc.rs:288:45
    |
288 | static HOOK: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());
    |                              -------------- ^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |                              required by a bound introduced by this call
    |
    = help: the trait `core::marker::Sized` is not implemented for `()`
note: required by a bound in `core::sync::atomic::AtomicPtr::<T>::new`
note: required by a bound in `core::sync::atomic::AtomicPtr::<T>::new`
   --> /checkout/library/core/src/sync/atomic.rs:880:6
    |
880 | impl<T> AtomicPtr<T> {
    |      ^ required by this bound in `core::sync::atomic::AtomicPtr::<T>::new`
error[E0277]: the size for values of type `()` cannot be known at compilation time
   --> library/std/src/alloc.rs:288:30
    |
    |
288 | static HOOK: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());
    |
    = help: the trait `core::marker::Sized` is not implemented for `()`
note: required by a bound in `core::sync::atomic::AtomicPtr`
   --> /checkout/library/core/src/sync/atomic.rs:167:22
   --> /checkout/library/core/src/sync/atomic.rs:167:22
    |
167 | pub struct AtomicPtr<T> {
    |                      ^ required by this bound in `core::sync::atomic::AtomicPtr`

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/ffi/c_str.rs:109:42
    |
109 | #[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
    |                                          ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/ffi/c_str.rs:186:10
   --> library/std/src/ffi/c_str.rs:186:10
    |
186 | #[derive(Hash)]
    |          ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0119]: conflicting implementations of trait `ffi::c_str::CString::new::SpecNewImpl` for type `&[u8]`
    |
    |
380 |         impl<T: Into<Vec<u8>>> SpecNewImpl for T {
    |         ---------------------------------------- first implementation here
...
412 |         impl SpecNewImpl for &'_ [u8] {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[u8]`
    |
    = note: upstream crates may add a new impl of trait `core::convert::Into<alloc_crate::vec::Vec<u8>>` for type `&[u8]` in future versions

error[E0119]: conflicting implementations of trait `ffi::c_str::CString::new::SpecNewImpl` for type `&str`
    |
    |
380 |         impl<T: Into<Vec<u8>>> SpecNewImpl for T {
    |         ---------------------------------------- first implementation here
...
418 |         impl SpecNewImpl for &'_ str {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&str`
    |
    = note: upstream crates may add a new impl of trait `core::convert::Into<alloc_crate::vec::Vec<u8>>` for type `&str` in future versions

error[E0119]: conflicting implementations of trait `ffi::c_str::CString::new::SpecNewImpl` for type `&mut [u8]`
    |
    |
380 |         impl<T: Into<Vec<u8>>> SpecNewImpl for T {
    |         ---------------------------------------- first implementation here
...
424 |         impl SpecNewImpl for &'_ mut [u8] {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut [u8]`
    |
    = note: upstream crates may add a new impl of trait `core::convert::Into<alloc_crate::vec::Vec<u8>>` for type `&mut [u8]` in future versions

---

error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1298:15
     |
1298 |     fn extend<T: IntoIterator<Item = OsString>>(&mut self, iter: T) {
     |               ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1308:15
     |
     |
1308 |     fn extend<T: IntoIterator<Item = &'a OsStr>>(&mut self, iter: T) {
     |               ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1318:15
     |
     |
1318 |     fn extend<T: IntoIterator<Item = Cow<'a, OsStr>>>(&mut self, iter: T) {
     |               ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1328:18
     |
     |
1328 |     fn from_iter<I: IntoIterator<Item = OsString>>(iter: I) -> Self {
     |                  ^ impl has extra requirement `I: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1347:18
     |
     |
1347 |     fn from_iter<I: IntoIterator<Item = &'a OsStr>>(iter: I) -> Self {
     |                  ^ impl has extra requirement `I: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/ffi/os_str.rs:1359:18
     |
     |
1359 |     fn from_iter<I: IntoIterator<Item = Cow<'a, OsStr>>>(iter: I) -> Self {
     |                  ^ impl has extra requirement `I: core::marker::Sized`

warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
74 |     default fn copy_to<R: Read + ?Sized>(reader: &mut R, writer: &mut Self) -> Result<u64> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
70 |     fn copy_to<R: Read + ?Sized>(reader: &mut R, writer: &mut Self) -> Result<u64>;


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
73 | impl<W: Write + ?Sized> BufferedCopySpec for W {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
80 |     fn copy_to<R: Read + ?Sized>(reader: &mut R, writer: &mut Self) -> Result<u64> {

error[E0276]: impl has stricter requirements than trait
  --> library/std/src/io/error.rs:91:34
   |
   |
91 | #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
   |                                  ^^^^ impl has extra requirement `__H: core::marker::Sized`
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
16 | impl<R: Read + ?Sized> Read for &mut R {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
85 | impl<S: Seek + ?Sized> Seek for &mut S {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   |
   |
97 | impl<B: BufRead + ?Sized> BufRead for &mut B {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
120 | impl<R: Read + ?Sized> Read for Box<R> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
157 | impl<W: Write + ?Sized> Write for Box<W> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
189 | impl<S: Seek + ?Sized> Seek for Box<S> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
201 | impl<B: BufRead + ?Sized> BufRead for Box<B> {


error[E0277]: the size for values of type `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>` cannot be known at compilation time
    |
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `u8` cannot be known at compilation time
   --> library/std/src/thread/local.rs:329:17
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `u8`
    = help: the trait `core::marker::Sized` is not implemented for `u8`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:16
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/thread/local.rs:329:17
    |
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:75
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                                                                           ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `&'static core::cell::Cell<core::option::Option<alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>>>` cannot be known at compilation time
    |
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'static core::cell::Cell<core::option::Option<alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>>>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>` cannot be known at compilation time
    |
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `u8` cannot be known at compilation time
   --> library/std/src/thread/local.rs:329:17
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `u8`
    = help: the trait `core::marker::Sized` is not implemented for `u8`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:16
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/thread/local.rs:329:17
    |
    |
329 |                   $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:75
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                                                                           ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>` cannot be known at compilation time
    |
312 |                       $crate::thread::__FastLocalKeyInner::new();
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::sync::Arc<sync::mutex::Mutex<alloc_crate::vec::Vec<u8>>>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
---
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `u8`
    = help: the trait `core::marker::Sized` is not implemented for `u8`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:16
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/thread/local.rs:312:21
    |
312 |                       $crate::thread::__FastLocalKeyInner::new();
312 |                       $crate::thread::__FastLocalKeyInner::new();
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
   ::: library/std/src/io/stdio.rs:20:1
    |
20  | / thread_local! {
21  | |     /// Used by the test crate to capture the output of the print macros and panics.
22  | |     static OUTPUT_CAPTURE: Cell<Option<LocalStream>> = {
23  | |         Cell::new(None)
25  | | }
    | |_- in this macro invocation
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
note: required by a bound in `alloc_crate::vec::Vec`
   --> /checkout/library/alloc/src/vec/mod.rs:400:75
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |                                                                           ^ required by this bound in `alloc_crate::vec::Vec`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0276]: impl has stricter requirements than trait
  --> library/std/src/net/addr.rs:41:38
   |
   |
41 | #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
   |                                      ^^^^ impl has extra requirement `__H: core::marker::Sized`
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/net/addr.rs:756:13
   --> library/std/src/net/addr.rs:756:13
    |
756 |     fn hash<H: hash::Hasher>(&self, s: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
   --> library/std/src/net/addr.rs:762:13
    |
    |
762 |     fn hash<H: hash::Hasher>(&self, s: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:900:17
    |
    |
900 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`

warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
     |
     |
1015 | impl<T: ToSocketAddrs + ?Sized> ToSocketAddrs for &T {

error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:908:17
    |
    |
908 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:916:17
    |
    |
916 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:924:17
    |
    |
924 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:936:17
    |
    |
936 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:945:17
    |
    |
945 |     type Iter = option::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::option::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:965:17
    |
    |
965 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/net/addr.rs:965:17
    |
    |
965 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:985:17
    |
    |
985 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/net/addr.rs:985:17
    |
    |
985 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/net/addr.rs:994:17
    |
    |
994 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
   --> library/std/src/net/addr.rs:994:17
    |
    |
994 |     type Iter = vec::IntoIter<SocketAddr>;
    |
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
    = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
    |
    |
885 |     type Iter: Iterator<Item = SocketAddr>;
    |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
    --> library/std/src/net/addr.rs:1007:17
     |
     |
1007 |     type Iter = iter::Cloned<slice::Iter<'a, SocketAddr>>;
     |
     = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
     = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
     = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::iter::Cloned<core::slice::Iter<'a, net::addr::SocketAddr>>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
     |
     |
885  |     type Iter: Iterator<Item = SocketAddr>;
     |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`

error[E0277]: the size for values of type `core::slice::Iter<'a, net::addr::SocketAddr>` cannot be known at compilation time
     |
     |
1007 |     type Iter = iter::Cloned<slice::Iter<'a, SocketAddr>>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::slice::Iter<'a, net::addr::SocketAddr>`
     = note: required because of the requirements on the impl of `core::iter::Iterator` for `core::iter::Cloned<core::slice::Iter<'a, net::addr::SocketAddr>>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
     |
     |
885  |     type Iter: Iterator<Item = SocketAddr>;
     |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
    --> library/std/src/net/addr.rs:1024:17
     |
     |
1024 |     type Iter = vec::IntoIter<SocketAddr>;
     |
     = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
     = help: the trait `core::marker::Sized` is not implemented for `net::addr::SocketAddr`
     = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
     |
     |
885  |     type Iter: Iterator<Item = SocketAddr>;
     |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0277]: the size for values of type `alloc_crate::alloc::Global` cannot be known at compilation time
    --> library/std/src/net/addr.rs:1024:17
     |
     |
1024 |     type Iter = vec::IntoIter<SocketAddr>;
     |
     = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
     = help: the trait `core::marker::Sized` is not implemented for `alloc_crate::alloc::Global`
     = note: required because of the requirements on the impl of `core::iter::Iterator` for `alloc_crate::vec::IntoIter<net::addr::SocketAddr>`
note: required by a bound in `net::addr::ToSocketAddrs::Iter`
     |
     |
885  |     type Iter: Iterator<Item = SocketAddr>;
     |                         ^^^^^^^^^^^^^^^^^ required by this bound in `net::addr::ToSocketAddrs::Iter`
error[E0276]: impl has stricter requirements than trait
  --> library/std/src/net/ip.rs:36:38
   |
   |
36 | #[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
   |                                      ^^^^ impl has extra requirement `__H: core::marker::Sized`
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/net/ip.rs:204:38
   --> library/std/src/net/ip.rs:204:38
    |
204 | #[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
    |                                      ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
    --> library/std/src/net/ip.rs:1079:13
    --> library/std/src/net/ip.rs:1079:13
     |
1079 |     fn hash<H: hash::Hasher>(&self, s: &mut H) {
     |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/net/ip.rs:1900:13
     |
     |
1900 |     fn hash<H: hash::Hasher>(&self, s: &mut H) {
     |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0277]: the size for values of type `net::parser::AddrParseError` cannot be known at compilation time
   --> library/std/src/net/parser.rs:279:16
    |
279 |     type Err = AddrParseError;
---
    |
532 |     type Err;
    |     ^^^^^^^^^ required by this bound in `core::str::FromStr::Err`

error[E0277]: the size for values of type `core::result::Result<net::tcp::TcpStream, io::error::Error>` cannot be known at compilation time
     |
     |
1012 |     type Item = io::Result<TcpStream>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<net::tcp::TcpStream, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `core::result::Result<net::tcp::TcpStream, io::error::Error>` cannot be known at compilation time
     |
     |
1020 |     type Item = io::Result<TcpStream>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<net::tcp::TcpStream, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

---
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `T`
     |
1455 | impl<'a, T: core::marker::Sized> Iterator for Iter<'a, T> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/std/src/sync/mpsc/mod.rs:1465:17
     |
---
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `T`
     |
1464 | impl<'a, T: core::marker::Sized> Iterator for TryIter<'a, T> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/std/src/sync/mpsc/mod.rs:1474:17
     |
---
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`
help: consider restricting type parameter `T`
     |
1473 | impl<'a, T: core::marker::Sized> IntoIterator for &'a Receiver<T> {


error[E0277]: the size for values of type `sync::mpsc::Iter<'a, T>` cannot be known at compilation time
     |
     |
1475 |     type IntoIter = Iter<'a, T>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `sync::mpsc::Iter<'a, T>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
     |
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`
error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/std/src/sync/mpsc/mod.rs:1484:17
     |
1484 |     type Item = T;
---
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `T`
     |
1483 | impl<T: core::marker::Sized> Iterator for IntoIter<T> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/std/src/sync/mpsc/mod.rs:1492:17
     |
---
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`
help: consider restricting type parameter `T`
     |
1491 | impl<T: core::marker::Sized> IntoIterator for Receiver<T> {


error[E0277]: the size for values of type `sync::mpsc::IntoIter<T>` cannot be known at compilation time
     |
1493 |     type IntoIter = IntoIter<T>;
     |                     ^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `sync::mpsc::IntoIter<T>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
    --> /checkout/library/core/src/iter/traits/collect.rs:205:5
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
426 | impl<T: ?Sized + Default> Default for Mutex<T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
434 | impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
466 | impl<T: ?Sized> Deref for MutexGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
475 | impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
482 | impl<T: ?Sized> Drop for MutexGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
493 | impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
500 | impl<T: ?Sized + fmt::Display> fmt::Display for MutexGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
428 | impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLock<T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
102 | pub struct RwLockReadGuard<'a, T: ?Sized + 'a> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
490 | impl<T: ?Sized + fmt::Display> fmt::Display for RwLockReadGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
125 | pub struct RwLockWriteGuard<'a, T: ?Sized + 'a> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
504 | impl<T: ?Sized + fmt::Display> fmt::Display for RwLockWriteGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
511 | impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
520 | impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
529 | impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
536 | impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
    |
    |
545 | impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {

error[E0277]: the size for values of type `libc::c_void` cannot be known at compilation time
   --> library/std/src/sys/windows/c.rs:190:83
    |
    |
190 | pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = CONDITION_VARIABLE { ptr: ptr::null_mut() };
    |                                                                                   ^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `libc::c_void`
note: required by a bound in `core::ptr::null_mut`
   --> /checkout/library/core/src/ptr/mod.rs:232:23
    |
232 | pub const fn null_mut<T>() -> *mut T {
    |                       ^ required by this bound in `core::ptr::null_mut`
error[E0277]: the size for values of type `libc::c_void` cannot be known at compilation time
   --> library/std/src/sys/windows/c.rs:191:50
    |
    |
191 | pub const SRWLOCK_INIT: SRWLOCK = SRWLOCK { ptr: ptr::null_mut() };
    |
    = help: the trait `core::marker::Sized` is not implemented for `libc::c_void`
note: required by a bound in `core::ptr::null_mut`
   --> /checkout/library/core/src/ptr/mod.rs:232:23
   --> /checkout/library/core/src/ptr/mod.rs:232:23
    |
232 | pub const fn null_mut<T>() -> *mut T {
    |                       ^ required by this bound in `core::ptr::null_mut`

error[E0277]: the size for values of type `(&'a ffi::os_str::OsStr, core::option::Option<&'a ffi::os_str::OsStr>)` cannot be known at compilation time
    |
    |
102 |     type Item = (&'a OsStr, Option<&'a OsStr>);
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `(&'a ffi::os_str::OsStr, core::option::Option<&'a ffi::os_str::OsStr>)`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `sys_common::thread_info::ThreadInfo` cannot be known at compilation time
    |
    |
268 |             $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `sys_common::thread_info::ThreadInfo`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `&'static core::cell::RefCell<core::option::Option<sys_common::thread_info::ThreadInfo>>` cannot be known at compilation time
    |
    |
268 |             $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'static core::cell::RefCell<core::option::Option<sys_common::thread_info::ThreadInfo>>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `sys_common::thread_info::ThreadInfo` cannot be known at compilation time
    |
    |
268 |             $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `sys_common::thread_info::ThreadInfo`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `core::option::Option<sys_common::thread_info::ThreadInfo>` cannot be known at compilation time
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |                                                                           ------------ ^^^^ doesn't have a size known at compile-time
    |                                                                           required by a bound introduced by this call
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::option::Option<sys_common::thread_info::ThreadInfo>`
note: required by a bound in `core::cell::RefCell::<T>::new`
    |
    |
700 | impl<T> RefCell<T> {
    |      ^ required by this bound in `core::cell::RefCell::<T>::new`

error[E0277]: the size for values of type `sys_common::thread_info::ThreadInfo` cannot be known at compilation time
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |                                                                           ------------ ^^^^ doesn't have a size known at compile-time
    |                                                                           required by a bound introduced by this call
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `sys_common::thread_info::ThreadInfo`
note: required by a bound in `core::prelude::v1::None`
    |
515 | pub enum Option<T> {
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::prelude::v1::None`

error[E0277]: the size for values of type `sys_common::thread_info::ThreadInfo` cannot be known at compilation time
    |
    |
13  | thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `sys_common::thread_info::ThreadInfo`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`


error[E0277]: the size for values of type `unsafe extern "C" fn(*mut u8)` cannot be known at compilation time
    |
    |
30  |     static DTORS: StaticKey = StaticKey::new(Some(run_dtors));
    |
    = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "C" fn(*mut u8)`
note: required by a bound in `core::option::Option`
   --> /checkout/library/core/src/option.rs:515:17
   --> /checkout/library/core/src/option.rs:515:17
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`

error[E0277]: the size for values of type `unsafe extern "C" fn(*mut u8)` cannot be known at compilation time
    |
    |
30  |     static DTORS: StaticKey = StaticKey::new(Some(run_dtors));
    |                                              ---- ^^^^^^^^^ doesn't have a size known at compile-time
    |                                              required by a bound introduced by this call
    |
    = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "C" fn(*mut u8)`
    = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "C" fn(*mut u8)`
note: required by a bound in `core::prelude::v1::Some`
    |
515 | pub enum Option<T> {
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::prelude::v1::Some`

error[E0277]: the size for values of type `unsafe extern "C" fn(*mut u8)` cannot be known at compilation time
    |
    |
117 | pub const INIT: StaticKey = StaticKey::new(None);
    |
    = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "C" fn(*mut u8)`
note: required by a bound in `core::option::Option`
   --> /checkout/library/core/src/option.rs:515:17
   --> /checkout/library/core/src/option.rs:515:17
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys_common/wtf8.rs:409:18
    |
409 |     fn from_iter<T: IntoIterator<Item = CodePoint>>(iter: T) -> Wtf8Buf {
    |                  ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys_common/wtf8.rs:421:15
    |
    |
421 |     fn extend<T: IntoIterator<Item = CodePoint>>(&mut self, iter: T) {
    |               ^ impl has extra requirement `T: core::marker::Sized`

error[E0277]: the size for values of type `sys_common::wtf8::CodePoint` cannot be known at compilation time
   --> library/std/src/sys_common/wtf8.rs:845:17
845 |     type Item = CodePoint;
    |                 ^^^^^^^^^ doesn't have a size known at compile-time
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `sys_common::wtf8::CodePoint`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

---

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys_common/wtf8.rs:904:13
    |
904 |     fn hash<H: Hasher>(&self, state: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys_common/wtf8.rs:911:13
    |
    |
911 |     fn hash<H: Hasher>(&self, state: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys_common/wtf8.rs:919:13
    |
    |
919 |     fn hash<H: Hasher>(&self, state: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`
error[E0277]: the size for values of type `net::addr::SocketAddr` cannot be known at compilation time
   --> library/std/src/sys_common/net.rs:136:17
    |
136 |     type Item = SocketAddr;
---
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

error[E0277]: the size for values of type `io::error::Error` cannot be known at compilation time
    |
161 |     type Error = io::Error;
    |                  ^^^^^^^^^ doesn't have a size known at compile-time
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `io::error::Error`
note: required by a bound in `core::convert::TryFrom::Error`
   --> /checkout/library/core/src/convert/mod.rs:471:5
    |
471 |     type Error;
    |     ^^^^^^^^^^^ required by this bound in `core::convert::TryFrom::Error`

error[E0277]: the size for values of type `io::error::Error` cannot be known at compilation time
    |
181 |     type Error = io::Error;
    |                  ^^^^^^^^^ doesn't have a size known at compile-time
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `io::error::Error`
note: required by a bound in `core::convert::TryFrom::Error`
   --> /checkout/library/core/src/convert/mod.rs:471:5
    |
471 |     type Error;
    |     ^^^^^^^^^^^ required by this bound in `core::convert::TryFrom::Error`

error[E0277]: the size for values of type `&'static core::cell::Cell<usize>` cannot be known at compilation time
    |
    |
329 |                 $crate::thread::LocalKey::new(__getit)
    |
   ::: library/std/src/panicking.rs:321:5
    |
    |
321 |     thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0) }
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'static core::cell::Cell<usize>`
note: required by a bound in `core::option::Option`
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::option::Option`
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `usize` cannot be known at compilation time
   --> library/std/src/../../backtrace/src/print.rs:7:55
    |
7   | const HEX_WIDTH: usize = 2 + 2 * core::mem::size_of::<usize>();
    |
    = help: the trait `core::marker::Sized` is not implemented for `usize`
note: required by a bound in `core::mem::size_of`
   --> /checkout/library/core/src/mem/mod.rs:304:22
   --> /checkout/library/core/src/mem/mod.rs:304:22
    |
304 | pub const fn size_of<T>() -> usize {
    |                      ^ required by this bound in `core::mem::size_of`

warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
     |
     |
1149 | impl<K, Q: ?Sized, V, S> Index<&Q> for HashMap<K, V, S>


error[E0277]: the size for values of type `(&'a K, &'a V)` cannot be known at compilation time
     |
     |
1989 |     type Item = (&'a K, &'a V);
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(&'a K, &'a V)`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`


error[E0277]: the size for values of type `collections::hash::map::Iter<'a, K, V>` cannot be known at compilation time
     |
     |
1990 |     type IntoIter = Iter<'a, K, V>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::map::Iter<'a, K, V>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
     |
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `(&'a K, &'a mut V)` cannot be known at compilation time
     |
     |
2000 |     type Item = (&'a K, &'a mut V);
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(&'a K, &'a mut V)`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`


error[E0277]: the size for values of type `collections::hash::map::IterMut<'a, K, V>` cannot be known at compilation time
     |
     |
2001 |     type IntoIter = IterMut<'a, K, V>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::map::IterMut<'a, K, V>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
     |
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `(K, V)` cannot be known at compilation time
     |
2011 |     type Item = (K, V);
     |                 ^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(K, V)`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`


error[E0277]: the size for values of type `collections::hash::map::IntoIter<K, V>` cannot be known at compilation time
     |
2012 |     type IntoIter = IntoIter<K, V>;
     |                     ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::map::IntoIter<K, V>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
     |
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `(&'a K, &'a V)` cannot be known at compilation time
     |
     |
2040 |     type Item = (&'a K, &'a V);
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(&'a K, &'a V)`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `(&'a K, &'a mut V)` cannot be known at compilation time
     |
     |
2064 |     type Item = (&'a K, &'a mut V);
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(&'a K, &'a mut V)`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `(K, V)` cannot be known at compilation time
     |
2098 |     type Item = (K, V);
     |                 ^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(K, V)`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a K` cannot be known at compilation time
     |
     |
2128 |     type Item = &'a K;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a K`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a V` cannot be known at compilation time
     |
     |
2151 |     type Item = &'a V;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a V`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a mut V` cannot be known at compilation time
     |
2174 |     type Item = &'a mut V;
     |                 ^^^^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a mut V`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `K` cannot be known at compilation time
     |
2204 |     type Item = K;
     |                 ^ doesn't have a size known at compile-time
     |
     |
note: required by a bound in `core::iter::Iterator::Item`
    --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `K`
     |
2203 | impl<K: core::marker::Sized, V> Iterator for IntoKeys<K, V> {

error[E0277]: the size for values of type `V` cannot be known at compilation time
    --> library/std/src/collections/hash/map.rs:2234:17
     |
---
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `V`
     |
2233 | impl<K, V: core::marker::Sized> Iterator for IntoValues<K, V> {


error[E0277]: the size for values of type `(K, V)` cannot be known at compilation time
     |
2264 |     type Item = (K, V);
     |                 ^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(K, V)`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `(K, V)` cannot be known at compilation time
     |
2301 |     type Item = (K, V);
     |                 ^^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `(K, V)`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
    --> library/std/src/collections/hash/map.rs:2831:18
     |
2831 |     fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> HashMap<K, V, S> {
     |                  ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/collections/hash/map.rs:2847:15
     |
     |
2847 |     fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
     |               ^ impl has extra requirement `T: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/collections/hash/map.rs:2870:15
     |
     |
2870 |     fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
     |               ^ impl has extra requirement `T: core::marker::Sized`
error[E0277]: the size for values of type `&'static core::cell::Cell<(u64, u64)>` cannot be known at compilation time
    --> library/std/src/thread/local.rs:329:47
     |
     |
329  |                   $crate::thread::LocalKey::new(__getit)
     |
    ::: library/std/src/collections/hash/map.rs:2935:9
     |
     |
2935 | /         thread_local!(static KEYS: Cell<(u64, u64)> = {
2936 | |             Cell::new(sys::hashmap_random_keys())
     | |__________- in this macro invocation
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'static core::cell::Cell<(u64, u64)>`
note: required by a bound in `core::option::Option`
note: required by a bound in `core::option::Option`
    --> /checkout/library/core/src/option.rs:515:17
     |
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::option::Option`
     = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `collections::hash::map::DefaultHasher` cannot be known at compilation time
     |
     |
2949 |     type Hasher = DefaultHasher;
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::map::DefaultHasher`
note: required by a bound in `core::hash::BuildHasher::Hasher`
    --> /checkout/library/core/src/hash/mod.rs:492:5
    --> /checkout/library/core/src/hash/mod.rs:492:5
     |
492  |     type Hasher: Hasher;
     |     ^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::hash::BuildHasher::Hasher`

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/collections/hash/set.rs:990:18
    |
990 |     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> HashSet<T, S> {
    |                  ^ impl has extra requirement `I: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/collections/hash/set.rs:1035:15
     |
     |
1035 |     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
     |               ^ impl has extra requirement `I: core::marker::Sized`
error[E0276]: impl has stricter requirements than trait
    --> library/std/src/collections/hash/set.rs:1057:15
     |
     |
1057 |     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
     |               ^ impl has extra requirement `I: core::marker::Sized`
error[E0277]: the size for values of type `collections::hash::set::HashSet<T, S>` cannot be known at compilation time
    --> library/std/src/collections/hash/set.rs:1090:19
     |
1090 |     type Output = HashSet<T, S>;
---
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::set::HashSet<T, S>`
note: required by a bound in `core::ops::Sub::Output`
    --> /checkout/library/core/src/ops/arith.rs:185:5
     |
185  |     type Output;
     |     ^^^^^^^^^^^^ required by this bound in `core::ops::Sub::Output`

error[E0277]: the size for values of type `&'a T` cannot be known at compilation time
     |
1409 |     type Item = &'a T;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a T`
note: required by a bound in `core::iter::IntoIterator::Item`
     |
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`


error[E0277]: the size for values of type `collections::hash::set::Iter<'a, T>` cannot be known at compilation time
     |
     |
1410 |     type IntoIter = Iter<'a, T>;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::set::Iter<'a, T>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
     |
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`
error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/std/src/collections/hash/set.rs:1420:17
     |
1420 |     type Item = T;
---
201  |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`
help: consider restricting type parameter `T`
     |
1419 | impl<T: core::marker::Sized, S> IntoIterator for HashSet<T, S> {

error[E0277]: the size for values of type `collections::hash::set::IntoIter<T>` cannot be known at compilation time
    --> library/std/src/collections/hash/set.rs:1421:21
     |
     |
1421 |     type IntoIter = IntoIter<T>;
     |                     ^^^^^^^^^^^ doesn't have a size known at compile-time
     |
     = help: the trait `core::marker::Sized` is not implemented for `collections::hash::set::IntoIter<T>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
    --> /checkout/library/core/src/iter/traits/collect.rs:205:5
     |
205  |     type IntoIter: Iterator<Item = Self::Item>;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `&'a K` cannot be known at compilation time
     |
     |
1458 |     type Item = &'a K;
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a K`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `K` cannot be known at compilation time
     |
1488 |     type Item = K;
     |                 ^ doesn't have a size known at compile-time
     |
     |
note: required by a bound in `core::iter::Iterator::Item`
    --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `K`
     |
1487 | impl<K: core::marker::Sized> Iterator for IntoIter<K> {


error[E0277]: the size for values of type `K` cannot be known at compilation time
     |
1518 |     type Item = K;
     |                 ^ doesn't have a size known at compile-time
     |
     |
note: required by a bound in `core::iter::Iterator::Item`
    --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `K`
     |
1517 | impl<'a, K: core::marker::Sized> Iterator for Drain<'a, K> {


error[E0277]: the size for values of type `K` cannot be known at compilation time
     |
1551 |     type Item = K;
     |                 ^ doesn't have a size known at compile-time
     |
     |
note: required by a bound in `core::iter::Iterator::Item`
    --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider further restricting type parameter `K`
     |
1549 |     F: FnMut(&K) -> bool, K: core::marker::Sized


error[E0277]: the size for values of type `&'a T` cannot be known at compilation time
     |
1590 |     type Item = &'a T;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a T`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a T` cannot be known at compilation time
     |
1642 |     type Item = &'a T;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a T`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a T` cannot be known at compilation time
     |
1694 |     type Item = &'a T;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a T`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `&'a T` cannot be known at compilation time
     |
1758 |     type Item = &'a T;
     |                 ^^^^^ doesn't have a size known at compile-time
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `&'a T`
note: required by a bound in `core::iter::Iterator::Item`
     |
58   |     type Item;
     |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
  --> library/std/src/os/unix/ucred.rs:13:34
   |
13 | #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
   |                                  ^^^^ impl has extra requirement `__H: core::marker::Sized`
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
 --> library/std/src/sync/mpsc/cache_aligned.rs:3:64
 --> library/std/src/sync/mpsc/cache_aligned.rs:3:64
  |
3 | #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
  |                                                                ^^^^ impl has extra requirement `__H: core::marker::Sized`
  = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `ffi::os_str::OsString` cannot be known at compilation time
   |
   |
36 |     type Item = OsString;
   |
   = help: the trait `core::marker::Sized` is not implemented for `ffi::os_str::OsString`
note: required by a bound in `core::iter::Iterator::Item`
  --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
  --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
   |
58 |     type Item;
   |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys/unix/fs.rs:253:38
    |
253 | #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |                                      ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the size for values of type `core::result::Result<sys::unix::fs::DirEntry, io::error::Error>` cannot be known at compilation time
    |
449 |     type Item = io::Result<DirEntry>;
    |                 ^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<sys::unix::fs::DirEntry, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   --> library/std/src/sys/unix/kernel_copy.rs:142:6
    |
142 | impl<R: Read + ?Sized, W: Write + ?Sized> SpecCopy for Copier<'_, '_, R, W> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   --> library/std/src/sys/unix/kernel_copy.rs:142:24
    |
142 | impl<R: Read + ?Sized, W: Write + ?Sized> SpecCopy for Copier<'_, '_, R, W> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   --> library/std/src/sys/unix/kernel_copy.rs:133:23
    |
133 | struct Copier<'a, 'b, R: Read + ?Sized, W: Write + ?Sized> {


warning: default bound relaxed for a type parameter, but this does nothing because the given bound is not a default; only `?Sized` is supported
   --> library/std/src/sys/unix/kernel_copy.rs:133:41
    |
133 | struct Copier<'a, 'b, R: Read + ?Sized, W: Write + ?Sized> {

error[E0277]: the size for values of type `path::PathBuf` cannot be known at compilation time
   --> library/std/src/sys/unix/os.rs:199:17
    |
---
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

error[E0277]: the size for values of type `(ffi::os_str::OsString, ffi::os_str::OsString)` cannot be known at compilation time
   --> library/std/src/sys/unix/os.rs:465:17
    |
465 |     type Item = (OsString, OsString);
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `(ffi::os_str::OsString, ffi::os_str::OsString)`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0276]: impl has stricter requirements than trait
  --> library/std/src/sys/unix/os_str.rs:20:10
   |
20 | #[derive(Hash)]
   |          ^^^^ impl has extra requirement `__H: core::marker::Sized`
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys/unix/time.rs:111:13
   --> library/std/src/sys/unix/time.rs:111:13
    |
111 |     fn hash<H: Hasher>(&self, state: &mut H) {
    |             ^ impl has extra requirement `H: core::marker::Sized`

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut libc::c_void, *const u16) -> i64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1186:1
     |
1186 | / compat_fn! {
1187 | |     "kernel32":
1188 | |
1189 | |     // >= Win10 1607
1207 | |     }
1208 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut libc::c_void, *const u16) -> i64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut sys::c::FILETIME)` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1186:1
     |
1186 | / compat_fn! {
1187 | |     "kernel32":
1188 | |
1189 | |     // >= Win10 1607
1207 | |     }
1208 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut sys::c::FILETIME)`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(u64, *const u16) -> u64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1186:1
     |
1186 | / compat_fn! {
1187 | |     "kernel32":
1188 | |
1189 | |     // >= Win10 1607
1207 | |     }
1208 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(u64, *const u16) -> u64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut libc::c_void, *mut libc::c_void, usize, u64) -> i32` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1210:1
     |
1210 | / compat_fn! {
1211 | |     "api-ms-win-core-synch-l1-2-0":
1212 | |
1213 | |     // >= Windows 8 / Server 2012
1226 | |     }
1227 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut libc::c_void, *mut libc::c_void, usize, u64) -> i32`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut libc::c_void)` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1210:1
     |
1210 | / compat_fn! {
1211 | |     "api-ms-win-core-synch-l1-2-0":
1212 | |
1213 | |     // >= Windows 8 / Server 2012
1226 | |     }
1227 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut libc::c_void)`
note: required by a bound in `core::prelude::v1::None`
    --> /checkout/library/core/src/option.rs:515:17
     |
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut *mut libc::c_void, u64, *const sys::c::OBJECT_ATTRIBUTES, *mut sys::c::IO_STATUS_BLOCK, u64, u64) -> i64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1229:1
     |
1229 | / compat_fn! {
1230 | |     "ntdll":
1231 | |     pub fn NtOpenFile(
1232 | |         FileHandle: *mut HANDLE,
1269 | |     }
1270 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut *mut libc::c_void, u64, *const sys::c::OBJECT_ATTRIBUTES, *mut sys::c::IO_STATUS_BLOCK, u64, u64) -> i64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(i64) -> u64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1229:1
     |
1229 | / compat_fn! {
1230 | |     "ntdll":
1231 | |     pub fn NtOpenFile(
1232 | |         FileHandle: *mut HANDLE,
1269 | |     }
1270 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(i64) -> u64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut *mut libc::c_void, u64, *mut libc::c_void, u64) -> i64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1229:1
     |
1229 | / compat_fn! {
1230 | |     "ntdll":
1231 | |     pub fn NtOpenFile(
1232 | |         FileHandle: *mut HANDLE,
1269 | |     }
1270 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut *mut libc::c_void, u64, *mut libc::c_void, u64) -> i64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `unsafe extern "system" fn(*mut libc::c_void, *mut libc::c_void, u8, *mut i64) -> i64` cannot be known at compilation time
    --> library/std/src/sys/windows/compat.rs:70:41
     |
70   |               static mut PTR: Option<F> = None;
     |
     |
    ::: library/std/src/sys/windows/c.rs:1229:1
     |
1229 | / compat_fn! {
1230 | |     "ntdll":
1231 | |     pub fn NtOpenFile(
1232 | |         FileHandle: *mut HANDLE,
1269 | |     }
1270 | | }
     | |_- in this macro invocation
     |
     |
     = help: the trait `core::marker::Sized` is not implemented for `unsafe extern "system" fn(*mut libc::c_void, *mut libc::c_void, u8, *mut i64) -> i64`
note: required by a bound in `core::prelude::v1::None`
     |
515  | pub enum Option<T> {
515  | pub enum Option<T> {
     |                 ^ required by this bound in `core::prelude::v1::None`
     = note: this error originates in the macro `compat_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the size for values of type `backtrace_rs::symbolize::gimli::Cache` cannot be known at compilation time
    |
    |
264 |         static mut MAPPINGS_CACHE: Option<Cache> = None;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `backtrace_rs::symbolize::gimli::Cache`
note: required by a bound in `core::prelude::v1::None`
    |
515 | pub enum Option<T> {
515 | pub enum Option<T> {
    |                 ^ required by this bound in `core::prelude::v1::None`
error[E0277]: the size for values of type `T` cannot be known at compilation time
   --> library/std/src/os/unix/net/ancillary.rs:165:17
    |
165 |     type Item = T;
---
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`
help: consider restricting type parameter `T`
    |
164 | impl<'a, T: core::marker::Sized> Iterator for AncillaryDataIter<'a, T> {

error[E0277]: the size for values of type `i32` cannot be known at compilation time
   --> library/std/src/os/unix/net/ancillary.rs:245:17
    |
    |
245 |     type Item = RawFd;
    |
    = help: the trait `core::marker::Sized` is not implemented for `i32`
note: required by a bound in `core::iter::Iterator::Item`
   --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
   --> /checkout/library/core/src/iter/traits/iterator.rs:58:5
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`

error[E0277]: the size for values of type `os::unix::net::ancillary::SocketCred` cannot be known at compilation time
    |
    |
262 |     type Item = SocketCred;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `os::unix::net::ancillary::SocketCred`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `core::result::Result<os::unix::net::ancillary::AncillaryData<'a>, os::unix::net::ancillary::AncillaryError>` cannot be known at compilation time
    |
    |
345 |     type Item = Result<AncillaryData<'a>, AncillaryError>;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<os::unix::net::ancillary::AncillaryData<'a>, os::unix::net::ancillary::AncillaryError>`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `core::result::Result<os::unix::net::stream::UnixStream, io::error::Error>` cannot be known at compilation time
    |
    |
329 |     type Item = io::Result<UnixStream>;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<os::unix::net::stream::UnixStream, io::error::Error>`
note: required by a bound in `core::iter::IntoIterator::Item`
    |
201 |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::Item`


error[E0277]: the size for values of type `os::unix::net::listener::Incoming<'a>` cannot be known at compilation time
    |
    |
330 |     type IntoIter = Incoming<'a>;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `os::unix::net::listener::Incoming<'a>`
note: required by a bound in `core::iter::IntoIterator::IntoIter`
    |
    |
205 |     type IntoIter: Iterator<Item = Self::Item>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::iter::IntoIterator::IntoIter`

error[E0277]: the size for values of type `core::result::Result<os::unix::net::stream::UnixStream, io::error::Error>` cannot be known at compilation time
    |
    |
376 |     type Item = io::Result<UnixStream>;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `core::result::Result<os::unix::net::stream::UnixStream, io::error::Error>`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `()` cannot be known at compilation time
   --> library/std/src/os/windows/io/handle.rs:138:18
138 |     type Error = ();
    |                  ^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `()`
    = help: the trait `core::marker::Sized` is not implemented for `()`
note: required by a bound in `core::convert::TryFrom::Error`
   --> /checkout/library/core/src/convert/mod.rs:471:5
    |
471 |     type Error;
    |     ^^^^^^^^^^^ required by this bound in `core::convert::TryFrom::Error`

error[E0277]: the size for values of type `()` cannot be known at compilation time
   --> library/std/src/os/windows/io/handle.rs:148:18
148 |     type Error = ();
    |                  ^^ doesn't have a size known at compile-time
    |
    = help: the trait `core::marker::Sized` is not implemented for `()`
    = help: the trait `core::marker::Sized` is not implemented for `()`
note: required by a bound in `core::convert::TryFrom::Error`
   --> /checkout/library/core/src/convert/mod.rs:471:5
    |
471 |     type Error;
    |     ^^^^^^^^^^^ required by this bound in `core::convert::TryFrom::Error`

error[E0277]: the size for values of type `*const u8` cannot be known at compilation time
    |
    |
87  |     static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(ptr::null_mut());
    |                                         -------------- ^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |                                         required by a bound introduced by this call
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `*const u8`
note: required by a bound in `core::sync::atomic::AtomicPtr::<T>::new`
    |
    |
880 | impl<T> AtomicPtr<T> {
    |      ^ required by this bound in `core::sync::atomic::AtomicPtr::<T>::new`

error[E0277]: the size for values of type `*const u8` cannot be known at compilation time
    |
    |
87  |     static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(ptr::null_mut());
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `*const u8`
note: required by a bound in `core::sync::atomic::AtomicPtr`
    |
167 | pub struct AtomicPtr<T> {
    |                      ^ required by this bound in `core::sync::atomic::AtomicPtr`


error[E0277]: the size for values of type `&'a ffi::os_str::OsStr` cannot be known at compilation time
    |
    |
484 |     type Item = &'a OsStr;
    |
    |
    = help: the trait `core::marker::Sized` is not implemented for `&'a ffi::os_str::OsStr`
note: required by a bound in `core::iter::Iterator::Item`
    |
58  |     type Item;
    |     ^^^^^^^^^^ required by this bound in `core::iter::Iterator::Item`


error[E0277]: the size for values of type `libc::c_void` cannot be known at compilation time
   --> library/std/src/sys/unix/stack_overflow.rs:120:68
    |
120 |     static MAIN_ALTSTACK: AtomicPtr<libc::c_void> = AtomicPtr::new(ptr::null_mut());
    |                                                     -------------- ^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |                                                     required by a bound introduced by this call
    |
    = help: the trait `core::marker::Sized` is not implemented for `libc::c_void`
note: required by a bound in `core::sync::atomic::AtomicPtr::<T>::new`
note: required by a bound in `core::sync::atomic::AtomicPtr::<T>::new`
   --> /checkout/library/core/src/sync/atomic.rs:880:6
    |
880 | impl<T> AtomicPtr<T> {
    |      ^ required by this bound in `core::sync::atomic::AtomicPtr::<T>::new`
error[E0277]: the size for values of type `libc::c_void` cannot be known at compilation time
   --> library/std/src/sys/unix/stack_overflow.rs:120:53
    |
    |
120 |     static MAIN_ALTSTACK: AtomicPtr<libc::c_void> = AtomicPtr::new(ptr::null_mut());
    |
    = help: the trait `core::marker::Sized` is not implemented for `libc::c_void`
note: required by a bound in `core::sync::atomic::AtomicPtr`
   --> /checkout/library/core/src/sync/atomic.rs:167:22
   --> /checkout/library/core/src/sync/atomic.rs:167:22
    |
167 | pub struct AtomicPtr<T> {
    |                      ^ required by this bound in `core::sync::atomic::AtomicPtr`

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys/unix/time.rs:282:59
    |
282 |     #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                                           ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0276]: impl has stricter requirements than trait
   --> library/std/src/sys/unix/time.rs:287:59
   --> library/std/src/sys/unix/time.rs:287:59
    |
287 |     #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                                           ^^^^ impl has extra requirement `__H: core::marker::Sized`
    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0119, E0276, E0277.
For more information about an error, try `rustc --explain E0119`.
For more information about an error, try `rustc --explain E0119`.
warning: `std` (lib doc) generated 44 warnings

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=a8f309827038c5ab -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-6c10aa8faa8d510a.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-7a686409217c739d.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-e755a7b36c28bcd7.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-72c436c2eb83779d.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-07ea62845071a1d9.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-4e8740ea3977ce10.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-ec07c66ec37364d4.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-77d18573c44dfbdb.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libobject-7365bf845372726c.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-efad7dcc582999eb.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_detect-8be572abe3cab69c.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (d3db80372
  2022-01-21)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.60.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:00:35
