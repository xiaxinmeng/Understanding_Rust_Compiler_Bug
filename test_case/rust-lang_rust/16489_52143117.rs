
  thread #2: tid = 0x0001, 0x0000000110bf8450 librbml-4e7c5e5c.dylib`reader::maybe_get_doc::h49f0a6e62ed2aa2eMMa + 192, stop reason = signal SIGSTOP
    frame #0: 0x0000000110bf8450 librbml-4e7c5e5c.dylib`reader::maybe_get_doc::h49f0a6e62ed2aa2eMMa + 192
    frame #1: 0x00000000017fb0d4
    frame #2: 0x000000010ea5a4ae librustc-4e7c5e5c.dylib`metadata::creader::PluginMetadataReader$LT$$x27a$GT$::read_plugin_metadata::h8a67b9643df81f1enCt + 2174
    frame #3: 0x000000010eb152d3 librustc-4e7c5e5c.dylib`plugin::load::PluginLoader$LT$$x27a$GT$.Visitor$LT$$LP$$RP$$GT$::visit_view_item::h63e14bd2cb2a16fdF4A + 611
    frame #4: 0x000000010eaab91f librustc-4e7c5e5c.dylib`plugin::load::load_plugins::hc01fc2451d60e1b0r3A + 319
    frame #5: 0x000000010e8f5b7d librustc-4e7c5e5c.dylib`driver::driver::phase_2_configure_and_expand::hc69692654be8f218WBw + 1981
    frame #6: 0x000000010ea6d9ff librustc-4e7c5e5c.dylib`driver::driver::compile_input::h558ac2baf30310beuvw + 943
    frame #7: 0x000000010eb01d62 librustc-4e7c5e5c.dylib`driver::main_args::closure.136228 + 6082
    frame #8: 0x000000010eb0e9cb librustc-4e7c5e5c.dylib`task::TaskBuilder$LT$S$GT$::try_future::closure.137382 + 75
    frame #9: 0x000000010eb0e8f7 librustc-4e7c5e5c.dylib`task::TaskBuilder$LT$S$GT$::spawn_internal::closure.137346 + 215
    frame #10: 0x000000010e2ec53a libnative-4e7c5e5c.dylib`task::spawn_opts::closure.8378 + 74
    frame #11: 0x00000001117c2a7c librustrt-4e7c5e5c.dylib`rust_try_inner + 12
    frame #12: 0x00007f818a500000
    frame #13: 0x000000010e2ec3d0 libnative-4e7c5e5c.dylib`task::spawn_opts::closure.8324 + 240
    frame #14: 0x000000011175e490 librustrt-4e7c5e5c.dylib`thread::thread_start::hf75b60e3600e3439Fqd + 32
    frame #15: 0x00007fff9234d8bf libsystem_c.dylib`_pthread_start + 335
    frame #16: 0x00007fff92350b75 libsystem_c.dylib`thread_start + 13
