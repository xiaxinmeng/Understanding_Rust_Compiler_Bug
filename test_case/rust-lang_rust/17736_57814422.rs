
task '<main>' failed at 'called `Option::unwrap()` on a `None` value', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libcore/option.rs:278
stack backtrace:
   1:        0x105c62029 - rt::backtrace::imp::write::h5e81d8e6d0d10e01RPq
   2:        0x105c65555 - failure::on_fail::h68caae8756025d31v6q
   3:        0x105eed845 - unwind::begin_unwind_inner::h7bb0299ab4bd6f7cPQd
   4:        0x105eed503 - unwind::begin_unwind_fmt::hcfef2e4d8e502ed7hOd
   5:        0x105eed212 - rust_begin_unwind
   6:        0x105f4165c - failure::begin_unwind::hc194be7392de8abeank
   7:        0x105f3fe90 - atomic::AtomicBool::store::hbd4446643535e8ecSpj
   8:        0x1023a080e - html::markdown::render::header::hedb865dc06ef23a5Gwl
   9:        0x1024be22f - parse_block
  10:        0x1024bd2e4 - hoedown_document_render
  11:        0x10239eae6 - html::markdown::render::hae163b8349037f8cZnl
  12:        0x1023a86d2 - html::markdown::Markdown<'a>.fmt..Show::fmt::hcbaaf55fd0b67af6DWl
  13:        0x10228401e - fmt::secret_show::h7556523823418219885
  14:        0x105f4ad2f - fmt::write::hdb7f07cec2399131lQy
  15:        0x105f4ab4c - fmt::Arguments<'a>.Show::fmt::h5160d95c5f4620085Hy
  16:        0x105f4ad2f - fmt::write::hdb7f07cec2399131lQy
  17:        0x105bffeba - fmt::format::h7b71c05068bcfa34PBq
  18:        0x102283db2 - main::h75e2f69b8b19e349gaa
  19:        0x1022c0aba - start::closure.8595
  20:        0x105f5162c - rust_try_inner
  21:        0x105f51616 - rust_try
  22:        0x105eeabf7 - unwind::try::heb45a9f3ad06986fxFd
  23:        0x105eeaa6c - task::Task::run::hcd92ee740e9bb36dMVc
  24:        0x1022c090e - start::h9fa01a403bf2217eake
  25:        0x1022c0759 - lang_start::h83801431a7a0e072tje
  26:        0x102283f2f - main
