\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/lint_without_lint_pass.rs","byte_start":142,"byte_end":168,"line_start":9,"line_end":9,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/lint_without_lint_pass.rs:9:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T21:36:06.7080478Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T21:36:06.7080537Z 
2019-11-06T21:36:06.7080701Z ------------------------------------------
2019-11-06T21:36:06.7080734Z 
---
2019-11-06T21:36:06.7082862Z expected stderr:
2019-11-06T21:36:06.7082917Z error: trivial regex
2019-11-06T21:36:06.7083091Z   --> $DIR/regex.rs:13:45
2019-11-06T21:36:06.7083130Z    |
2019-11-06T21:36:06.7083185Z LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T21:36:06.7083344Z    |
2019-11-06T21:36:06.7083565Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T21:36:06.7083627Z    = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7083661Z 
2019-11-06T21:36:06.7083661Z 
2019-11-06T21:36:06.7083696Z error: trivial regex
2019-11-06T21:36:06.7083887Z   --> $DIR/regex.rs:14:60
2019-11-06T21:36:06.7083925Z    |
2019-11-06T21:36:06.7083965Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T21:36:06.7084067Z    |
2019-11-06T21:36:06.7084106Z    = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7084132Z 
2019-11-06T21:36:06.7084132Z 
2019-11-06T21:36:06.7084192Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T21:36:06.7084407Z    |
2019-11-06T21:36:06.7084407Z    |
2019-11-06T21:36:06.7084769Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T21:36:06.7084865Z    |
2019-11-06T21:36:06.7084865Z    |
2019-11-06T21:36:06.7085377Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T21:36:06.7085426Z 
2019-11-06T21:36:06.7085467Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T21:36:06.7085678Z    |
2019-11-06T21:36:06.7085678Z    |
2019-11-06T21:36:06.7085855Z LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T21:36:06.7085919Z 
2019-11-06T21:36:06.7085969Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7086127Z   --> $DIR/regex.rs:18:33
2019-11-06T21:36:06.7086160Z    |
2019-11-06T21:36:06.7086160Z    |
2019-11-06T21:36:06.7086211Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T21:36:06.7086281Z 
2019-11-06T21:36:06.7086313Z error: trivial regex
2019-11-06T21:36:06.7087303Z   --> $DIR/regex.rs:20:53
2019-11-06T21:36:06.7087357Z    |
2019-11-06T21:36:06.7087357Z    |
2019-11-06T21:36:06.7087402Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T21:36:06.7087525Z    |
2019-11-06T21:36:06.7087570Z    = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7087600Z 
2019-11-06T21:36:06.7087662Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7087662Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7087866Z   --> $DIR/regex.rs:21:41
2019-11-06T21:36:06.7087909Z    |
2019-11-06T21:36:06.7087971Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T21:36:06.7088052Z 
2019-11-06T21:36:06.7088097Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7088313Z   --> $DIR/regex.rs:22:56
2019-11-06T21:36:06.7088363Z    |
2019-11-06T21:36:06.7088363Z    |
2019-11-06T21:36:06.7088411Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T21:36:06.7088512Z 
2019-11-06T21:36:06.7088650Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7088881Z   --> $DIR/regex.rs:34:37
2019-11-06T21:36:06.7088940Z    |
2019-11-06T21:36:06.7088940Z    |
2019-11-06T21:36:06.7089190Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T21:36:06.7089289Z 
2019-11-06T21:36:06.7089333Z error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7089532Z   --> $DIR/regex.rs:35:39
2019-11-06T21:36:06.7089575Z    |
2019-11-06T21:36:06.7089575Z    |
2019-11-06T21:36:06.7089841Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T21:36:06.7090165Z 
2019-11-06T21:36:06.7090217Z error: regex syntax error: unrecognized escape sequence
2019-11-06T21:36:06.7090555Z   --> $DIR/regex.rs:37:45
2019-11-06T21:36:06.7090587Z    |
2019-11-06T21:36:06.7090587Z    |
2019-11-06T21:36:06.7090629Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T21:36:06.7090706Z 
2019-11-06T21:36:06.7090741Z error: regex syntax error: unrecognized escape sequence
2019-11-06T21:36:06.7090912Z   --> $DIR/regex.rs:38:46
2019-11-06T21:36:06.7090945Z    |
2019-11-06T21:36:06.7090945Z    |
2019-11-06T21:36:06.7090984Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T21:36:06.7091062Z 
2019-11-06T21:36:06.7091092Z error: trivial regex
2019-11-06T21:36:06.7091244Z   --> $DIR/regex.rs:42:33
2019-11-06T21:36:06.7091293Z    |
2019-11-06T21:36:06.7091293Z    |
2019-11-06T21:36:06.7091326Z LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T21:36:06.7091418Z    |
2019-11-06T21:36:06.7091418Z    |
2019-11-06T21:36:06.7091452Z    = help: consider using `==` on `str`s
2019-11-06T21:36:06.7091505Z error: trivial regex
2019-11-06T21:36:06.7091684Z   --> $DIR/regex.rs:44:48
2019-11-06T21:36:06.7091718Z    |
2019-11-06T21:36:06.7091718Z    |
2019-11-06T21:36:06.7091753Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T21:36:06.7091842Z    |
2019-11-06T21:36:06.7091842Z    |
2019-11-06T21:36:06.7091875Z    = help: consider using `==` on `str`s
2019-11-06T21:36:06.7091945Z error: trivial regex
2019-11-06T21:36:06.7092101Z   --> $DIR/regex.rs:46:42
2019-11-06T21:36:06.7092134Z    |
2019-11-06T21:36:06.7092134Z    |
2019-11-06T21:36:06.7092168Z LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T21:36:06.7092252Z    |
2019-11-06T21:36:06.7092292Z    = help: consider using `str::starts_with`
2019-11-06T21:36:06.7092333Z 
2019-11-06T21:36:06.7092540Z error: trivial regex
2019-11-06T21:36:06.7092540Z error: trivial regex
2019-11-06T21:36:06.7092952Z   --> $DIR/regex.rs:48:40
2019-11-06T21:36:06.7093166Z    |
2019-11-06T21:36:06.7093226Z LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T21:36:06.7093485Z    |
2019-11-06T21:36:06.7093540Z    = help: consider using `str::ends_with`
2019-11-06T21:36:06.7093567Z 
2019-11-06T21:36:06.7093601Z error: trivial regex
---
2019-11-06T21:36:06.7094025Z 
2019-11-06T21:36:06.7094059Z error: trivial regex
2019-11-06T21:36:06.7094259Z   --> $DIR/regex.rs:52:39
2019-11-06T21:36:06.7094296Z    |
2019-11-06T21:36:06.7094336Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T21:36:06.7094497Z    |
2019-11-06T21:36:06.7094539Z    = help: consider using `str::contains`
2019-11-06T21:36:06.7094564Z 
2019-11-06T21:36:06.7094615Z error: trivial regex
2019-11-06T21:36:06.7094615Z error: trivial regex
2019-11-06T21:36:06.7094806Z   --> $DIR/regex.rs:54:40
2019-11-06T21:36:06.7094844Z    |
2019-11-06T21:36:06.7094898Z LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T21:36:06.7094978Z    |
2019-11-06T21:36:06.7095032Z    = help: consider using `str::contains`
2019-11-06T21:36:06.7095060Z 
2019-11-06T21:36:06.7095100Z error: trivial regex
---
2019-11-06T21:36:06.7095668Z 
2019-11-06T21:36:06.7095706Z error: trivial regex
2019-11-06T21:36:06.7095951Z   --> $DIR/regex.rs:59:36
2019-11-06T21:36:06.7095992Z    |
2019-11-06T21:36:06.7096033Z LL |     let trivial_empty = Regex::new("^");
2019-11-06T21:36:06.7096801Z    |
2019-11-06T21:36:06.7096846Z    = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7096877Z 
2019-11-06T21:36:06.7096940Z error: trivial regex
2019-11-06T21:36:06.7096940Z error: trivial regex
2019-11-06T21:36:06.7097166Z   --> $DIR/regex.rs:61:36
2019-11-06T21:36:06.7097210Z    |
2019-11-06T21:36:06.7097254Z LL |     let trivial_empty = Regex::new("^$");
2019-11-06T21:36:06.7097360Z    |
2019-11-06T21:36:06.7097413Z    = help: consider using `str::is_empty`
2019-11-06T21:36:06.7097459Z 
2019-11-06T21:36:06.7097500Z error: trivial regex
2019-11-06T21:36:06.7097500Z error: trivial regex
2019-11-06T21:36:06.7097704Z   --> $DIR/regex.rs:63:44
2019-11-06T21:36:06.7097747Z    |
2019-11-06T21:36:06.7097816Z LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T21:36:06.7097907Z    |
2019-11-06T21:36:06.7097969Z    = help: consider using `str::is_empty`
2019-11-06T21:36:06.7097998Z 
2019-11-06T21:36:06.7098040Z error: aborting due to 23 previous errors
---
2019-11-06T21:36:06.7099388Z -  --> $DIR/regex.rs:13:45
2019-11-06T21:36:06.7099606Z +error[E0463]: can't find crate for `regex`
2019-11-06T21:36:06.7099803Z +  --> $DIR/regex.rs:4:1
2019-11-06T21:36:06.7100032Z     |
2019-11-06T21:36:06.7100209Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T21:36:06.7100519Z -   |
2019-11-06T21:36:06.7100720Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T21:36:06.7100897Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7100934Z +LL | extern crate regex;
2019-11-06T21:36:06.7100934Z +LL | extern crate regex;
2019-11-06T21:36:06.7101112Z +   | ^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T21:36:06.7101147Z  
2019-11-06T21:36:06.7101291Z -error: trivial regex
2019-11-06T21:36:06.7101457Z -  --> $DIR/regex.rs:14:60
2019-11-06T21:36:06.7101596Z -   |
2019-11-06T21:36:06.7101781Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T21:36:06.7102121Z -   |
2019-11-06T21:36:06.7103219Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7103280Z +error: aborting due to previous error
2019-11-06T21:36:06.7103332Z  
2019-11-06T21:36:06.7103332Z  
2019-11-06T21:36:06.7103534Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T21:36:06.7104026Z -   |
2019-11-06T21:36:06.7104026Z -   |
2019-11-06T21:36:06.7104732Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T21:36:06.7105153Z -   |
2019-11-06T21:36:06.7105153Z -   |
2019-11-06T21:36:06.7105359Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T21:36:06.7105515Z -
2019-11-06T21:36:06.7105756Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T21:36:06.7106626Z -   |
2019-11-06T21:36:06.7106626Z -   |
2019-11-06T21:36:06.7106886Z -LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T21:36:06.7107277Z -
2019-11-06T21:36:06.7108013Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7108258Z -  --> $DIR/regex.rs:18:33
2019-11-06T21:36:06.7108433Z -   |
2019-11-06T21:36:06.7108433Z -   |
2019-11-06T21:36:06.7108652Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T21:36:06.7109080Z -
2019-11-06T21:36:06.7109267Z -error: trivial regex
2019-11-06T21:36:06.7109480Z -  --> $DIR/regex.rs:20:53
2019-11-06T21:36:06.7109656Z -   |
2019-11-06T21:36:06.7109656Z -   |
2019-11-06T21:36:06.7109890Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T21:36:06.7110313Z -   |
2019-11-06T21:36:06.7110534Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7110706Z -
2019-11-06T21:36:06.7111092Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7111092Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7111262Z -  --> $DIR/regex.rs:21:41
2019-11-06T21:36:06.7111410Z -   |
2019-11-06T21:36:06.7111785Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T21:36:06.7112266Z -
2019-11-06T21:36:06.7112444Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7112628Z -  --> $DIR/regex.rs:22:56
2019-11-06T21:36:06.7112769Z -   |
2019-11-06T21:36:06.7112769Z -   |
2019-11-06T21:36:06.7112962Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T21:36:06.7113310Z -
2019-11-06T21:36:06.7113487Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7113662Z -  --> $DIR/regex.rs:34:37
2019-11-06T21:36:06.7113800Z -   |
2019-11-06T21:36:06.7113800Z -   |
2019-11-06T21:36:06.7113999Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T21:36:06.7114335Z -
2019-11-06T21:36:06.7114523Z -error: regex syntax error on position 0: unclosed group
2019-11-06T21:36:06.7114682Z -  --> $DIR/regex.rs:35:39
2019-11-06T21:36:06.7115031Z -   |
2019-11-06T21:36:06.7115031Z -   |
2019-11-06T21:36:06.7115224Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T21:36:06.7115736Z -
2019-11-06T21:36:06.7116184Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T21:36:06.7116356Z -  --> $DIR/regex.rs:37:45
2019-11-06T21:36:06.7116672Z -   |
2019-11-06T21:36:06.7116672Z -   |
2019-11-06T21:36:06.7117108Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T21:36:06.7117523Z -
2019-11-06T21:36:06.7117745Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T21:36:06.7117942Z -  --> $DIR/regex.rs:38:46
2019-11-06T21:36:06.7118116Z -   |
2019-11-06T21:36:06.7118116Z -   |
2019-11-06T21:36:06.7118613Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T21:36:06.7119023Z -
2019-11-06T21:36:06.7119229Z -error: trivial regex
2019-11-06T21:36:06.7119427Z -  --> $DIR/regex.rs:42:33
2019-11-06T21:36:06.7119601Z -   |
2019-11-06T21:36:06.7119601Z -   |
2019-11-06T21:36:06.7119947Z -LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T21:36:06.7120669Z -   |
2019-11-06T21:36:06.7120669Z -   |
2019-11-06T21:36:06.7120838Z -   = help: consider using `==` on `str`s
2019-11-06T21:36:06.7121148Z -error: trivial regex
2019-11-06T21:36:06.7121309Z -  --> $DIR/regex.rs:44:48
2019-11-06T21:36:06.7121468Z -   |
2019-11-06T21:36:06.7121468Z -   |
2019-11-06T21:36:06.7121813Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T21:36:06.7122152Z -   |
2019-11-06T21:36:06.7122152Z -   |
2019-11-06T21:36:06.7122317Z -   = help: consider using `==` on `str`s
2019-11-06T21:36:06.7122693Z -error: trivial regex
2019-11-06T21:36:06.7122864Z -  --> $DIR/regex.rs:46:42
2019-11-06T21:36:06.7123003Z -   |
2019-11-06T21:36:06.7123003Z -   |
2019-11-06T21:36:06.7123181Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T21:36:06.7123525Z -   |
2019-11-06T21:36:06.7123693Z -   = help: consider using `str::starts_with`
2019-11-06T21:36:06.7123846Z -
2019-11-06T21:36:06.7124219Z -error: trivial regex
2019-11-06T21:36:06.7124219Z -error: trivial regex
2019-11-06T21:36:06.7124564Z -  --> $DIR/regex.rs:48:40
2019-11-06T21:36:06.7124709Z -   |
2019-11-06T21:36:06.7124968Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T21:36:06.7125294Z -   |
2019-11-06T21:36:06.7125481Z -   = help: consider using `str::ends_with`
2019-11-06T21:36:06.7125624Z -
2019-11-06T21:36:06.7125778Z -error: trivial regex
---
2019-11-06T21:36:06.7127513Z -
2019-11-06T21:36:06.7127715Z -error: trivial regex
2019-11-06T21:36:06.7127911Z -  --> $DIR/regex.rs:52:39
2019-11-06T21:36:06.7128083Z -   |
2019-11-06T21:36:06.7128311Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T21:36:06.7128737Z -   |
2019-11-06T21:36:06.7128946Z -   = help: consider using `str::contains`
2019-11-06T21:36:06.7129134Z -
2019-11-06T21:36:06.7129319Z -error: trivial regex
2019-11-06T21:36:06.7129319Z -error: trivial regex
2019-11-06T21:36:06.7129514Z -  --> $DIR/regex.rs:54:40
2019-11-06T21:36:06.7129703Z -   |
2019-11-06T21:36:06.7129924Z -LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T21:36:06.7130613Z -   |
2019-11-06T21:36:06.7130786Z -   = help: consider using `str::contains`
2019-11-06T21:36:06.7130925Z -
2019-11-06T21:36:06.7131078Z -error: trivial regex
---
2019-11-06T21:36:06.7132233Z -
2019-11-06T21:36:06.7132557Z -error: trivial regex
2019-11-06T21:36:06.7132713Z -  --> $DIR/regex.rs:59:36
2019-11-06T21:36:06.7132851Z -   |
2019-11-06T21:36:06.7133033Z -LL |     let trivial_empty = Regex::new("^");
2019-11-06T21:36:06.7133342Z -   |
2019-11-06T21:36:06.7133542Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T21:36:06.7133680Z -
2019-11-06T21:36:06.7133829Z -error: trivial regex
2019-11-06T21:36:06.7133829Z -error: trivial regex
2019-11-06T21:36:06.7133986Z -  --> $DIR/regex.rs:61:36
2019-11-06T21:36:06.7134139Z -   |
2019-11-06T21:36:06.7134403Z -LL |     let trivial_empty = Regex::new("^$");
2019-11-06T21:36:06.7134760Z -   |
2019-11-06T21:36:06.7134924Z -   = help: consider using `str::is_empty`
2019-11-06T21:36:06.7135058Z -
2019-11-06T21:36:06.7135222Z -error: trivial regex
2019-11-06T21:36:06.7135222Z -error: trivial regex
2019-11-06T21:36:06.7135379Z -  --> $DIR/regex.rs:63:44
2019-11-06T21:36:06.7135516Z -   |
2019-11-06T21:36:06.7135690Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T21:36:06.7136026Z -   |
2019-11-06T21:36:06.7136565Z -   = help: consider using `str::is_empty`
2019-11-06T21:36:06.7136765Z -
2019-11-06T21:36:06.7136979Z -error: aborting due to 23 previous errors
---
2019-11-06T21:36:06.7141473Z 
2019-11-06T21:36:06.7141661Z ------------------------------------------
2019-11-06T21:36:06.7141698Z stderr:
2019-11-06T21:36:06.7141866Z ------------------------------------------
2019-11-06T21:36:06.7142970Z {"message":"can't find crate for `regex`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n