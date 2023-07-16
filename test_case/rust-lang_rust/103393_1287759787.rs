plain
   Compiling derive_more v0.13.0
   Compiling darling_macro v0.8.0
   Compiling darling v0.8.0
   Compiling derive_common v0.0.1 (/checkout/obj/build/ct/servo/components/derive_common)
thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"deg\" => v, \"grad\" => v * 360. / 400., \"rad\" => v * 360. / (2. * PI),\n    \"turn\" => v * 360., _ => return\n    Err(location.new_unexpected_token_error(Token ::\n    Ident(unit.clone()))),)).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155a8b - cssparser_macros::cssparser_internal__assert_ascii_lowercase__max_len::{{closure}}::h99c56391f18531f8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817890f - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c4a6c4fcb8e4a71
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164b10 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hfb23e818345ad7c3
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c59f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfcea70caf3ac4f58
  17:     0x7ff7c816be36 - std::panicking::try::do_call::h6c601d498bdd10c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816bb9b - std::panicking::try::hca74d43fb07ab5f7
  20:     0x7ff7c814ef10 - std::panic::catch_unwind::h8d9f2f4576ede1aa
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164021 - std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3be1dd08510714
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f41e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd9220e78bcca515
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:39:17
    |
    |
39  |                   cssparser_internal__assert_ascii_lowercase__max_len!( $( $match_body )* )
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:227:31
    |
    |
