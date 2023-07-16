plain

failures:

---- test_reverse::prop_reverse_headers stdout ----
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-773]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-779]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-786]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["¢\u{10fffe}\u{e}F", "\u{1a}"], ["\u{b73e2}I赶&", "佣 "], ["1\u{2063}", ""], ["\u{dd65a}\u{9b}@+", "`\u{19}\u{180e}"], ["", "$"], ["®", "5y8"], ["\u{9a}\u{6}\u{f}\u{202f}", "&0"], ["c", "x"], ["⎋-", "Xn¦\u{1e}"], ["", ">퍢"], ["\u{86}", "-"], ["", ""], ["\u{5c513}9*\u{82}", "\"klE"], ["\u{e}\u{16}\u{604}V", "0;\u{85c8c}\u{fffe}"], ["\u{f3c2b}-|{", ""], ["\u{1b}\\", "锉\u{fff8};"], ["", "of➴}"], ["", "ⴟ/n𛄔"], ["-\u{6}:h", "\u{dba71}"], ["_dk", "\u{2069}\u{ec5a}&"], ["", "\u{1d}"], ["\n/", ""], ["?", "¨\u{89}"], ["", "\u{fff8}¨_"], ["", "a"], ["\u{6}", "/\\¡¨"], ["", "\u{100f4e}핋&"], ["", "J0ª\u{ffffe}"], ["\u{fff7}", "\r\u{faeb}~<"], ["\u{4a36e}\u{4d44f}\u{92}\u{9b}", ""], ["-\u{2002}<𨇻", "®"], ["", "\u{8b}x"], ["\u{3}\u{86}䗄", "G\u{5}\u{8d}"], ["", "\u{8}\u{1b}"], ["}O", "3\u{92}ZK"], [" \u{601}\u{e167}", "\u{2065}\u{b49a7}\u{fff2}1"], ["", "050\u{5}"], ["j", ""], ["6/", "[\u{10fffd}"], ["\u{7f}u", "5\nB"], [" \u{e001}", "내"], ["{cHT", "LF⁃%"], ["", "}"], ["1𥩯-", "\u{ecb}#"], ["鐰", "\u{85}z\u{97}"], ["<䢖※\u{96}", "\u{f0000}$慰\u{2064}"], ["h", ""], ["<", "Ꮚ¢1"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["¢\u{10fffe}\u{e}F", "\u{1a}"], ["\u{b73e2}I赶&", "佣 "], ["1\u{2063}", ""], ["\u{dd65a}\u{9b}@+", "`\u{19}\u{180e}"], ["", "$"], ["®", "5y8"], ["\u{9a}\u{6}\u{f}\u{202f}", "&0"], ["c", "x"], ["⎋-", "Xn¦\u{1e}"], ["", ">퍢"], ["\u{86}", "-"], ["", ""], ["\u{5c513}9*\u{82}", "\"klE"], ["\u{e}\u{16}\u{604}V", "0;\u{85c8c}\u{fffe}"], ["\u{f3c2b}-|{", ""], ["\u{1b}\\", "锉\u{fff8};"], ["", "of➴}"], ["", "ⴟ/n𛄔"], ["-\u{6}:h", "\u{dba71}"], ["_dk", "\u{2069}\u{ec5a}&"], ["", "\u{1d}"], ["\n/", ""], ["?", "¨\u{89}"], ["", "\u{fff8}¨_"], ["", "a"], ["\u{6}", "/\\¡¨"], ["", "\u{100f4e}핋&"], ["", "J0ª\u{ffffe}"], ["\u{fff7}", "\r\u{faeb}~<"], ["\u{4a36e}\u{4d44f}\u{92}\u{9b}", ""], ["-\u{2002}<𨇻", "®"], ["", "\u{8b}x"], ["\u{3}\u{86}䗄", "G\u{5}\u{8d}"], ["", "\u{8}\u{1b}"], ["}O", "3\u{92}ZK"], [" \u{601}\u{e167}", "\u{2065}\u{b49a7}\u{fff2}1"], ["", "050\u{5}"], ["j", ""], ["6/", "[\u{10fffd}"], ["\u{7f}u", "5\nB"], [" \u{e001}", "내"], ["{cHT", "LF⁃%"], ["", "}"], ["1𥩯-", "\u{ecb}#"], ["鐰", "\u{85}z\u{97}"], ["<䢖※\u{96}", "\u{f0000}$慰\u{2064}"], ["h", ""], ["<", "Ꮚ¢1"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`', tests\test_reverse.rs:22:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-825]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-831]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-837]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{3}\u{86}䗄", "G\u{5}\u{8d}"], ["", "\u{8}\u{1b}"], ["}O", "3\u{92}ZK"], [" \u{601}\u{e167}", "\u{2065}\u{b49a7}\u{fff2}1"], ["", "050\u{5}"], ["j", ""], ["6/", "[\u{10fffd}"], ["\u{7f}u", "5\nB"], [" \u{e001}", "내"], ["{cHT", "LF⁃%"], ["", "}"], ["1𥩯-", "\u{ecb}#"], ["鐰", "\u{85}z\u{97}"], ["<䢖※\u{96}", "\u{f0000}$慰\u{2064}"], ["h", ""], ["<", "Ꮚ¢1"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{3}\u{86}䗄", "G\u{5}\u{8d}"], ["", "\u{8}\u{1b}"], ["}O", "3\u{92}ZK"], [" \u{601}\u{e167}", "\u{2065}\u{b49a7}\u{fff2}1"], ["", "050\u{5}"], ["j", ""], ["6/", "[\u{10fffd}"], ["\u{7f}u", "5\nB"], [" \u{e001}", "내"], ["{cHT", "LF⁃%"], ["", "}"], ["1𥩯-", "\u{ecb}#"], ["鐰", "\u{85}z\u{97}"], ["<䢖※\u{96}", "\u{f0000}$慰\u{2064}"], ["h", ""], ["<", "Ꮚ¢1"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-851]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-856]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-863]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], [">", "\u{fff5}"], ["", "\u{5a6be}\u{8}!"], ["4q,", ""], ["㵟s", ">G"], ["ª", ""], ["", ":/"], [" ", "\n_#"], ["ⶂ", "R¢"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-870]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-876]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-882]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{b7895} 0", "ᦤ\u{19}\u{8c}¢"], ["'q\u{88}", "\u{ed08d}\u{9d}"], ["⁋\u{a0}", "\u{100000}鏒a"], ["\u{1b}/\t=", "~"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-889]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-895]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-901]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["\u{e}\u{96}(", "2"], ["", "(*O)"], ["8", "t"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-906]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-912]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-919]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["8", "t"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"], ["8", "t"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-924]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-930]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-936]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"], ["x", "*/"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-943]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-949]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-955]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}K=", "\u{1};\u{100000}\u{2}"]]`,
 right: `[["\u{feff}\u{fffff}K=", "\u{1};\u{100000}\u{2}"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-960]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-966]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-973]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-977]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["\u{fffff}", "\u{1};\u{100000}\u{2}"]]`,
 right: `[["\u{feff}\u{fffff}", "\u{1};\u{100000}\u{2}"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-984]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-989]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-996]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1000]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["", "\u{1};\u{100000}\u{2}"]]`,
 right: `[["\u{feff}", "\u{1};\u{100000}\u{2}"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1007]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1011]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1015]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1023]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1028]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1034]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1039]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1046]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1050]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1058]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1063]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1068]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1074]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1080]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1084]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1090]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1094]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1099]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[["", ""]]`,
 right: `[["\u{feff}", ""]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1106]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1111]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1117]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1123]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1130]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1137]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1141]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1149]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1153]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1160]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1165]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1172]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1178]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1185]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1191]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1198]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1204]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1211]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1216]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1223]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at 'assertion failed: `(left == right)`
  left: `[]`,
 right: `[["\u{feff}"]]`', tests\test_reverse.rs:22:5
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1229]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1237]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1242]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1250]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1255]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1262]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1268]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1275]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1281]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1288]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1295]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1302]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1309]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1315]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1320]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1327]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
[D:\a\rust\rust\build\ct\xsv\target\debug\xit\prop_reverse_headers\test-1334]: "D:\\a\\rust\\rust\\build\\ct\\xsv\\target\\debug\\xsv" "reverse" "in.csv"
thread 'test_reverse::prop_reverse_headers' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (CsvData { data: [[[239, 187, 191]]] })
Error: "assertion failed: `(left == right)`\n  left: `[]`,\n right: `[[\"\\u{feff}\"]]`"', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\quickcheck-0.7.1\src\tester.rs:176:28

failures:
    test_reverse::prop_reverse_headers

