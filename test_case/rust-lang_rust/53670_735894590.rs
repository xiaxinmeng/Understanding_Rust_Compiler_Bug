
#0  0x006f33f0 in crossbeam_queue::seg_queue::SegQueue<T>::push ()
#1  0x006f14e0 in rayon_core::registry::Registry::inject ()
#2  0x006d33c0 in std::thread::local::LocalKey<T>::with ()
#3  0x006dfab4 in rayon_core::registry::Registry::in_worker ()
#4  0x006d21f8 in installer::tarballer::Tarballer::run ()
#5  0x006d5b58 in installer::generator::Generator::run ()
#6  0x006ccdb0 in fabricate::main ()
#7  0x006cb7f8 in std::rt::lang_start::{{closure}} ()
#8  0x007ab83c in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:52
#9  std::panicking::try::do_call () at src/libstd/panicking.rs:348
#10 std::panicking::try () at src/libstd/panicking.rs:325
#11 std::panic::catch_unwind () at src/libstd/panic.rs:394
#12 std::rt::lang_start_internal () at src/libstd/rt.rs:51
#13 0x006cd90c in main ()