227 |                   let degrees = match_ignore_ascii_case! { &*unit,
    |  _______________________________-
228 | |                     "deg" => v,
229 | |                     "grad" => v * 360. / 400.,
230 | |                     "rad" => v * 360. / (2. * PI),
231 | |                     "turn" => v * 360.,
232 | |                     _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
    | |_________________- in this macro invocation
    |
    = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    ((Color) \"black\" (rgb!(0, 0, 0)) \"silver\" (rgb!(192, 192, 192)) \"gray\"\n    (rgb!(128, 128, 128)) \"white\" (rgb!(255, 255, 255)) \"maroon\"\n    (rgb!(128, 0, 0)) \"red\" (rgb!(255, 0, 0)) \"purple\" (rgb!(128, 0, 128))\n    \"fuchsia\" (rgb!(255, 0, 255)) \"green\" (rgb!(0, 128, 0)) \"lime\"\n    (rgb!(0, 255, 0)) \"olive\" (rgb!(128, 128, 0)) \"yellow\" (rgb!(255, 255, 0))\n    \"navy\" (rgb!(0, 0, 128)) \"blue\" (rgb!(0, 0, 255)) \"teal\"\n    (rgb!(0, 128, 128)) \"aqua\" (rgb!(0, 255, 255)) \"aliceblue\"\n    (rgb!(240, 248, 255)) \"antiquewhite\" (rgb!(250, 235, 215)) \"aquamarine\"\n    (rgb!(127, 255, 212)) \"azure\" (rgb!(240, 255, 255)) \"beige\"\n    (rgb!(245, 245, 220)) \"bisque\" (rgb!(255, 228, 196)) \"blanchedalmond\"\n    (rgb!(255, 235, 205)) \"blueviolet\" (rgb!(138, 43, 226)) \"brown\"\n    (rgb!(165, 42, 42)) \"burlywood\" (rgb!(222, 184, 135)) \"cadetblue\"\n    (rgb!(95, 158, 160)) \"chartreuse\" (rgb!(127, 255, 0)) \"chocolate\"\n    (rgb!(210, 105, 30)) \"coral\" (rgb!(255, 127, 80)) \"cornflowerblue\"\n    (rgb!(100, 149, 237)) \"cornsilk\" (rgb!(255, 248, 220)) \"crimson\"\n    (rgb!(220, 20, 60)) \"cyan\" (rgb!(0, 255, 255)) \"darkblue\"\n    (rgb!(0, 0, 139)) \"darkcyan\" (rgb!(0, 139, 139)) \"darkgoldenrod\"\n    (rgb!(184, 134, 11)) \"darkgray\" (rgb!(169, 169, 169)) \"darkgreen\"\n    (rgb!(0, 100, 0)) \"darkgrey\" (rgb!(169, 169, 169)) \"darkkhaki\"\n    (rgb!(189, 183, 107)) \"darkmagenta\" (rgb!(139, 0, 139)) \"darkolivegreen\"\n    (rgb!(85, 107, 47)) \"darkorange\" (rgb!(255, 140, 0)) \"darkorchid\"\n    (rgb!(153, 50, 204)) \"darkred\" (rgb!(139, 0, 0)) \"darksalmon\"\n    (rgb!(233, 150, 122)) \"darkseagreen\" (rgb!(143, 188, 143)) \"darkslateblue\"\n    (rgb!(72, 61, 139)) \"darkslategray\" (rgb!(47, 79, 79)) \"darkslategrey\"\n    (rgb!(47, 79, 79)) \"darkturquoise\" (rgb!(0, 206, 209)) \"darkviolet\"\n    (rgb!(148, 0, 211)) \"deeppink\" (rgb!(255, 20, 147)) \"deepskyblue\"\n    (rgb!(0, 191, 255)) \"dimgray\" (rgb!(105, 105, 105)) \"dimgrey\"\n    (rgb!(105, 105, 105)) \"dodgerblue\" (rgb!(30, 144, 255)) \"firebrick\"\n    (rgb!(178, 34, 34)) \"floralwhite\" (rgb!(255, 250, 240)) \"forestgreen\"\n    (rgb!(34, 139, 34)) \"gainsboro\" (rgb!(220, 220, 220)) \"ghostwhite\"\n    (rgb!(248, 248, 255)) \"gold\" (rgb!(255, 215, 0)) \"goldenrod\"\n    (rgb!(218, 165, 32)) \"greenyellow\" (rgb!(173, 255, 47)) \"grey\"\n    (rgb!(128, 128, 128)) \"honeydew\" (rgb!(240, 255, 240)) \"hotpink\"\n    (rgb!(255, 105, 180)) \"indianred\" (rgb!(205, 92, 92)) \"indigo\"\n    (rgb!(75, 0, 130)) \"ivory\" (rgb!(255, 255, 240)) \"khaki\"\n    (rgb!(240, 230, 140)) \"lavender\" (rgb!(230, 230, 250)) \"lavenderblush\"\n    (rgb!(255, 240, 245)) \"lawngreen\" (rgb!(124, 252, 0)) \"lemonchiffon\"\n    (rgb!(255, 250, 205)) \"lightblue\" (rgb!(173, 216, 230)) \"lightcoral\"\n    (rgb!(240, 128, 128)) \"lightcyan\" (rgb!(224, 255, 255))\n    \"lightgoldenrodyellow\" (rgb!(250, 250, 210)) \"lightgray\"\n    (rgb!(211, 211, 211)) \"lightgreen\" (rgb!(144, 238, 144)) \"lightgrey\"\n    (rgb!(211, 211, 211)) \"lightpink\" (rgb!(255, 182, 193)) \"lightsalmon\"\n    (rgb!(255, 160, 122)) \"lightseagreen\" (rgb!(32, 178, 170)) \"lightskyblue\"\n    (rgb!(135, 206, 250)) \"lightslategray\" (rgb!(119, 136, 153))\n    \"lightslategrey\" (rgb!(119, 136, 153)) \"lightsteelblue\"\n    (rgb!(176, 196, 222)) \"lightyellow\" (rgb!(255, 255, 224)) \"limegreen\"\n    (rgb!(50, 205, 50)) \"linen\" (rgb!(250, 240, 230)) \"magenta\"\n    (rgb!(255, 0, 255)) \"mediumaquamarine\" (rgb!(102, 205, 170)) \"mediumblue\"\n    (rgb!(0, 0, 205)) \"mediumorchid\" (rgb!(186, 85, 211)) \"mediumpurple\"\n    (rgb!(147, 112, 219)) \"mediumseagreen\" (rgb!(60, 179, 113))\n    \"mediumslateblue\" (rgb!(123, 104, 238)) \"mediumspringgreen\"\n    (rgb!(0, 250, 154)) \"mediumturquoise\" (rgb!(72, 209, 204))\n    \"mediumvioletred\" (rgb!(199, 21, 133)) \"midnightblue\" (rgb!(25, 25, 112))\n    \"mintcream\" (rgb!(245, 255, 250)) \"mistyrose\" (rgb!(255, 228, 225))\n    \"moccasin\" (rgb!(255, 228, 181)) \"navajowhite\" (rgb!(255, 222, 173))\n    \"oldlace\" (rgb!(253, 245, 230)) \"olivedrab\" (rgb!(107, 142, 35)) \"orange\"\n    (rgb!(255, 165, 0)) \"orangered\" (rgb!(255, 69, 0)) \"orchid\"\n    (rgb!(218, 112, 214)) \"palegoldenrod\" (rgb!(238, 232, 170)) \"palegreen\"\n    (rgb!(152, 251, 152)) \"paleturquoise\" (rgb!(175, 238, 238))\n    \"palevioletred\" (rgb!(219, 112, 147)) \"papayawhip\" (rgb!(255, 239, 213))\n    \"peachpuff\" (rgb!(255, 218, 185)) \"peru\" (rgb!(205, 133, 63)) \"pink\"\n    (rgb!(255, 192, 203)) \"plum\" (rgb!(221, 160, 221)) \"powderblue\"\n    (rgb!(176, 224, 230)) \"rebeccapurple\" (rgb!(102, 51, 153)) \"rosybrown\"\n    (rgb!(188, 143, 143)) \"royalblue\" (rgb!(65, 105, 225)) \"saddlebrown\"\n    (rgb!(139, 69, 19)) \"salmon\" (rgb!(250, 128, 114)) \"sandybrown\"\n    (rgb!(244, 164, 96)) \"seagreen\" (rgb!(46, 139, 87)) \"seashell\"\n    (rgb!(255, 245, 238)) \"sienna\" (rgb!(160, 82, 45)) \"skyblue\"\n    (rgb!(135, 206, 235)) \"slateblue\" (rgb!(106, 90, 205)) \"slategray\"\n    (rgb!(112, 128, 144)) \"slategrey\" (rgb!(112, 128, 144)) \"snow\"\n    (rgb!(255, 250, 250)) \"springgreen\" (rgb!(0, 255, 127)) \"steelblue\"\n    (rgb!(70, 130, 180)) \"tan\" (rgb!(210, 180, 140)) \"thistle\"\n    (rgb!(216, 191, 216)) \"tomato\" (rgb!(255, 99, 71)) \"turquoise\"\n    (rgb!(64, 224, 208)) \"violet\" (rgb!(238, 130, 238)) \"wheat\"\n    (rgb!(245, 222, 179)) \"whitesmoke\" (rgb!(245, 245, 245)) \"yellowgreen\"\n    (rgb!(154, 205, 50)) \"transparent\"\n    (Color::RGBA(RGBA { red: 0, green: 0, blue: 0, alpha: 0 })) \"currentcolor\"\n    (Color::CurrentColor))).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8156cfb - cssparser_macros::cssparser_internal__phf_map::{{closure}}::h37d67d37afdd20ba
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817894f - std::sys_common::backtrace::__rust_begin_short_backtrace::h76f3b62b86d65aab
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164a90 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::h212f8cf30f94c343
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c4af - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h4bb28bdac4c5f1a9
  17:     0x7ff7c816bcc6 - std::panicking::try::do_call::h1ca9bab0dbe8a743
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816b56b - std::panicking::try::h0828cd5123d999a4
  20:     0x7ff7c814ef50 - std::panic::catch_unwind::hd4dce24d3380f57a
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c81644c1 - std::thread::Builder::spawn_unchecked_::{{closure}}::ha56d793bfe063f3f
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f35e - core::ops::function::FnOnce::call_once{{vtable.shim}}::h036c5425656a7b2c
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:87:17
    |
    |
87  |                   cssparser_internal__phf_map!( ($ValueType) $( $key ($value) )+ )
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:371:5
    |
    |
371 | /     ascii_case_insensitive_phf_map! {
372 | |         keyword -> Color = {
373 | |             "black" => rgb!(0, 0, 0),
374 | |             "silver" => rgb!(192, 192, 192),
525 | |         }
526 | |     }
    | |_____- in this macro invocation
    |
    |
    = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
    = note: this error originates in the macro `ascii_case_insensitive_phf_map` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"black\" \"silver\" \"gray\" \"white\" \"maroon\" \"red\" \"purple\" \"fuchsia\" \"green\"\n    \"lime\" \"olive\" \"yellow\" \"navy\" \"blue\" \"teal\" \"aqua\" \"aliceblue\"\n    \"antiquewhite\" \"aquamarine\" \"azure\" \"beige\" \"bisque\" \"blanchedalmond\"\n    \"blueviolet\" \"brown\" \"burlywood\" \"cadetblue\" \"chartreuse\" \"chocolate\"\n    \"coral\" \"cornflowerblue\" \"cornsilk\" \"crimson\" \"cyan\" \"darkblue\" \"darkcyan\"\n    \"darkgoldenrod\" \"darkgray\" \"darkgreen\" \"darkgrey\" \"darkkhaki\"\n    \"darkmagenta\" \"darkolivegreen\" \"darkorange\" \"darkorchid\" \"darkred\"\n    \"darksalmon\" \"darkseagreen\" \"darkslateblue\" \"darkslategray\"\n    \"darkslategrey\" \"darkturquoise\" \"darkviolet\" \"deeppink\" \"deepskyblue\"\n    \"dimgray\" \"dimgrey\" \"dodgerblue\" \"firebrick\" \"floralwhite\" \"forestgreen\"\n    \"gainsboro\" \"ghostwhite\" \"gold\" \"goldenrod\" \"greenyellow\" \"grey\"\n    \"honeydew\" \"hotpink\" \"indianred\" \"indigo\" \"ivory\" \"khaki\" \"lavender\"\n    \"lavenderblush\" \"lawngreen\" \"lemonchiffon\" \"lightblue\" \"lightcoral\"\n    \"lightcyan\" \"lightgoldenrodyellow\" \"lightgray\" \"lightgreen\" \"lightgrey\"\n    \"lightpink\" \"lightsalmon\" \"lightseagreen\" \"lightskyblue\" \"lightslategray\"\n    \"lightslategrey\" \"lightsteelblue\" \"lightyellow\" \"limegreen\" \"linen\"\n    \"magenta\" \"mediumaquamarine\" \"mediumblue\" \"mediumorchid\" \"mediumpurple\"\n    \"mediumseagreen\" \"mediumslateblue\" \"mediumspringgreen\" \"mediumturquoise\"\n    \"mediumvioletred\" \"midnightblue\" \"mintcream\" \"mistyrose\" \"moccasin\"\n    \"navajowhite\" \"oldlace\" \"olivedrab\" \"orange\" \"orangered\" \"orchid\"\n    \"palegoldenrod\" \"palegreen\" \"paleturquoise\" \"palevioletred\" \"papayawhip\"\n    \"peachpuff\" \"peru\" \"pink\" \"plum\" \"powderblue\" \"rebeccapurple\" \"rosybrown\"\n    \"royalblue\" \"saddlebrown\" \"salmon\" \"sandybrown\" \"seagreen\" \"seashell\"\n    \"sienna\" \"skyblue\" \"slateblue\" \"slategray\" \"slategrey\" \"snow\"\n    \"springgreen\" \"steelblue\" \"tan\" \"thistle\" \"tomato\" \"turquoise\" \"violet\"\n    \"wheat\" \"whitesmoke\" \"yellowgreen\" \"transparent\" \"currentcolor\")).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155f0b - cssparser_macros::cssparser_internal__max_len::{{closure}}::h4571630688bc0605
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817898f - std::sys_common::backtrace::__rust_begin_short_backtrace::h8bdfd3e00c123932
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164ad0 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hb6ad7a3ad570b97e
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c46f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h4a42cb4318bf0539
  17:     0x7ff7c816bfc6 - std::panicking::try::do_call::hf3c0a97be71ac8c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816ba1b - std::panicking::try::hb86e7d75fcf31c44
  19:     0x7ff7c816ba1b - std::panicking::try::hb86e7d75fcf31c44
                               at /checkout/library/std/src/panicking.rs:447:19
  20:     0x7ff7c814eed0 - std::panic::catch_unwind::h81d2a3090fd809fa
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164881 - std::thread::Builder::spawn_unchecked_::{{closure}}::hc7b5ced4ecc7b0db
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f3be - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6e92bec132b60f89
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:92:21
    |
    |
92  |                       cssparser_internal__max_len!( $( $key )+ )
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:371:5
    |
    |
371 | /     ascii_case_insensitive_phf_map! {
372 | |         keyword -> Color = {
373 | |             "black" => rgb!(0, 0, 0),
374 | |             "silver" => rgb!(192, 192, 192),
525 | |         }
526 | |     }
    | |_____- in this macro invocation
    |
    |
    = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
    = note: this error originates in the macro `ascii_case_insensitive_phf_map` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"rgb\" | \"rgba\" => parse_rgb_components_rgb(component_parser, arguments)\n    ?, \"hsl\" | \"hsla\" => parse_rgb_components_hsl(component_parser, arguments)\n    ?, _ => return\n    Err(arguments.new_unexpected_token_error(Token ::\n    Ident(name.to_owned().into()))),)).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155a8b - cssparser_macros::cssparser_internal__assert_ascii_lowercase__max_len::{{closure}}::h99c56391f18531f8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817890f - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c4a6c4fcb8e4a71
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164b10 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hfb23e818345ad7c3
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c59f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfcea70caf3ac4f58
  17:     0x7ff7c816be36 - std::panicking::try::do_call::h6c601d498bdd10c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816bb9b - std::panicking::try::hca74d43fb07ab5f7
  20:     0x7ff7c814ef10 - std::panic::catch_unwind::h8d9f2f4576ede1aa
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164021 - std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3be1dd08510714
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f41e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd9220e78bcca515
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:39:17
    |
    |
39  |                   cssparser_internal__assert_ascii_lowercase__max_len!( $( $match_body )* )
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:571:43
    |
    |
571 |       let (red, green, blue, uses_commas) = match_ignore_ascii_case! { name,
    |  ___________________________________________-
572 | |         "rgb" | "rgba" => parse_rgb_components_rgb(component_parser, arguments)?,
573 | |         "hsl" | "hsla" => parse_rgb_components_hsl(component_parser, arguments)?,
574 | |         _ => return Err(arguments.new_unexpected_token_error(Token::Ident(name.to_owned().into()))),
    | |_____- in this macro invocation
    |
    = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"n\" => Ok(parse_b(input, a) ?), \"n-\" =>\n    Ok(parse_signless_b(input, a, - 1) ?), _ => match\n    parse_n_dash_digits(& * unit)\n    {\n        Ok(b) => Ok((a, b)), Err(()) =>\n        Err(input.new_basic_unexpected_token_error(Token ::\n        Ident(unit.clone())))\n    })).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155a8b - cssparser_macros::cssparser_internal__assert_ascii_lowercase__max_len::{{closure}}::h99c56391f18531f8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817890f - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c4a6c4fcb8e4a71
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164b10 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hfb23e818345ad7c3
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c59f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfcea70caf3ac4f58
  17:     0x7ff7c816be36 - std::panicking::try::do_call::h6c601d498bdd10c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816bb9b - std::panicking::try::hca74d43fb07ab5f7
  20:     0x7ff7c814ef10 - std::panic::catch_unwind::h8d9f2f4576ede1aa
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164021 - std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3be1dd08510714
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f41e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd9220e78bcca515
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:39:17
   |
   |
39 |                   cssparser_internal__assert_ascii_lowercase__max_len!( $( $match_body )* )
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/nth.rs:22:13
   |
22 | /             match_ignore_ascii_case! {
22 | /             match_ignore_ascii_case! {
23 | |                 &unit,
24 | |                 "n" => Ok(parse_b(input, a)?),
25 | |                 "n-" => Ok(parse_signless_b(input, a, -1)?),
29 | |                 }
30 | |             }
   | |_____________- in this macro invocation
   |
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
   = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"even\" => Ok((2, 0)), \"odd\" => Ok((2, 1)), \"n\" =>\n    Ok(parse_b(input, 1) ?), \"-n\" => Ok(parse_b(input, - 1) ?), \"n-\" =>\n    Ok(parse_signless_b(input, 1, - 1) ?), \"-n-\" =>\n    Ok(parse_signless_b(input, - 1, - 1) ?), _ =>\n    {\n        let(slice, a) = if value.starts_with(\"-\") { (& value [1 ..], - 1) }\n        else { (& * value, 1) } ; match parse_n_dash_digits(slice)\n        {\n            Ok(b) => Ok((a, b)), Err(()) =>\n            Err(input.new_basic_unexpected_token_error(Token ::\n            Ident(value.clone())))\n        }\n    })).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155a8b - cssparser_macros::cssparser_internal__assert_ascii_lowercase__max_len::{{closure}}::h99c56391f18531f8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817890f - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c4a6c4fcb8e4a71
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164b10 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hfb23e818345ad7c3
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c59f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfcea70caf3ac4f58
  17:     0x7ff7c816be36 - std::panicking::try::do_call::h6c601d498bdd10c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816bb9b - std::panicking::try::hca74d43fb07ab5f7
  20:     0x7ff7c814ef10 - std::panic::catch_unwind::h8d9f2f4576ede1aa
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164021 - std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3be1dd08510714
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f41e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd9220e78bcca515
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:39:17
   |
   |
39 |                   cssparser_internal__assert_ascii_lowercase__max_len!( $( $match_body )* )
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/nth.rs:33:13
   |
33 | /             match_ignore_ascii_case! { &value,
33 | /             match_ignore_ascii_case! { &value,
34 | |                 "even" => Ok((2, 0)),
35 | |                 "odd" => Ok((2, 1)),
36 | |                 "n" => Ok(parse_b(input, 1)?),
50 | |                 }
51 | |             }
   | |_____________- in this macro invocation
   |
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
   = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)

thread '<unnamed>' panicked at 'expected suffix ").0," not found in "#[allow(unused)] enum ProceduralMasqueradeDummyType\n{\n    Input =\n    (0, stringify!\n    (\"n\" => parse_b(input, 1), \"n-\" => parse_signless_b(input, 1, - 1), _ =>\n    match parse_n_dash_digits(& * value)\n    {\n        Ok(b) => Ok((1, b)), Err(()) =>\n        Err(input.new_basic_unexpected_token_error(Token ::\n        Ident(value.clone())))\n    })).0\n}"', /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   0:     0x7ff7c83185d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2399ed0976353927
   1:     0x7ff7c8335fe8 - core::fmt::write::h528091302fef9f5d
   2:     0x7ff7c8316a21 - std::io::Write::write_fmt::hd5f03cdb40b9b644
   3:     0x7ff7c8318395 - std::sys_common::backtrace::print::h4b313cf87995efdb
   4:     0x7ff7c831a3b7 - std::panicking::default_hook::{{closure}}::h78a948a373d372be
   5:     0x7ff7c831a115 - std::panicking::default_hook::hd44d981863f10854
   6:     0x7ff7c82e8292 - proc_macro::bridge::client::maybe_install_panic_hook::{{closure}}::{{closure}}::he08ee3782a8f2926
   7:     0x7ff7c831abd3 - std::panicking::rust_panic_with_hook::hab06249292d721f8
   8:     0x7ff7c831a907 - std::panicking::begin_panic_handler::{{closure}}::hb278a440b64f982a
   9:     0x7ff7c8318b7c - std::sys_common::backtrace::__rust_end_short_backtrace::hb4ef235c06175d4e
  10:     0x7ff7c831a5d2 - rust_begin_unwind
  11:     0x7ff7c8132463 - core::panicking::panic_fmt::h52a1e46e2afc6bb0
  12:     0x7ff7c82f5577 - procedural_masquerade::_extract_input::h6904df4968a57681
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:211:9
  13:     0x7ff7c8155a8b - cssparser_macros::cssparser_internal__assert_ascii_lowercase__max_len::{{closure}}::h99c56391f18531f8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/procedural-masquerade-0.1.6/lib.rs:185:33
  14:     0x7ff7c817890f - std::sys_common::backtrace::__rust_begin_short_backtrace::h3c4a6c4fcb8e4a71
                               at /checkout/library/std/src/sys_common/backtrace.rs:121:18
  15:     0x7ff7c8164b10 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::hfb23e818345ad7c3
                               at /checkout/library/std/src/thread/mod.rs:551:17
  16:     0x7ff7c815c59f - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfcea70caf3ac4f58
  17:     0x7ff7c816be36 - std::panicking::try::do_call::h6c601d498bdd10c5
                               at /checkout/library/std/src/panicking.rs:483:40
  18:     0x7ff7c816ea9b - __rust_try
  18:     0x7ff7c816ea9b - __rust_try
  19:     0x7ff7c816bb9b - std::panicking::try::hca74d43fb07ab5f7
  20:     0x7ff7c814ef10 - std::panic::catch_unwind::h8d9f2f4576ede1aa
                               at /checkout/library/std/src/panic.rs:137:14
                               at /checkout/library/std/src/panic.rs:137:14
  21:     0x7ff7c8164021 - std::thread::Builder::spawn_unchecked_::{{closure}}::h6c3be1dd08510714
                               at /checkout/library/std/src/thread/mod.rs:550:30
  22:     0x7ff7c814f41e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbd9220e78bcca515
                               at /checkout/library/core/src/ops/function.rs:251:5
  23:     0x7ff7c831dc3e - std::sys::unix::thread::Thread::new::thread_start::hf511873dc087fa27
  24:     0x7ff7dbb1cb43 - <unknown>
  25:     0x7ff7dbbaea00 - <unknown>
  26:                0x0 - <unknown>
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:39:17
   |
   |
39 |                   cssparser_internal__assert_ascii_lowercase__max_len!( $( $match_body )* )
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/nth.rs:56:17
   |
56 | /                 match_ignore_ascii_case! { &value,
56 | /                 match_ignore_ascii_case! { &value,
57 | |                     "n" => parse_b(input, 1),
58 | |                     "n-" => parse_signless_b(input, 1, -1),
59 | |                     _ => match parse_n_dash_digits(&*value) {
62 | |                     }
63 | |                 }
   | |_________________- in this macro invocation
   |
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Any { .. }
   = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find value `MAX_LENGTH` in this scope
    |
    |
44  |                   cssparser_internal__to_lowercase!($input, MAX_LENGTH => lowercase);
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:227:31
    |
    |
227 |                   let degrees = match_ignore_ascii_case! { &*unit,
    |  _______________________________-
228 | |                     "deg" => v,
229 | |                     "grad" => v * 360. / 400.,
230 | |                     "rad" => v * 360. / (2. * PI),
231 | |                     "turn" => v * 360.,
232 | |                     _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
    | |_________________- in this macro invocation
    |
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0425]: cannot find value `MAX_LENGTH` in this scope
    |
    |
95  |                   cssparser_internal__to_lowercase!(input, MAX_LENGTH => lowercase);
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:371:5
    |
    |
371 | /     ascii_case_insensitive_phf_map! {
372 | |         keyword -> Color = {
373 | |             "black" => rgb!(0, 0, 0),
374 | |             "silver" => rgb!(192, 192, 192),
525 | |         }
526 | |     }
    | |_____- in this macro invocation
    |
    |
    = note: this error originates in the macro `ascii_case_insensitive_phf_map` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `MAP` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/macros.rs:96:40
    |
    |
96  |                   lowercase.and_then(|s| MAP.get(s))
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:371:5
    |
    |
371 | /     ascii_case_insensitive_phf_map! {
372 | |         keyword -> Color = {
373 | |             "black" => rgb!(0, 0, 0),
374 | |             "silver" => rgb!(192, 192, 192),
525 | |         }
526 | |     }
    | |_____- in this macro invocation
    |
    |
    = note: this error originates in the macro `ascii_case_insensitive_phf_map` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find value `MAX_LENGTH` in this scope
    |
    |
44  |                   cssparser_internal__to_lowercase!($input, MAX_LENGTH => lowercase);
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/color.rs:571:43
    |
    |
571 |       let (red, green, blue, uses_commas) = match_ignore_ascii_case! { name,
    |  ___________________________________________-
572 | |         "rgb" | "rgba" => parse_rgb_components_rgb(component_parser, arguments)?,
573 | |         "hsl" | "hsla" => parse_rgb_components_hsl(component_parser, arguments)?,
574 | |         _ => return Err(arguments.new_unexpected_token_error(Token::Ident(name.to_owned().into()))),
    | |_____- in this macro invocation
    |
    = note: this error originates in the macro `match_ignore_ascii_case` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0425]: cannot find value `MAX_LENGTH` in this scope
   |
   |
44 |                   cssparser_internal__to_lowercase!($input, MAX_LENGTH => lowercase);
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.25.5/src/nth.rs:22:13
   |
---
warning: `to_shmem_derive` (lib) generated 2 warnings
thread 'main' panicked at 'tests failed for https://github.com/servo/servo', src/tools/cargotest/main.rs:124:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:18:41
make: *** [Makefile:44: check-aux] Error 1
