plain

failures:

---- test_cat::prop_cat_cols stdout ----
error: test failed, to rerun pass '--test tests'
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-7]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-24]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-37]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-46]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-57]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'main' panicked at 'tests failed for https://github.com/BurntSushi/xsv', src/tools/cargotest/main.rs:101:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-66]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-82]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-94]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-123]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-138]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"], ["6]\u{2}R", "\u{95e54}0£", "Y\u{94}\u{35f}R", "\u{8}¨", "2", "x\u{18}\u{fff5}", "1"], ["\u{6}\u{8a83e}犆", "", "/\u{9c}*\u{1e}", "3‰W", "", " \u{9b}\u{ad}", "‣"], ["\u{1}Z\u{8d}", "›", "3", "\u{2006}", ")", "41n", "B"], ["", "\u{85}\u{7f4a5}[", "®㆙~)", "", "¦WO", "", "/\u{92421}j"], ["*4", "㮵'©'", "", "S", "-mª", "Ⰶ", "쐈{"], ["", "pD1", "#", "", "\u{2067}\u{8f}", "", "¦["]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"], ["6]\u{2}R", "\u{95e54}0£", "Y\u{94}\u{35f}R", "\u{8}¨", "2", "x\u{18}\u{fff5}", "1"], ["\u{6}\u{8a83e}犆", "", "/\u{9c}*\u{1e}", "3‰W", "", " \u{9b}\u{ad}", "‣"], ["\u{1}Z\u{8d}", "›", "3", "\u{2006}", ")", "41n", "B"], ["", "\u{85}\u{7f4a5}[", "®㆙~)", "", "¦WO", "", "/\u{92421}j"], ["*4", "㮵'©'", "", "S", "-mª", "Ⰶ", "쐈{"], ["", "pD1", "#", "", "\u{2067}\u{8f}", "", "¦["]]`', tests/test_cat.rs:86:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-571]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-590]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-604]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"], ["6]\u{2}R", "\u{95e54}0£", "Y\u{94}\u{35f}R", "\u{8}¨", "2", "x\u{18}\u{fff5}", "1"], ["\u{6}\u{8a83e}犆", "", "/\u{9c}*\u{1e}", "3‰W", "", " \u{9b}\u{ad}", "‣"], ["\u{1}Z\u{8d}", "›", "3", "\u{2006}", ")", "41n", "B"], ["", "\u{85}\u{7f4a5}[", "®㆙~)", "", "¦WO", "", "/\u{92421}j"], ["*4", "㮵'©'", "", "S", "-mª", "Ⰶ", "쐈{"], ["", "pD1", "#", "", "\u{2067}\u{8f}", "", "¦["]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"], ["6]\u{2}R", "\u{95e54}0£", "Y\u{94}\u{35f}R", "\u{8}¨", "2", "x\u{18}\u{fff5}", "1"], ["\u{6}\u{8a83e}犆", "", "/\u{9c}*\u{1e}", "3‰W", "", " \u{9b}\u{ad}", "‣"], ["\u{1}Z\u{8d}", "›", "3", "\u{2006}", ")", "41n", "B"], ["", "\u{85}\u{7f4a5}[", "®㆙~)", "", "¦WO", "", "/\u{92421}j"], ["*4", "㮵'©'", "", "S", "-mª", "Ⰶ", "쐈{"], ["", "pD1", "#", "", "\u{2067}\u{8f}", "", "¦["]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1025]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1043]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1057]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["2", "\u{92}\u{17}", "1⁃\u{99}3", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"], ["z\u{4e8f3}>*", "", "7\u{6}\\", "\u{9a}", "齯\u{f283}\u{16}\u{100000}", "1", "{\u{97}\u{8e}"], ["", "3}", "\u{492ab}", "\u{1ce6}", "~", "\u{17}\u{81}_\u{cbedd}", ""], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1207]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1216]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1226]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"], ["\u{cff68}䨅W\u{fff5}", "\u{10}:\n", "\u{180e} ", "9ª\u{8d}囃", "\u{80}", "", "^|ꪎ"], ["5\u{dfbb8}^", "‹\u{96}1", "~L};", "\u{8c}튇퉲", "+\n\u{1d292}\u{80}", "\u{b}!", "z\u{ffffd}\r!"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1369]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1379]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1393]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"], ["덙:y", "r@\u{ef8ff}¡", "", "", "h=g", "\u{6}\u{8}-᾽", "\u{202f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1489]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1496]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1502]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"]]`,
 right: `[["\u{feff}", "\u{98}‛\u{e3923}", "\u{2}", "", "ퟔ", "‒", "\u{f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1562]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1570]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1576]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1602]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1609]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1616]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1627]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1637]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1646]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1651]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1654]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1658]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1661]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1664]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1667]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1671]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1674]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1677]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "", "\u{2}", "", "ퟔ", "‒", "\u{f}"]]`,
 right: `[["\u{feff}", "", "\u{2}", "", "ퟔ", "‒", "\u{f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1686]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1689]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1693]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1695]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1698]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1702]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1705]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1708]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1712]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1714]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1718]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1721]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1723]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1726]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1727]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1730]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1731]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1733]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "", "", "", "ퟔ", "‒", "\u{f}"]]`,
 right: `[["\u{feff}", "", "", "", "ퟔ", "‒", "\u{f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1735]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1736]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1738]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1739]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1740]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1742]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1743]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1745]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1746]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1748]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1749]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1750]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1752]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1753]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1755]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1756]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1758]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1759]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1760]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1762]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[["", "", "", "ퟔ", "‒", "\u{f}"]]`,
 right: `[["\u{feff}", "", "", "ퟔ", "‒", "\u{f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1764]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1765]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1767]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1768]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1770]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1771]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1773]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1774]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1775]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1777]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1778]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1780]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1781]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1782]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1784]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1785]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1787]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1788]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1789]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1791]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", "ퟔ", "‒", "\u{f}"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1793]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1794]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1796]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1797]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1799]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1800]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1802]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1803]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1804]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1806]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1807]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1809]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1810]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1811]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1813]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1814]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1816]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1817]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1819]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "ఖf", "\u{96}K", "\u{a4cf}\u{769d3}\u{83}", "\u{fff2}⁀{"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1820]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1822]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1823]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1824]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1826]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1827]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1829]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1830]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1832]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1833]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1834]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1836]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1837]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1839]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1840]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1841]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1843]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1844]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1846]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "\u{2006}", ")", "41n", "B"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1847]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1849]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1850]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1852]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1853]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1854]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1856]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1857]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1858]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1859]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1860]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1861]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1862]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1863]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1864]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1865]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1866]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1867]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1868]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "S", "-mª", "Ⰶ", "쐈{"]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1869]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1870]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1871]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1872]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1873]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1874]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1875]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1876]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1877]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1878]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1879]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1880]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1881]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1882]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1883]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1884]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1885]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1886]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1887]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", "\u{2067}\u{8f}", "", "¦["]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1888]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1889]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1890]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1891]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1892]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1893]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1894]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1895]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1896]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1897]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1898]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1899]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1900]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1901]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1902]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1903]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1904]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1905]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1906]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", "", "", "¦["]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1907]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1908]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1909]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1910]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1911]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1912]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1913]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1914]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1915]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1916]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1917]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1918]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1919]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1920]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1921]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1922]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1923]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1924]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1925]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", "", "", ""]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1926]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1927]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1928]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1929]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1930]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1931]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1932]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1933]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1934]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1935]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1936]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1937]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1938]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1939]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1940]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1941]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1942]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1943]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1944]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1945]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", "", ""]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1946]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1947]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1948]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1949]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1950]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1951]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1952]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1953]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1954]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1955]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1956]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1957]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1958]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1959]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1960]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1961]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1962]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1963]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1964]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1965]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", "", ""]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1966]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1967]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1968]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1969]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1970]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1971]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1972]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1973]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1974]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1975]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1976]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1977]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1978]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1979]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1980]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1981]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1982]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1983]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1984]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1985]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}", ""]]`', tests/test_cat.rs:86:9
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1986]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1987]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1988]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1989]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1990]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1991]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1992]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1993]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1994]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1995]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1996]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1997]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1998]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-1999]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-2000]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-2001]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-2002]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
[/checkout/obj/build/ct/xsv/target/debug/xit/cat_cols/test-2003]: "/checkout/obj/build/ct/xsv/target/debug/xsv" "cat" "columns" "in1.csv" "in2.csv" "--no-headers"
thread 'test_cat::prop_cat_cols' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[239, 187, 191]]] }, CsvData { data: [[[]]] })
Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\\u{feff}\", \"\"]]`"', /cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-0.7.1/src/tester.rs:176:28

failures:
    test_cat::prop_cat_cols

---
expected success, got: exit status: 101


Build completed unsuccessfully in 0:29:43
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
