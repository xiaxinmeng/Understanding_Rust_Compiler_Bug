
error: internal compiler error: src/librustc_mir/borrow_check/universal_regions.rs:754: cannot convert `ReEarlyBound(0, 'a)` to a region vid

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
stack backtrace:
   0:        0x10b21cf6f - <unknown>
   1:        0x10b251f5e - <unknown>
   2:        0x10b20e217 - <unknown>
   3:        0x10b22191a - <unknown>
   4:        0x10b22165c - <unknown>
   5:        0x10f9217f8 - <unknown>
   6:        0x10b222035 - <unknown>
   7:        0x1138bb5e6 - <unknown>
   8:        0x113576037 - <unknown>
   9:        0x113574cf7 - <unknown>
  10:        0x1132d71a0 - <unknown>
  11:        0x1132ccd66 - <unknown>
  12:        0x1132cccec - <unknown>
  13:        0x1132d70a8 - <unknown>
  14:        0x1138b473b - <unknown>
  15:        0x1122006ac - <unknown>
  16:        0x11220060a - <unknown>
  17:        0x1120bc13e - <unknown>
  18:        0x1124b6f24 - <unknown>
  19:        0x1121f0c88 - <unknown>
  20:        0x1123ef10e - <unknown>
  21:        0x1124aef9a - <unknown>
  22:        0x1123ec87a - <unknown>
  23:        0x1120b46ca - <unknown>
  24:        0x112449edb - <unknown>
  25:        0x112253042 - <unknown>
  26:        0x112477519 - <unknown>
  27:        0x11244a692 - <unknown>
  28:        0x1123fa81c - <unknown>
  29:        0x1120b4a8a - <unknown>
  30:        0x112449fcb - <unknown>
  31:        0x11225e0a2 - <unknown>
  32:        0x11244eca8 - <unknown>
  33:        0x1122c6593 - <unknown>
  34:        0x112431f70 - <unknown>
  35:        0x1120b4d46 - <unknown>
  36:        0x11244a098 - <unknown>
  37:        0x11225115b - <unknown>
  38:        0x11248e697 - <unknown>
  39:        0x112430eba - <unknown>
  40:        0x1120b5e26 - <unknown>
  41:        0x11244a458 - <unknown>
  42:        0x1122641e1 - <unknown>
  43:        0x112453853 - <unknown>
  44:        0x112430c2a - <unknown>
  45:        0x1131478ec - <unknown>
  46:        0x112f98608 - <unknown>
  47:        0x113316325 - <unknown>
  48:        0x112e81de4 - <unknown>
  49:        0x112e72cb2 - <unknown>
  50:        0x112c92577 - <unknown>
  51:        0x112c9a6fe - <unknown>
  52:        0x112bd1bf1 - <unknown>
  53:        0x112c9a18e - <unknown>
  54:        0x111c15ea1 - <unknown>
  55:        0x111c0014c - <unknown>
  56:        0x111d36462 - <unknown>
  57:        0x111cf789f - <unknown>
  58:        0x111c64518 - <unknown>
  59:        0x111c5932a - <unknown>
  60:        0x111c833db - <unknown>
  61:        0x111bed77f - <unknown>
  62:        0x111cd24ea - <unknown>
  63:        0x111c83850 - <unknown>
  64:        0x111bb6e98 - <unknown>
  65:        0x111d0e83d - <unknown>
  66:        0x111c13e91 - <unknown>
  67:        0x111d2be80 - <unknown>
  68:        0x10fb484ca - <unknown>
  69:        0x10f92b5c9 - <unknown>
  70:        0x10f985595 - <unknown>
  71:        0x10f961403 - <unknown>
  72:        0x10f993c3c - <unknown>
  73:        0x10f966068 - <unknown>
  74:        0x10f927e85 - <unknown>
  75:        0x10f968b7c - <unknown>
  76:        0x10b1fe56e - <unknown>
  77:        0x10b230376 - <unknown>
  78:     0x7fff71dcb2eb - <unknown>
  79:     0x7fff71dce249 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (1057dc97a 2020-03-20) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C linker=x86_64-linux-musl-cc -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `file_descriptors::directory::DirectoryEntryIterator::buffer::{{constant}}#0`
#1 [optimized_mir] processing `file_descriptors::directory::DirectoryEntryIterator::buffer::{{constant}}#0`
#2 [const_eval_raw] const-evaluating `file_descriptors::directory::DirectoryEntryIterator::buffer::{{constant}}#0`
#3 [const_eval_validated] const-evaluating + checking `file_descriptors::directory::DirectoryEntryIterator::buffer::{{constant}}#0`
#4 [const_eval_validated] const-evaluating + checking `file_descriptors::directory::DirectoryEntryIterator::buffer::{{constant}}#0`
#5 [check_item_well_formed] processing `file_descriptors::directory::DirectoryEntryIterator`
#6 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `linux-support`.
