plain
test src/sync.rs - sync::Weak<T>::upgrade (line 1969) ... ok
test src/sync.rs - sync::Weak<T>::ptr_eq (line 2100) ... ok
test src/sync.rs - sync::Weak<T>::into_raw (line 1875) ... ok
test src/vec/drain.rs - vec::drain::Drain (line 17) ... ok
test src/vec/drain.rs - vec::drain::Drain<'a,T,A>::as_slice (line 48) ... ok
test src/vec/in_place_collect.rs - vec::in_place_collect (line 100) ... ok
test src/vec/in_place_collect.rs - vec::in_place_collect (line 111) ... ok
test src/vec/drain_filter.rs - vec::drain_filter::DrainFilter (line 15) ... ok
test src/vec/drain_filter.rs - vec::drain_filter::DrainFilter (line 15) ... ok
test src/vec/drain.rs - vec::drain::Drain<'a,T,A>::keep_rest (line 73) ... ok
test src/vec/into_iter.rs - vec::into_iter::IntoIter (line 24) ... ok
test src/vec/drain_filter.rs - vec::drain_filter::DrainFilter<'_,T,F,A>::keep_rest (line 63) ... ok
test src/vec/into_iter.rs - vec::into_iter::IntoIter<T,A>::forget_allocation_drop_remaining (line 103) ... ok
test src/vec/into_iter.rs - vec::into_iter::IntoIter<T,A>::as_mut_slice (line 72) ... ok
test src/vec/into_iter.rs - vec::into_iter::IntoIter<T,A>::as_slice (line 56) ... ok
test src/vec/in_place_collect.rs - vec::in_place_collect (line 121) ... ok
---
test src/slice/iter.rs - slice::iter::SplitInclusive (line 528) ... ok
test src/slice/iter.rs - slice::iter::Windows (line 1288) ... ok
test src/slice/iter.rs - slice::iter::SplitInclusiveMut (line 778) ... ok
test src/slice/iter.rs - slice::iter::SplitMut (line 649) ... ok
test src/slice/iter.rs - slice::iter::Split<'a,T,P>::as_slice (line 411) ... ok
test src/slice/iter.rs - slice::iter::SplitNMut (line 1203) ... ok
test src/slice/iter.rs - slice::iter::SplitN (line 1122) ... ok
test src/slice/mod.rs - slice::[T]::align_to_mut (line 3521) ... ok
test src/slice/mod.rs - slice::[T]::array_chunks (line 1065) ... ok
---
test src/time.rs - time::SystemTimeError (line 247) - compile ... ok
test src/time.rs - time (line 7) ... ok
test src/thread/mod.rs - thread::spawn (line 686) ... ok
test src/time.rs - time::Instant::now (line 268) ... ok
test src/thread/scoped.rs - thread::scoped::ScopedJoinHandle<'scope,T>::join (line 296) ... ok
test src/thread/scoped.rs - thread::scoped::ScopedJoinHandle<'scope,T>::thread (line 264) ... ok
test src/thread/scoped.rs - thread::scoped::Builder::spawn_scoped (line 212) ... ok
test src/thread/scoped.rs - thread::scoped::scope (line 84) ... ok
test src/sync/mpsc/mod.rs - sync::mpsc::TryIter (line 401) ... ok
test src/thread/mod.rs - thread::spawn (line 662) ... ok
---
   Doc-tests rustc_hir

running 11 tests
test src/hir.rs - hir::LocalSource::AsyncFn (line 2062) ... ignored
test src/hir.rs - hir::Node<'hir>::ident (line 3387) ... ignored
test src/hir.rs - hir::ParamName::Fresh (line 57) ... ignored
test src/hir.rs - hir::PatKind::Slice (line 1149) ... ignored
test src/hir.rs - hir::TypeBinding (line 2333) ... ignored
test src/def.rs - def::Res::SelfTyAlias (line 332) ... ok
---
   |
20 | //! std::atomic<T> API). It is therefore possible for this implementation to generate behaviours never observable when the
   |                ^^^
   |
   = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`
   |
20 | //! `std::atomic<T>` API). It is therefore possible for this implementation to generate behaviours never observable when the
   |     +              +


error: could not document `miri`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name miri src/tools/miri/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc --cfg 'feature="default"' --cfg 'feature="stack-cache"' -Zunstable-options --check-cfg 'values(feature, "default", "stack-cache")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=3c1e4fbd35df40d2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-751014f43673acfb.rmeta --extern getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libgetrandom-d1fce1ad6522dbd1.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-2a3ac2e2c89c9066.rmeta --extern libffi=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibffi-518a66794a5a25c8.rmeta --extern libloading=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblibloading-3ded268c83d85657.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblog-53345ef8ca6ba290.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libmeasureme-97ba12c5b4eb21de.rmeta --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librand-16050e492f4c59db.rmeta --extern rustc_workspace_hack=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_workspace_hack-d766480c5c7416fe.rmeta --extern shell_escape=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libshell_escape-c54c09e164d2b96c.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-a03264754a00f2e4.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0
  (9da13091b
Build completed unsuccessfully in 0:28:16
