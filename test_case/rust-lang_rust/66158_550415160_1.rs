\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/lint_without_lint_pass.rs","byte_start":142,"byte_end":168,"line_start":9,"line_end":9,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/lint_without_lint_pass.rs:9:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T16:48:44.8284594Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T16:48:44.8284649Z 
2019-11-06T16:48:44.8284899Z ------------------------------------------
2019-11-06T16:48:44.8284953Z 
---
2019-11-06T16:48:44.8286990Z expected stderr:
2019-11-06T16:48:44.8287035Z error: trivial regex
2019-11-06T16:48:44.8287264Z   --> $DIR/regex.rs:13:45
2019-11-06T16:48:44.8287311Z    |
2019-11-06T16:48:44.8287376Z LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T16:48:44.8287471Z    |
2019-11-06T16:48:44.8287740Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T16:48:44.8287796Z    = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8287828Z 
2019-11-06T16:48:44.8287828Z 
2019-11-06T16:48:44.8287871Z error: trivial regex
2019-11-06T16:48:44.8288111Z   --> $DIR/regex.rs:14:60
2019-11-06T16:48:44.8288166Z    |
2019-11-06T16:48:44.8288214Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T16:48:44.8288331Z    |
2019-11-06T16:48:44.8288484Z    = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8288521Z 
2019-11-06T16:48:44.8288521Z 
2019-11-06T16:48:44.8288588Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T16:48:44.8288884Z    |
2019-11-06T16:48:44.8288884Z    |
2019-11-06T16:48:44.8289141Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T16:48:44.8289237Z    |
2019-11-06T16:48:44.8289237Z    |
2019-11-06T16:48:44.8289503Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T16:48:44.8289538Z 
2019-11-06T16:48:44.8289586Z error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T16:48:44.8289984Z    |
2019-11-06T16:48:44.8289984Z    |
2019-11-06T16:48:44.8290232Z LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T16:48:44.8290333Z 
2019-11-06T16:48:44.8290387Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8290609Z   --> $DIR/regex.rs:18:33
2019-11-06T16:48:44.8290655Z    |
2019-11-06T16:48:44.8290655Z    |
2019-11-06T16:48:44.8290718Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T16:48:44.8290797Z 
2019-11-06T16:48:44.8290856Z error: trivial regex
2019-11-06T16:48:44.8291073Z   --> $DIR/regex.rs:20:53
2019-11-06T16:48:44.8291118Z    |
2019-11-06T16:48:44.8291118Z    |
2019-11-06T16:48:44.8291164Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T16:48:44.8291274Z    |
2019-11-06T16:48:44.8291319Z    = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8291377Z 
2019-11-06T16:48:44.8291423Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8291423Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8291645Z   --> $DIR/regex.rs:21:41
2019-11-06T16:48:44.8291690Z    |
2019-11-06T16:48:44.8291761Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T16:48:44.8291843Z 
2019-11-06T16:48:44.8291905Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8292128Z   --> $DIR/regex.rs:22:56
2019-11-06T16:48:44.8292174Z    |
2019-11-06T16:48:44.8292174Z    |
2019-11-06T16:48:44.8292224Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T16:48:44.8292327Z 
2019-11-06T16:48:44.8292372Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8292609Z   --> $DIR/regex.rs:34:37
2019-11-06T16:48:44.8292663Z    |
2019-11-06T16:48:44.8292663Z    |
2019-11-06T16:48:44.8292937Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T16:48:44.8293040Z 
2019-11-06T16:48:44.8293084Z error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8293310Z   --> $DIR/regex.rs:35:39
2019-11-06T16:48:44.8293380Z    |
2019-11-06T16:48:44.8293380Z    |
2019-11-06T16:48:44.8293651Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T16:48:44.8293753Z 
2019-11-06T16:48:44.8293798Z error: regex syntax error: unrecognized escape sequence
2019-11-06T16:48:44.8294017Z   --> $DIR/regex.rs:37:45
2019-11-06T16:48:44.8294061Z    |
2019-11-06T16:48:44.8294061Z    |
2019-11-06T16:48:44.8294125Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T16:48:44.8294212Z 
2019-11-06T16:48:44.8294277Z error: regex syntax error: unrecognized escape sequence
2019-11-06T16:48:44.8294498Z   --> $DIR/regex.rs:38:46
2019-11-06T16:48:44.8294544Z    |
2019-11-06T16:48:44.8294544Z    |
2019-11-06T16:48:44.8294590Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T16:48:44.8294778Z 
2019-11-06T16:48:44.8294818Z error: trivial regex
2019-11-06T16:48:44.8295076Z   --> $DIR/regex.rs:42:33
2019-11-06T16:48:44.8295122Z    |
2019-11-06T16:48:44.8295122Z    |
2019-11-06T16:48:44.8295166Z LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T16:48:44.8295274Z    |
2019-11-06T16:48:44.8295274Z    |
2019-11-06T16:48:44.8295318Z    = help: consider using `==` on `str`s
2019-11-06T16:48:44.8295387Z error: trivial regex
2019-11-06T16:48:44.8295623Z   --> $DIR/regex.rs:44:48
2019-11-06T16:48:44.8295667Z    |
2019-11-06T16:48:44.8295667Z    |
2019-11-06T16:48:44.8295713Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T16:48:44.8295913Z    |
2019-11-06T16:48:44.8295913Z    |
2019-11-06T16:48:44.8295957Z    = help: consider using `==` on `str`s
2019-11-06T16:48:44.8296046Z error: trivial regex
2019-11-06T16:48:44.8296619Z   --> $DIR/regex.rs:46:42
2019-11-06T16:48:44.8296670Z    |
2019-11-06T16:48:44.8296670Z    |
2019-11-06T16:48:44.8296736Z LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T16:48:44.8296827Z    |
2019-11-06T16:48:44.8296889Z    = help: consider using `str::starts_with`
2019-11-06T16:48:44.8296918Z 
2019-11-06T16:48:44.8296958Z error: trivial regex
2019-11-06T16:48:44.8296958Z error: trivial regex
2019-11-06T16:48:44.8297181Z   --> $DIR/regex.rs:48:40
2019-11-06T16:48:44.8297243Z    |
2019-11-06T16:48:44.8297289Z LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T16:48:44.8297410Z    |
2019-11-06T16:48:44.8297453Z    = help: consider using `str::ends_with`
2019-11-06T16:48:44.8297482Z 
2019-11-06T16:48:44.8297521Z error: trivial regex
---
2019-11-06T16:48:44.8298042Z 
2019-11-06T16:48:44.8298101Z error: trivial regex
2019-11-06T16:48:44.8298322Z   --> $DIR/regex.rs:52:39
2019-11-06T16:48:44.8298366Z    |
2019-11-06T16:48:44.8298429Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T16:48:44.8298525Z    |
2019-11-06T16:48:44.8298568Z    = help: consider using `str::contains`
2019-11-06T16:48:44.8298615Z 
2019-11-06T16:48:44.8298663Z error: trivial regex
2019-11-06T16:48:44.8298663Z error: trivial regex
2019-11-06T16:48:44.8298884Z   --> $DIR/regex.rs:54:40
2019-11-06T16:48:44.8298946Z    |
2019-11-06T16:48:44.8298991Z LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T16:48:44.8299083Z    |
2019-11-06T16:48:44.8299154Z    = help: consider using `str::contains`
2019-11-06T16:48:44.8299185Z 
2019-11-06T16:48:44.8299225Z error: trivial regex
---
2019-11-06T16:48:44.8299745Z 
2019-11-06T16:48:44.8299787Z error: trivial regex
2019-11-06T16:48:44.8300022Z   --> $DIR/regex.rs:59:36
2019-11-06T16:48:44.8300068Z    |
2019-11-06T16:48:44.8300112Z LL |     let trivial_empty = Regex::new("^");
2019-11-06T16:48:44.8300228Z    |
2019-11-06T16:48:44.8300273Z    = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8300303Z 
2019-11-06T16:48:44.8300362Z error: trivial regex
2019-11-06T16:48:44.8300362Z error: trivial regex
2019-11-06T16:48:44.8300685Z   --> $DIR/regex.rs:61:36
2019-11-06T16:48:44.8300737Z    |
2019-11-06T16:48:44.8300801Z LL |     let trivial_empty = Regex::new("^$");
2019-11-06T16:48:44.8300892Z    |
2019-11-06T16:48:44.8300953Z    = help: consider using `str::is_empty`
2019-11-06T16:48:44.8300982Z 
2019-11-06T16:48:44.8301024Z error: trivial regex
2019-11-06T16:48:44.8301024Z error: trivial regex
2019-11-06T16:48:44.8301264Z   --> $DIR/regex.rs:63:44
2019-11-06T16:48:44.8301326Z    |
2019-11-06T16:48:44.8301373Z LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T16:48:44.8301484Z    |
2019-11-06T16:48:44.8301639Z    = help: consider using `str::is_empty`
2019-11-06T16:48:44.8301668Z 
2019-11-06T16:48:44.8301711Z error: aborting due to 23 previous errors
---
2019-11-06T16:48:44.8302364Z -  --> $DIR/regex.rs:13:45
2019-11-06T16:48:44.8302601Z +error[E0463]: can't find crate for `regex`
2019-11-06T16:48:44.8302835Z +  --> $DIR/regex.rs:4:1
2019-11-06T16:48:44.8302883Z     |
2019-11-06T16:48:44.8303127Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2019-11-06T16:48:44.8303589Z -   |
2019-11-06T16:48:44.8303839Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2019-11-06T16:48:44.8304087Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8304153Z +LL | extern crate regex;
2019-11-06T16:48:44.8304153Z +LL | extern crate regex;
2019-11-06T16:48:44.8304396Z +   | ^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T16:48:44.8304442Z  
2019-11-06T16:48:44.8304665Z -error: trivial regex
2019-11-06T16:48:44.8304882Z -  --> $DIR/regex.rs:14:60
2019-11-06T16:48:44.8305081Z -   |
2019-11-06T16:48:44.8305344Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2019-11-06T16:48:44.8305824Z -   |
2019-11-06T16:48:44.8306065Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8306386Z +error: aborting due to previous error
2019-11-06T16:48:44.8306444Z  
2019-11-06T16:48:44.8306444Z  
2019-11-06T16:48:44.8306762Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T16:48:44.8307217Z -   |
2019-11-06T16:48:44.8307217Z -   |
2019-11-06T16:48:44.8307457Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2019-11-06T16:48:44.8307929Z -   |
2019-11-06T16:48:44.8307929Z -   |
2019-11-06T16:48:44.8308179Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2019-11-06T16:48:44.8308392Z -
2019-11-06T16:48:44.8308666Z -error: regex syntax error: invalid character class range, the start must be <= the end
2019-11-06T16:48:44.8309773Z -   |
2019-11-06T16:48:44.8309773Z -   |
2019-11-06T16:48:44.8310064Z -LL |     let some_unicode = Regex::new("[é-è]");
2019-11-06T16:48:44.8310500Z -
2019-11-06T16:48:44.8310764Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8310986Z -  --> $DIR/regex.rs:18:33
2019-11-06T16:48:44.8311182Z -   |
2019-11-06T16:48:44.8311182Z -   |
2019-11-06T16:48:44.8311439Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2019-11-06T16:48:44.8311876Z -
2019-11-06T16:48:44.8312087Z -error: trivial regex
2019-11-06T16:48:44.8312340Z -  --> $DIR/regex.rs:20:53
2019-11-06T16:48:44.8312535Z -   |
2019-11-06T16:48:44.8312535Z -   |
2019-11-06T16:48:44.8313774Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2019-11-06T16:48:44.8318932Z -   |
2019-11-06T16:48:44.8320484Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8320763Z -
2019-11-06T16:48:44.8321016Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8321016Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8321236Z -  --> $DIR/regex.rs:21:41
2019-11-06T16:48:44.8321434Z -   |
2019-11-06T16:48:44.8321706Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2019-11-06T16:48:44.8322148Z -
2019-11-06T16:48:44.8322411Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8322631Z -  --> $DIR/regex.rs:22:56
2019-11-06T16:48:44.8322829Z -   |
2019-11-06T16:48:44.8322829Z -   |
2019-11-06T16:48:44.8323113Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2019-11-06T16:48:44.8323701Z -
2019-11-06T16:48:44.8323966Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8324187Z -  --> $DIR/regex.rs:34:37
2019-11-06T16:48:44.8324390Z -   |
2019-11-06T16:48:44.8324390Z -   |
2019-11-06T16:48:44.8324667Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T16:48:44.8325137Z -
2019-11-06T16:48:44.8325381Z -error: regex syntax error on position 0: unclosed group
2019-11-06T16:48:44.8325622Z -  --> $DIR/regex.rs:35:39
2019-11-06T16:48:44.8325818Z -   |
2019-11-06T16:48:44.8325818Z -   |
2019-11-06T16:48:44.8326090Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2019-11-06T16:48:44.8326886Z -
2019-11-06T16:48:44.8327143Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T16:48:44.8327380Z -  --> $DIR/regex.rs:37:45
2019-11-06T16:48:44.8327577Z -   |
2019-11-06T16:48:44.8327577Z -   |
2019-11-06T16:48:44.8327824Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2019-11-06T16:48:44.8328294Z -
2019-11-06T16:48:44.8328538Z -error: regex syntax error: unrecognized escape sequence
2019-11-06T16:48:44.8328758Z -  --> $DIR/regex.rs:38:46
2019-11-06T16:48:44.8328974Z -   |
2019-11-06T16:48:44.8328974Z -   |
2019-11-06T16:48:44.8329225Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2019-11-06T16:48:44.8329682Z -
2019-11-06T16:48:44.8329895Z -error: trivial regex
2019-11-06T16:48:44.8330111Z -  --> $DIR/regex.rs:42:33
2019-11-06T16:48:44.8330305Z -   |
2019-11-06T16:48:44.8330305Z -   |
2019-11-06T16:48:44.8330561Z -LL |     let trivial_eq = Regex::new("^foobar$");
2019-11-06T16:48:44.8331010Z -   |
2019-11-06T16:48:44.8331010Z -   |
2019-11-06T16:48:44.8331259Z -   = help: consider using `==` on `str`s
2019-11-06T16:48:44.8331659Z -error: trivial regex
2019-11-06T16:48:44.8331892Z -  --> $DIR/regex.rs:44:48
2019-11-06T16:48:44.8332089Z -   |
2019-11-06T16:48:44.8332089Z -   |
2019-11-06T16:48:44.8332349Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2019-11-06T16:48:44.8332831Z -   |
2019-11-06T16:48:44.8332831Z -   |
2019-11-06T16:48:44.8333058Z -   = help: consider using `==` on `str`s
2019-11-06T16:48:44.8333477Z -error: trivial regex
2019-11-06T16:48:44.8333693Z -  --> $DIR/regex.rs:46:42
2019-11-06T16:48:44.8333889Z -   |
2019-11-06T16:48:44.8333889Z -   |
2019-11-06T16:48:44.8334153Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2019-11-06T16:48:44.8334599Z -   |
2019-11-06T16:48:44.8334829Z -   = help: consider using `str::starts_with`
2019-11-06T16:48:44.8335057Z -
2019-11-06T16:48:44.8335268Z -error: trivial regex
2019-11-06T16:48:44.8335268Z -error: trivial regex
2019-11-06T16:48:44.8335484Z -  --> $DIR/regex.rs:48:40
2019-11-06T16:48:44.8335696Z -   |
2019-11-06T16:48:44.8335938Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2019-11-06T16:48:44.8336901Z -   |
2019-11-06T16:48:44.8337138Z -   = help: consider using `str::ends_with`
2019-11-06T16:48:44.8337333Z -
2019-11-06T16:48:44.8337541Z -error: trivial regex
---
2019-11-06T16:48:44.8339117Z -
2019-11-06T16:48:44.8339327Z -error: trivial regex
2019-11-06T16:48:44.8339650Z -  --> $DIR/regex.rs:52:39
2019-11-06T16:48:44.8339846Z -   |
2019-11-06T16:48:44.8340116Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2019-11-06T16:48:44.8340569Z -   |
2019-11-06T16:48:44.8340825Z -   = help: consider using `str::contains`
2019-11-06T16:48:44.8341018Z -
2019-11-06T16:48:44.8341224Z -error: trivial regex
2019-11-06T16:48:44.8341224Z -error: trivial regex
2019-11-06T16:48:44.8341456Z -  --> $DIR/regex.rs:54:40
2019-11-06T16:48:44.8341653Z -   |
2019-11-06T16:48:44.8341891Z -LL |     let trivial_backslash = Regex::new("a/.b");
2019-11-06T16:48:44.8342352Z -   |
2019-11-06T16:48:44.8342579Z -   = help: consider using `str::contains`
2019-11-06T16:48:44.8342769Z -
2019-11-06T16:48:44.8342993Z -error: trivial regex
---
2019-11-06T16:48:44.8344561Z -
2019-11-06T16:48:44.8344774Z -error: trivial regex
2019-11-06T16:48:44.8344998Z -  --> $DIR/regex.rs:59:36
2019-11-06T16:48:44.8345213Z -   |
2019-11-06T16:48:44.8345449Z -LL |     let trivial_empty = Regex::new("^");
2019-11-06T16:48:44.8345878Z -   |
2019-11-06T16:48:44.8346478Z -   = help: the regex is unlikely to be useful as it is
2019-11-06T16:48:44.8346730Z -
2019-11-06T16:48:44.8346942Z -error: trivial regex
2019-11-06T16:48:44.8346942Z -error: trivial regex
2019-11-06T16:48:44.8347186Z -  --> $DIR/regex.rs:61:36
2019-11-06T16:48:44.8347381Z -   |
2019-11-06T16:48:44.8347614Z -LL |     let trivial_empty = Regex::new("^$");
2019-11-06T16:48:44.8348066Z -   |
2019-11-06T16:48:44.8348306Z -   = help: consider using `str::is_empty`
2019-11-06T16:48:44.8348497Z -
2019-11-06T16:48:44.8348723Z -error: trivial regex
2019-11-06T16:48:44.8348723Z -error: trivial regex
2019-11-06T16:48:44.8348946Z -  --> $DIR/regex.rs:63:44
2019-11-06T16:48:44.8349356Z -   |
2019-11-06T16:48:44.8349631Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2019-11-06T16:48:44.8350087Z -   |
2019-11-06T16:48:44.8350335Z -   = help: consider using `str::is_empty`
2019-11-06T16:48:44.8350528Z -
2019-11-06T16:48:44.8350807Z -error: aborting due to 23 previous errors
---
2019-11-06T16:48:44.8354341Z 
2019-11-06T16:48:44.8354575Z ------------------------------------------
2019-11-06T16:48:44.8354622Z stderr:
2019-11-06T16:48:44.8354848Z ------------------------------------------
2019-11-06T16:48:44.8356117Z {"message":"can't find crate for `regex`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n