plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: duplicate diagnostic item found: `dbg_macro`.
    |
291 | / macro_rules! dbg {
291 | / macro_rules! dbg {
292 | |     // NOTE: We cannot use `concat!` to make a static string as a format argument
293 | |     // of `eprintln!` because `file!` could contain a `{` or
294 | |     // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
312 | |     };
313 | | }
    | |_^
    |
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `eprint_macro`.
    |
128 | / macro_rules! eprint {
128 | / macro_rules! eprint {
129 | |     ($($arg:tt)*) => ($crate::io::_eprint($crate::format_args!($($arg)*)));
    | |_^
    |
    = note: the diagnostic item is first defined in crate `std`.


error: duplicate diagnostic item found: `thread_local_macro`.
    |
147 | / macro_rules! thread_local {
148 | |     // empty (base case for the recursion)
149 | |     () => {};
---
    | |_^
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `print_macro`.
   |
62 | / macro_rules! print {
63 | |     ($($arg:tt)*) => ($crate::io::_print($crate::format_args!($($arg)*)));
64 | | }
64 | | }
   | |_^
   |
   = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `println_macro`.
    |
96  | / macro_rules! println {
97  | |     () => ($crate::print!("\n"));
98  | |     ($($arg:tt)*) => ({
98  | |     ($($arg:tt)*) => ({
99  | |         $crate::io::_print($crate::format_args_nl!($($arg)*));
100 | |     })
101 | | }
    | |_^
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `eprintln_macro`.
    |
157 | / macro_rules! eprintln {
157 | / macro_rules! eprintln {
158 | |     () => ($crate::eprint!("\n"));
160 | |         $crate::io::_eprint($crate::format_args_nl!($($arg)*));
161 | |     })
162 | | }
    | |_^
