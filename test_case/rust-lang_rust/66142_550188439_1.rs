\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/lint_without_lint_pass.rs","byte_start":142,"byte_end":168,"line_start":9,"line_end":9,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/lint_without_lint_pass.rs:9:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T07:13:13.5239991Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T07:13:13.5240065Z 
2019-11-06T07:13:13.5240285Z ------------------------------------------
2019-11-06T07:13:13.5240318Z 
---
2019-11-06T07:13:13.5242036Z expected stderr:
2019-11-06T07:13:13.5242099Z error: trivial regex
2019-11-06T07:13:13.5242341Z   --> $DIR/regex.rs:13:45
2019-11-06T07:13:13.5242385Z    |
2019-11-06T07:13:13.5242429Z LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T07:13:13.5242535Z    |
2019-11-06T07:13:13.5242770Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T07:13:13.5242837Z    = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5242869Z 
2019-11-06T07:13:13.5242869Z 
2019-11-06T07:13:13.5242909Z error: trivial regex
2019-11-06T07:13:13.5243108Z   --> $DIR/regex.rs:14:60
2019-11-06T07:13:13.5243166Z    |
2019-11-06T07:13:13.5243211Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T07:13:13.5243319Z    |
2019-11-06T07:13:13.5243372Z    = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5243410Z 
2019-11-06T07:13:13.5243410Z 
2019-11-06T07:13:13.5243457Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T07:13:13.5243727Z    |
2019-11-06T07:13:13.5243727Z    |
2019-11-06T07:13:13.5243949Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T07:13:13.5244057Z    |
2019-11-06T07:13:13.5244057Z    |
2019-11-06T07:13:13.5244290Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T07:13:13.5244323Z 
2019-11-06T07:13:13.5244386Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T07:13:13.5244633Z    |
2019-11-06T07:13:13.5244633Z    |
2019-11-06T07:13:13.5244873Z LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T07:13:13.5244952Z 
2019-11-06T07:13:13.5245022Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5245235Z   --> $DIR/regex.rs:18:33
2019-11-06T07:13:13.5245278Z    |
2019-11-06T07:13:13.5245278Z    |
2019-11-06T07:13:13.5245322Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T07:13:13.5245417Z 
2019-11-06T07:13:13.5245457Z error: trivial regex
2019-11-06T07:13:13.5245661Z   --> $DIR/regex.rs:20:53
2019-11-06T07:13:13.5245720Z    |
2019-11-06T07:13:13.5245720Z    |
2019-11-06T07:13:13.5245763Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T07:13:13.5245868Z    |
2019-11-06T07:13:13.5245911Z    = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5245941Z 
2019-11-06T07:13:13.5245999Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5245999Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5246204Z   --> $DIR/regex.rs:21:41
2019-11-06T07:13:13.5246246Z    |
2019-11-06T07:13:13.5246298Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T07:13:13.5246400Z 
2019-11-06T07:13:13.5246445Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5246952Z   --> $DIR/regex.rs:22:56
2019-11-06T07:13:13.5247008Z    |
2019-11-06T07:13:13.5247008Z    |
2019-11-06T07:13:13.5247055Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T07:13:13.5247156Z 
2019-11-06T07:13:13.5247199Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5247412Z   --> $DIR/regex.rs:34:37
2019-11-06T07:13:13.5247470Z    |
2019-11-06T07:13:13.5247470Z    |
2019-11-06T07:13:13.5247725Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T07:13:13.5247805Z 
2019-11-06T07:13:13.5248001Z error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5248310Z   --> $DIR/regex.rs:35:39
2019-11-06T07:13:13.5248359Z    |
2019-11-06T07:13:13.5248359Z    |
2019-11-06T07:13:13.5248650Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T07:13:13.5248735Z 
2019-11-06T07:13:13.5248778Z error: regex syntax error: unrecognized escape sequence
2019-11-06T07:13:13.5248999Z   --> $DIR/regex.rs:37:45
2019-11-06T07:13:13.5249041Z    |
2019-11-06T07:13:13.5249041Z    |
2019-11-06T07:13:13.5249084Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T07:13:13.5249177Z 
2019-11-06T07:13:13.5249220Z error: regex syntax error: unrecognized escape sequence
2019-11-06T07:13:13.5249424Z   --> $DIR/regex.rs:38:46
2019-11-06T07:13:13.5249483Z    |
2019-11-06T07:13:13.5249483Z    |
2019-11-06T07:13:13.5249527Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T07:13:13.5249636Z 
2019-11-06T07:13:13.5249676Z error: trivial regex
2019-11-06T07:13:13.5249878Z   --> $DIR/regex.rs:42:33
2019-11-06T07:13:13.5249920Z    |
2019-11-06T07:13:13.5249920Z    |
2019-11-06T07:13:13.5249979Z LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T07:13:13.5250066Z    |
2019-11-06T07:13:13.5250066Z    |
2019-11-06T07:13:13.5250123Z    = help: consider using `==` on `str`s
2019-11-06T07:13:13.5250191Z error: trivial regex
2019-11-06T07:13:13.5250393Z   --> $DIR/regex.rs:44:48
2019-11-06T07:13:13.5250450Z    |
2019-11-06T07:13:13.5250450Z    |
2019-11-06T07:13:13.5250495Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T07:13:13.5250603Z    |
2019-11-06T07:13:13.5250603Z    |
2019-11-06T07:13:13.5250645Z    = help: consider using `==` on `str`s
2019-11-06T07:13:13.5250711Z error: trivial regex
2019-11-06T07:13:13.5250940Z   --> $DIR/regex.rs:46:42
2019-11-06T07:13:13.5250990Z    |
2019-11-06T07:13:13.5250990Z    |
2019-11-06T07:13:13.5251033Z LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T07:13:13.5251147Z    |
2019-11-06T07:13:13.5251189Z    = help: consider using `str::starts_with`
2019-11-06T07:13:13.5251219Z 
2019-11-06T07:13:13.5251274Z error: trivial regex
2019-11-06T07:13:13.5251274Z error: trivial regex
2019-11-06T07:13:13.5251482Z   --> $DIR/regex.rs:48:40
2019-11-06T07:13:13.5251523Z    |
2019-11-06T07:13:13.5251582Z LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T07:13:13.5251714Z    |
2019-11-06T07:13:13.5251772Z    = help: consider using `str::ends_with`
2019-11-06T07:13:13.5251801Z 
2019-11-06T07:13:13.5251839Z error: trivial regex
---
2019-11-06T07:13:13.5252347Z 
2019-11-06T07:13:13.5252386Z error: trivial regex
2019-11-06T07:13:13.5252607Z   --> $DIR/regex.rs:52:39
2019-11-06T07:13:13.5252651Z    |
2019-11-06T07:13:13.5252696Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T07:13:13.5252804Z    |
2019-11-06T07:13:13.5252846Z    = help: consider using `str::contains`
2019-11-06T07:13:13.5252875Z 
2019-11-06T07:13:13.5252930Z error: trivial regex
2019-11-06T07:13:13.5252930Z error: trivial regex
2019-11-06T07:13:13.5253132Z   --> $DIR/regex.rs:54:40
2019-11-06T07:13:13.5253173Z    |
2019-11-06T07:13:13.5253216Z LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T07:13:13.5253413Z    |
2019-11-06T07:13:13.5253505Z    = help: consider using `str::contains`
2019-11-06T07:13:13.5253555Z 
2019-11-06T07:13:13.5253595Z error: trivial regex
---
2019-11-06T07:13:13.5254104Z 
2019-11-06T07:13:13.5254143Z error: trivial regex
2019-11-06T07:13:13.5254347Z   --> $DIR/regex.rs:59:36
2019-11-06T07:13:13.5254408Z    |
2019-11-06T07:13:13.5254450Z LL |     let trivial_empty = Regex::new("^");
2019-11-06T07:13:13.5254553Z    |
2019-11-06T07:13:13.5254598Z    = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5254628Z 
2019-11-06T07:13:13.5254675Z error: trivial regex
2019-11-06T07:13:13.5254675Z error: trivial regex
2019-11-06T07:13:13.5254895Z   --> $DIR/regex.rs:61:36
2019-11-06T07:13:13.5254946Z    |
2019-11-06T07:13:13.5254988Z LL |     let trivial_empty = Regex::new("^$");
2019-11-06T07:13:13.5255091Z    |
2019-11-06T07:13:13.5255133Z    = help: consider using `str::is_empty`
2019-11-06T07:13:13.5255161Z 
2019-11-06T07:13:13.5255216Z error: trivial regex
2019-11-06T07:13:13.5255216Z error: trivial regex
2019-11-06T07:13:13.5255419Z   --> $DIR/regex.rs:63:44
2019-11-06T07:13:13.5255462Z    |
2019-11-06T07:13:13.5255521Z LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T07:13:13.5255609Z    |
2019-11-06T07:13:13.5255650Z    = help: consider using `str::is_empty`
2019-11-06T07:13:13.5255695Z 
2019-11-06T07:13:13.5255736Z error: aborting due to 23 previous errors
---
2019-11-06T07:13:13.5256332Z -  --> $DIR/regex.rs:13:45
2019-11-06T07:13:13.5256554Z +error[E0463]: can't find crate for `regex`
2019-11-06T07:13:13.5257027Z +  --> $DIR/regex.rs:4:1
2019-11-06T07:13:13.5257073Z     |
2019-11-06T07:13:13.5257322Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T07:13:13.5257729Z -   |
2019-11-06T07:13:13.5257977Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T07:13:13.5258210Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5258257Z +LL | extern crate regex;
2019-11-06T07:13:13.5258257Z +LL | extern crate regex;
2019-11-06T07:13:13.5258484Z +   | ^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T07:13:13.5258528Z  
2019-11-06T07:13:13.5258722Z -error: trivial regex
2019-11-06T07:13:13.5258921Z -  --> $DIR/regex.rs:14:60
2019-11-06T07:13:13.5259119Z -   |
2019-11-06T07:13:13.5259372Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T07:13:13.5259823Z -   |
2019-11-06T07:13:13.5260047Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5260094Z +error: aborting due to previous error
2019-11-06T07:13:13.5260151Z  
2019-11-06T07:13:13.5260151Z  
2019-11-06T07:13:13.5260408Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T07:13:13.5260818Z -   |
2019-11-06T07:13:13.5260818Z -   |
2019-11-06T07:13:13.5261044Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T07:13:13.5261458Z -   |
2019-11-06T07:13:13.5261458Z -   |
2019-11-06T07:13:13.5261693Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T07:13:13.5261869Z -
2019-11-06T07:13:13.5262119Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T07:13:13.5262740Z -   |
2019-11-06T07:13:13.5262740Z -   |
2019-11-06T07:13:13.5262992Z -LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T07:13:13.5263406Z -
2019-11-06T07:13:13.5263629Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5263845Z -  --> $DIR/regex.rs:18:33
2019-11-06T07:13:13.5264025Z -   |
2019-11-06T07:13:13.5264025Z -   |
2019-11-06T07:13:13.5264243Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T07:13:13.5264658Z -
2019-11-06T07:13:13.5264846Z -error: trivial regex
2019-11-06T07:13:13.5265043Z -  --> $DIR/regex.rs:20:53
2019-11-06T07:13:13.5265240Z -   |
2019-11-06T07:13:13.5265240Z -   |
2019-11-06T07:13:13.5265468Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T07:13:13.5265908Z -   |
2019-11-06T07:13:13.5266134Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5266317Z -
2019-11-06T07:13:13.5266541Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5266541Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5267031Z -  --> $DIR/regex.rs:21:41
2019-11-06T07:13:13.5267210Z -   |
2019-11-06T07:13:13.5267438Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T07:13:13.5267865Z -
2019-11-06T07:13:13.5268087Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5268304Z -  --> $DIR/regex.rs:22:56
2019-11-06T07:13:13.5268485Z -   |
2019-11-06T07:13:13.5268485Z -   |
2019-11-06T07:13:13.5268728Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T07:13:13.5269170Z -
2019-11-06T07:13:13.5269391Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5269602Z -  --> $DIR/regex.rs:34:37
2019-11-06T07:13:13.5269802Z -   |
2019-11-06T07:13:13.5269802Z -   |
2019-11-06T07:13:13.5270059Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T07:13:13.5270482Z -
2019-11-06T07:13:13.5270706Z -error: regex syntax error on position 0: unclosed group
2019-11-06T07:13:13.5270906Z -  --> $DIR/regex.rs:35:39
2019-11-06T07:13:13.5271098Z -   |
2019-11-06T07:13:13.5271098Z -   |
2019-11-06T07:13:13.5271352Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T07:13:13.5271771Z -
2019-11-06T07:13:13.5271996Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T07:13:13.5272194Z -  --> $DIR/regex.rs:37:45
2019-11-06T07:13:13.5272370Z -   |
2019-11-06T07:13:13.5272370Z -   |
2019-11-06T07:13:13.5272616Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T07:13:13.5273022Z -
2019-11-06T07:13:13.5273274Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T07:13:13.5273476Z -  --> $DIR/regex.rs:38:46
2019-11-06T07:13:13.5273651Z -   |
2019-11-06T07:13:13.5273651Z -   |
2019-11-06T07:13:13.5273895Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T07:13:13.5274299Z -
2019-11-06T07:13:13.5274487Z -error: trivial regex
2019-11-06T07:13:13.5274706Z -  --> $DIR/regex.rs:42:33
2019-11-06T07:13:13.5274883Z -   |
2019-11-06T07:13:13.5274883Z -   |
2019-11-06T07:13:13.5275099Z -LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T07:13:13.5275514Z -   |
2019-11-06T07:13:13.5275514Z -   |
2019-11-06T07:13:13.5275722Z -   = help: consider using `==` on `str`s
2019-11-06T07:13:13.5276105Z -error: trivial regex
2019-11-06T07:13:13.5276303Z -  --> $DIR/regex.rs:44:48
2019-11-06T07:13:13.5277191Z -   |
2019-11-06T07:13:13.5277191Z -   |
2019-11-06T07:13:13.5277547Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T07:13:13.5277996Z -   |
2019-11-06T07:13:13.5277996Z -   |
2019-11-06T07:13:13.5278227Z -   = help: consider using `==` on `str`s
2019-11-06T07:13:13.5278589Z -error: trivial regex
2019-11-06T07:13:13.5278806Z -  --> $DIR/regex.rs:46:42
2019-11-06T07:13:13.5278988Z -   |
2019-11-06T07:13:13.5278988Z -   |
2019-11-06T07:13:13.5279213Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T07:13:13.5279637Z -   |
2019-11-06T07:13:13.5279851Z -   = help: consider using `str::starts_with`
2019-11-06T07:13:13.5280026Z -
2019-11-06T07:13:13.5280237Z -error: trivial regex
2019-11-06T07:13:13.5280237Z -error: trivial regex
2019-11-06T07:13:13.5280434Z -  --> $DIR/regex.rs:48:40
2019-11-06T07:13:13.5280611Z -   |
2019-11-06T07:13:13.5280849Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T07:13:13.5281277Z -   |
2019-11-06T07:13:13.5281489Z -   = help: consider using `str::ends_with`
2019-11-06T07:13:13.5281685Z -
2019-11-06T07:13:13.5281873Z -error: trivial regex
---
2019-11-06T07:13:13.5283293Z -
2019-11-06T07:13:13.5283480Z -error: trivial regex
2019-11-06T07:13:13.5283699Z -  --> $DIR/regex.rs:52:39
2019-11-06T07:13:13.5283876Z -   |
2019-11-06T07:13:13.5284106Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T07:13:13.5284547Z -   |
2019-11-06T07:13:13.5284755Z -   = help: consider using `str::contains`
2019-11-06T07:13:13.5284953Z -
2019-11-06T07:13:13.5285148Z -error: trivial regex
2019-11-06T07:13:13.5285148Z -error: trivial regex
2019-11-06T07:13:13.5285345Z -  --> $DIR/regex.rs:54:40
2019-11-06T07:13:13.5285521Z -   |
2019-11-06T07:13:13.5285760Z -LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T07:13:13.5286161Z -   |
2019-11-06T07:13:13.5286391Z -   = help: consider using `str::contains`
2019-11-06T07:13:13.5286566Z -
2019-11-06T07:13:13.5287034Z -error: trivial regex
---
2019-11-06T07:13:13.5288468Z -
2019-11-06T07:13:13.5288687Z -error: trivial regex
2019-11-06T07:13:13.5288888Z -  --> $DIR/regex.rs:59:36
2019-11-06T07:13:13.5289074Z -   |
2019-11-06T07:13:13.5289307Z -LL |     let trivial_empty = Regex::new("^");
2019-11-06T07:13:13.5289703Z -   |
2019-11-06T07:13:13.5289927Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T07:13:13.5290123Z -
2019-11-06T07:13:13.5290313Z -error: trivial regex
2019-11-06T07:13:13.5290313Z -error: trivial regex
2019-11-06T07:13:13.5290511Z -  --> $DIR/regex.rs:61:36
2019-11-06T07:13:13.5290706Z -   |
2019-11-06T07:13:13.5290926Z -LL |     let trivial_empty = Regex::new("^$");
2019-11-06T07:13:13.5291339Z -   |
2019-11-06T07:13:13.5291554Z -   = help: consider using `str::is_empty`
2019-11-06T07:13:13.5291729Z -
2019-11-06T07:13:13.5291919Z -error: trivial regex
2019-11-06T07:13:13.5291919Z -error: trivial regex
2019-11-06T07:13:13.5292136Z -  --> $DIR/regex.rs:63:44
2019-11-06T07:13:13.5292315Z -   |
2019-11-06T07:13:13.5292536Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T07:13:13.5293206Z -   |
2019-11-06T07:13:13.5293418Z -   = help: consider using `str::is_empty`
2019-11-06T07:13:13.5293593Z -
2019-11-06T07:13:13.5293826Z -error: aborting due to 23 previous errors
---
2019-11-06T07:13:13.5297151Z 
2019-11-06T07:13:13.5297384Z ------------------------------------------
2019-11-06T07:13:13.5297430Z stderr:
2019-11-06T07:13:13.5297641Z ------------------------------------------
2019-11-06T07:13:13.5298879Z {"message":"can't find crate for `regex`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n