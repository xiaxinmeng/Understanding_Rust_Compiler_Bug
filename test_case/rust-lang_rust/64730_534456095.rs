
error: internal compiler error: src/librustc_metadata/decoder.rs:483: entry: id not found: DefIndex(86) in crate aljabar with number 15

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
stack backtrace:
   0: <unknown>
   1: <unknown>
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: <unknown>
   6: <unknown>
   7: <unknown>
   8: <unknown>
   9: <unknown>
  10: <unknown>
  11: <unknown>
  12: <unknown>
  13: <unknown>
  14: <unknown>
  15: <unknown>
  16: <unknown>
  17: <unknown>
  18: <unknown>
  19: <unknown>
  20: <unknown>
  21: <unknown>
  22: <unknown>
  23: <unknown>
  24: <unknown>
  25: <unknown>
  26: <unknown>
  27: <unknown>
  28: <unknown>
  29: <unknown>
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <unknown>
  36: <unknown>
  37: <unknown>
  38: <unknown>
  39: <unknown>
  40: <unknown>
  41: <unknown>
  42: <unknown>
  43: <unknown>
  44: <unknown>
  45: <unknown>
  46: <unknown>
  47: <unknown>
  48: <unknown>
  49: <unknown>
  50: <unknown>
  51: <unknown>
  52: <unknown>
  53: <unknown>
  54: <unknown>
  55: <unknown>
  56: <unknown>
  57: <unknown>
  58: <unknown>
  59: <unknown>
  60: <unknown>
  61: <unknown>
  62: <unknown>
  63: <unknown>
  64: <unknown>
  65: <unknown>
  66: <unknown>
  67: <unknown>
  68: <unknown>
  69: <unknown>
  70: <unknown>
  71: <unknown>
  72: <unknown>
  73: <unknown>
  74: <unknown>
  75: <unknown>
  76: <unknown>
  77: <unknown>
  78: <unknown>
  79: <unknown>
  80: <unknown>
  81: <unknown>
  82: <unknown>
  83: <unknown>
  84: <unknown>
  85: <unknown>
  86: <unknown>
  87: <unknown>
  88: <unknown>
  89: <unknown>
  90: <unknown>
  91: <unknown>
  92: <unknown>
  93: <unknown>
  94: <unknown>
  95: <unknown>
  96: <unknown>
  97: <unknown>
  98: <unknown>
  99: <unknown>
query stack during panic:
#0 [type_of] processing `aljabar::Vector3::{{constant}}#0`
#1 [evaluate_obligation] evaluating trait selection obligation `aljabar::Vector<^1_0, 4usize>: std::ops::Mul<^1_1>`
#2 [typeck_tables_of] processing `tests::identity`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
