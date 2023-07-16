\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/lint_without_lint_pass.rs","byte_start":142,"byte_end":168,"line_start":9,"line_end":9,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/lint_without_lint_pass.rs:9:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T17:25:15.9616835Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T17:25:15.9616908Z 
2019-11-06T17:25:15.9617133Z ------------------------------------------
2019-11-06T17:25:15.9617166Z 
---
2019-11-06T17:25:15.9618696Z expected stderr:
2019-11-06T17:25:15.9618739Z error: trivial regex
2019-11-06T17:25:15.9618963Z   --> $DIR/regex.rs:13:45
2019-11-06T17:25:15.9619006Z    |
2019-11-06T17:25:15.9619050Z LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T17:25:15.9619160Z    |
2019-11-06T17:25:15.9619393Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T17:25:15.9619462Z    = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9619494Z 
2019-11-06T17:25:15.9619494Z 
2019-11-06T17:25:15.9619543Z error: trivial regex
2019-11-06T17:25:15.9619745Z   --> $DIR/regex.rs:14:60
2019-11-06T17:25:15.9619806Z    |
2019-11-06T17:25:15.9619853Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T17:25:15.9619967Z    |
2019-11-06T17:25:15.9620010Z    = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9620048Z 
2019-11-06T17:25:15.9620048Z 
2019-11-06T17:25:15.9620096Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T17:25:15.9620369Z    |
2019-11-06T17:25:15.9620369Z    |
2019-11-06T17:25:15.9620589Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T17:25:15.9620698Z    |
2019-11-06T17:25:15.9620698Z    |
2019-11-06T17:25:15.9620928Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T17:25:15.9620961Z 
2019-11-06T17:25:15.9621029Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T17:25:15.9621285Z    |
2019-11-06T17:25:15.9621285Z    |
2019-11-06T17:25:15.9621527Z LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T17:25:15.9621607Z 
2019-11-06T17:25:15.9621650Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9621957Z   --> $DIR/regex.rs:18:33
2019-11-06T17:25:15.9622009Z    |
2019-11-06T17:25:15.9622009Z    |
2019-11-06T17:25:15.9622054Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T17:25:15.9622150Z 
2019-11-06T17:25:15.9622191Z error: trivial regex
2019-11-06T17:25:15.9622412Z   --> $DIR/regex.rs:20:53
2019-11-06T17:25:15.9622474Z    |
2019-11-06T17:25:15.9622474Z    |
2019-11-06T17:25:15.9622518Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T17:25:15.9622702Z    |
2019-11-06T17:25:15.9622747Z    = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9622776Z 
2019-11-06T17:25:15.9622820Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9622820Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9623059Z   --> $DIR/regex.rs:21:41
2019-11-06T17:25:15.9623102Z    |
2019-11-06T17:25:15.9623146Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T17:25:15.9623252Z 
2019-11-06T17:25:15.9623295Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9623498Z   --> $DIR/regex.rs:22:56
2019-11-06T17:25:15.9623560Z    |
2019-11-06T17:25:15.9623560Z    |
2019-11-06T17:25:15.9623607Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T17:25:15.9623709Z 
2019-11-06T17:25:15.9623753Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9623964Z   --> $DIR/regex.rs:34:37
2019-11-06T17:25:15.9624006Z    |
2019-11-06T17:25:15.9624006Z    |
2019-11-06T17:25:15.9624275Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T17:25:15.9624596Z 
2019-11-06T17:25:15.9624676Z error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9624941Z   --> $DIR/regex.rs:35:39
2019-11-06T17:25:15.9624985Z    |
2019-11-06T17:25:15.9624985Z    |
2019-11-06T17:25:15.9625241Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T17:25:15.9625344Z 
2019-11-06T17:25:15.9625388Z error: regex syntax error: unrecognized escape sequence
2019-11-06T17:25:15.9625611Z   --> $DIR/regex.rs:37:45
2019-11-06T17:25:15.9625653Z    |
2019-11-06T17:25:15.9625653Z    |
2019-11-06T17:25:15.9625698Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T17:25:15.9625802Z 
2019-11-06T17:25:15.9625845Z error: regex syntax error: unrecognized escape sequence
2019-11-06T17:25:15.9626051Z   --> $DIR/regex.rs:38:46
2019-11-06T17:25:15.9626111Z    |
2019-11-06T17:25:15.9626111Z    |
2019-11-06T17:25:15.9626156Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T17:25:15.9626241Z 
2019-11-06T17:25:15.9626301Z error: trivial regex
2019-11-06T17:25:15.9626503Z   --> $DIR/regex.rs:42:33
2019-11-06T17:25:15.9626573Z    |
2019-11-06T17:25:15.9626573Z    |
2019-11-06T17:25:15.9626637Z LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T17:25:15.9626725Z    |
2019-11-06T17:25:15.9626725Z    |
2019-11-06T17:25:15.9626786Z    = help: consider using `==` on `str`s
2019-11-06T17:25:15.9626856Z error: trivial regex
2019-11-06T17:25:15.9627067Z   --> $DIR/regex.rs:44:48
2019-11-06T17:25:15.9627127Z    |
2019-11-06T17:25:15.9627127Z    |
2019-11-06T17:25:15.9627182Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T17:25:15.9627293Z    |
2019-11-06T17:25:15.9627293Z    |
2019-11-06T17:25:15.9627335Z    = help: consider using `==` on `str`s
2019-11-06T17:25:15.9627404Z error: trivial regex
2019-11-06T17:25:15.9627629Z   --> $DIR/regex.rs:46:42
2019-11-06T17:25:15.9627780Z    |
2019-11-06T17:25:15.9627780Z    |
2019-11-06T17:25:15.9627830Z LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T17:25:15.9627939Z    |
2019-11-06T17:25:15.9627981Z    = help: consider using `str::starts_with`
2019-11-06T17:25:15.9628011Z 
2019-11-06T17:25:15.9628070Z error: trivial regex
2019-11-06T17:25:15.9628070Z error: trivial regex
2019-11-06T17:25:15.9628299Z   --> $DIR/regex.rs:48:40
2019-11-06T17:25:15.9628342Z    |
2019-11-06T17:25:15.9628385Z LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T17:25:15.9628569Z    |
2019-11-06T17:25:15.9628612Z    = help: consider using `str::ends_with`
2019-11-06T17:25:15.9628659Z 
2019-11-06T17:25:15.9628699Z error: trivial regex
---
2019-11-06T17:25:15.9629213Z 
2019-11-06T17:25:15.9629253Z error: trivial regex
2019-11-06T17:25:15.9629457Z   --> $DIR/regex.rs:52:39
2019-11-06T17:25:15.9629519Z    |
2019-11-06T17:25:15.9629564Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T17:25:15.9629676Z    |
2019-11-06T17:25:15.9629717Z    = help: consider using `str::contains`
2019-11-06T17:25:15.9629754Z 
2019-11-06T17:25:15.9629794Z error: trivial regex
2019-11-06T17:25:15.9629794Z error: trivial regex
2019-11-06T17:25:15.9630015Z   --> $DIR/regex.rs:54:40
2019-11-06T17:25:15.9630058Z    |
2019-11-06T17:25:15.9630100Z LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T17:25:15.9630208Z    |
2019-11-06T17:25:15.9630257Z    = help: consider using `str::contains`
2019-11-06T17:25:15.9630286Z 
2019-11-06T17:25:15.9630346Z error: trivial regex
---
2019-11-06T17:25:15.9630832Z 
2019-11-06T17:25:15.9630872Z error: trivial regex
2019-11-06T17:25:15.9631072Z   --> $DIR/regex.rs:59:36
2019-11-06T17:25:15.9631140Z    |
2019-11-06T17:25:15.9631183Z LL |     let trivial_empty = Regex::new("^");
2019-11-06T17:25:15.9631270Z    |
2019-11-06T17:25:15.9631332Z    = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9631362Z 
2019-11-06T17:25:15.9631402Z error: trivial regex
2019-11-06T17:25:15.9631402Z error: trivial regex
2019-11-06T17:25:15.9631625Z   --> $DIR/regex.rs:61:36
2019-11-06T17:25:15.9631676Z    |
2019-11-06T17:25:15.9631719Z LL |     let trivial_empty = Regex::new("^$");
2019-11-06T17:25:15.9631826Z    |
2019-11-06T17:25:15.9631867Z    = help: consider using `str::is_empty`
2019-11-06T17:25:15.9631896Z 
2019-11-06T17:25:15.9631957Z error: trivial regex
2019-11-06T17:25:15.9631957Z error: trivial regex
2019-11-06T17:25:15.9632161Z   --> $DIR/regex.rs:63:44
2019-11-06T17:25:15.9632204Z    |
2019-11-06T17:25:15.9632246Z LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T17:25:15.9632361Z    |
2019-11-06T17:25:15.9632403Z    = help: consider using `str::is_empty`
2019-11-06T17:25:15.9632450Z 
2019-11-06T17:25:15.9632493Z error: aborting due to 23 previous errors
---
2019-11-06T17:25:15.9633152Z -  --> $DIR/regex.rs:13:45
2019-11-06T17:25:15.9633395Z +error[E0463]: can't find crate for `regex`
2019-11-06T17:25:15.9633595Z +  --> $DIR/regex.rs:4:1
2019-11-06T17:25:15.9633639Z     |
2019-11-06T17:25:15.9633881Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T17:25:15.9634290Z -   |
2019-11-06T17:25:15.9634541Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T17:25:15.9634772Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9634889Z +LL | extern crate regex;
2019-11-06T17:25:15.9634889Z +LL | extern crate regex;
2019-11-06T17:25:15.9635119Z +   | ^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T17:25:15.9635183Z  
2019-11-06T17:25:15.9635378Z -error: trivial regex
2019-11-06T17:25:15.9635577Z -  --> $DIR/regex.rs:14:60
2019-11-06T17:25:15.9635776Z -   |
2019-11-06T17:25:15.9636013Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T17:25:15.9638102Z -   |
2019-11-06T17:25:15.9639007Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9639076Z +error: aborting due to previous error
2019-11-06T17:25:15.9639145Z  
2019-11-06T17:25:15.9639145Z  
2019-11-06T17:25:15.9639676Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T17:25:15.9640188Z -   |
2019-11-06T17:25:15.9640188Z -   |
2019-11-06T17:25:15.9640421Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T17:25:15.9640846Z -   |
2019-11-06T17:25:15.9640846Z -   |
2019-11-06T17:25:15.9641102Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T17:25:15.9641281Z -
2019-11-06T17:25:15.9641534Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T17:25:15.9641948Z -   |
2019-11-06T17:25:15.9641948Z -   |
2019-11-06T17:25:15.9642176Z -LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T17:25:15.9642600Z -
2019-11-06T17:25:15.9642825Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9643027Z -  --> $DIR/regex.rs:18:33
2019-11-06T17:25:15.9643226Z -   |
2019-11-06T17:25:15.9643226Z -   |
2019-11-06T17:25:15.9643448Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T17:25:15.9643884Z -
2019-11-06T17:25:15.9644076Z -error: trivial regex
2019-11-06T17:25:15.9644276Z -  --> $DIR/regex.rs:20:53
2019-11-06T17:25:15.9644471Z -   |
2019-11-06T17:25:15.9644471Z -   |
2019-11-06T17:25:15.9644706Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T17:25:15.9645125Z -   |
2019-11-06T17:25:15.9645378Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9645560Z -
2019-11-06T17:25:15.9645785Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9645785Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9646007Z -  --> $DIR/regex.rs:21:41
2019-11-06T17:25:15.9646187Z -   |
2019-11-06T17:25:15.9646416Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T17:25:15.9647145Z -
2019-11-06T17:25:15.9647372Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9647591Z -  --> $DIR/regex.rs:22:56
2019-11-06T17:25:15.9647789Z -   |
2019-11-06T17:25:15.9647789Z -   |
2019-11-06T17:25:15.9648033Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T17:25:15.9648485Z -
2019-11-06T17:25:15.9648711Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9648913Z -  --> $DIR/regex.rs:34:37
2019-11-06T17:25:15.9649112Z -   |
2019-11-06T17:25:15.9649112Z -   |
2019-11-06T17:25:15.9649528Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T17:25:15.9650010Z -
2019-11-06T17:25:15.9650238Z -error: regex syntax error on position 0: unclosed group
2019-11-06T17:25:15.9650437Z -  --> $DIR/regex.rs:35:39
2019-11-06T17:25:15.9650632Z -   |
2019-11-06T17:25:15.9650632Z -   |
2019-11-06T17:25:15.9650888Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T17:25:15.9651404Z -
2019-11-06T17:25:15.9652617Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T17:25:15.9652841Z -  --> $DIR/regex.rs:37:45
2019-11-06T17:25:15.9653018Z -   |
2019-11-06T17:25:15.9653018Z -   |
2019-11-06T17:25:15.9653272Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T17:25:15.9653676Z -
2019-11-06T17:25:15.9653928Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T17:25:15.9654136Z -  --> $DIR/regex.rs:38:46
2019-11-06T17:25:15.9654315Z -   |
2019-11-06T17:25:15.9654315Z -   |
2019-11-06T17:25:15.9654544Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T17:25:15.9656306Z -
2019-11-06T17:25:15.9656756Z -error: trivial regex
2019-11-06T17:25:15.9657043Z -  --> $DIR/regex.rs:42:33
2019-11-06T17:25:15.9657224Z -   |
2019-11-06T17:25:15.9657224Z -   |
2019-11-06T17:25:15.9657444Z -LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T17:25:15.9658391Z -   |
2019-11-06T17:25:15.9658391Z -   |
2019-11-06T17:25:15.9658864Z -   = help: consider using `==` on `str`s
2019-11-06T17:25:15.9659269Z -error: trivial regex
2019-11-06T17:25:15.9659825Z -  --> $DIR/regex.rs:44:48
2019-11-06T17:25:15.9660013Z -   |
2019-11-06T17:25:15.9660013Z -   |
2019-11-06T17:25:15.9662588Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T17:25:15.9664665Z -   |
2019-11-06T17:25:15.9664665Z -   |
2019-11-06T17:25:15.9664936Z -   = help: consider using `==` on `str`s
2019-11-06T17:25:15.9665312Z -error: trivial regex
2019-11-06T17:25:15.9665514Z -  --> $DIR/regex.rs:46:42
2019-11-06T17:25:15.9665714Z -   |
2019-11-06T17:25:15.9665714Z -   |
2019-11-06T17:25:15.9665942Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T17:25:15.9667252Z -   |
2019-11-06T17:25:15.9667471Z -   = help: consider using `str::starts_with`
2019-11-06T17:25:15.9667650Z -
2019-11-06T17:25:15.9667860Z -error: trivial regex
2019-11-06T17:25:15.9667860Z -error: trivial regex
2019-11-06T17:25:15.9668062Z -  --> $DIR/regex.rs:48:40
2019-11-06T17:25:15.9668240Z -   |
2019-11-06T17:25:15.9668466Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T17:25:15.9668903Z -   |
2019-11-06T17:25:15.9669120Z -   = help: consider using `str::ends_with`
2019-11-06T17:25:15.9669318Z -
2019-11-06T17:25:15.9669509Z -error: trivial regex
---
2019-11-06T17:25:15.9670989Z -
2019-11-06T17:25:15.9671204Z -error: trivial regex
2019-11-06T17:25:15.9671451Z -  --> $DIR/regex.rs:52:39
2019-11-06T17:25:15.9671652Z -   |
2019-11-06T17:25:15.9671906Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T17:25:15.9672387Z -   |
2019-11-06T17:25:15.9672621Z -   = help: consider using `str::contains`
2019-11-06T17:25:15.9672969Z -
2019-11-06T17:25:15.9673714Z -error: trivial regex
2019-11-06T17:25:15.9673714Z -error: trivial regex
2019-11-06T17:25:15.9674257Z -  --> $DIR/regex.rs:54:40
2019-11-06T17:25:15.9674452Z -   |
2019-11-06T17:25:15.9674701Z -LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T17:25:15.9675110Z -   |
2019-11-06T17:25:15.9675342Z -   = help: consider using `str::contains`
2019-11-06T17:25:15.9675536Z -
2019-11-06T17:25:15.9675749Z -error: trivial regex
---
2019-11-06T17:25:15.9677763Z -
2019-11-06T17:25:15.9677970Z -error: trivial regex
2019-11-06T17:25:15.9678174Z -  --> $DIR/regex.rs:59:36
2019-11-06T17:25:15.9678361Z -   |
2019-11-06T17:25:15.9678579Z -LL |     let trivial_empty = Regex::new("^");
2019-11-06T17:25:15.9679001Z -   |
2019-11-06T17:25:15.9679223Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T17:25:15.9679419Z -
2019-11-06T17:25:15.9679613Z -error: trivial regex
2019-11-06T17:25:15.9679613Z -error: trivial regex
2019-11-06T17:25:15.9680535Z -  --> $DIR/regex.rs:61:36
2019-11-06T17:25:15.9680732Z -   |
2019-11-06T17:25:15.9680976Z -LL |     let trivial_empty = Regex::new("^$");
2019-11-06T17:25:15.9681981Z -   |
2019-11-06T17:25:15.9682303Z -   = help: consider using `str::is_empty`
2019-11-06T17:25:15.9682500Z -
2019-11-06T17:25:15.9682708Z -error: trivial regex
2019-11-06T17:25:15.9682708Z -error: trivial regex
2019-11-06T17:25:15.9684174Z -  --> $DIR/regex.rs:63:44
2019-11-06T17:25:15.9684935Z -   |
2019-11-06T17:25:15.9685172Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T17:25:15.9685629Z -   |
2019-11-06T17:25:15.9685840Z -   = help: consider using `str::is_empty`
2019-11-06T17:25:15.9686016Z -
2019-11-06T17:25:15.9686252Z -error: aborting due to 23 previous errors
---
2019-11-06T17:25:15.9689805Z 
2019-11-06T17:25:15.9690024Z ------------------------------------------
2019-11-06T17:25:15.9690087Z stderr:
2019-11-06T17:25:15.9690298Z ------------------------------------------
2019-11-06T17:25:15.9691526Z {"message":"can't find crate for `regex`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n