
2019-09-02T21:25:25.1150461Z thread '[ui] ui/format.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-09-02T21:25:25.1150578Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-02T21:25:25.1150642Z
2019-09-02T21:25:25.1150897Z ---- [ui] ui/unicode.rs stdout ----
2019-09-02T21:25:25.1150977Z normalized stderr:
2019-09-02T21:25:25.1151218Z error: zero-width space detected
2019-09-02T21:25:25.1151462Z --> $DIR/unicode.rs:3:12
2019-09-02T21:25:25.1151531Z |
2019-09-02T21:25:25.1151800Z LL | print!("Here >​< is a ZWS, and ​another");
2019-09-02T21:25:25.1151910Z | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider replacing the string with: `"Here >/u{200B}< is a ZWS, and /u{200B}another"`
2019-09-02T21:25:25.1152016Z |
2019-09-02T21:25:25.1152293Z = note: `-D clippy::zero-width-space` implied by `-D warnings`
2019-09-02T21:25:25.1152359Z
2019-09-02T21:25:25.1152603Z error: non-nfc unicode sequence detected
2019-09-02T21:25:25.1152851Z --> $DIR/unicode.rs:9:12
2019-09-02T21:25:25.1152930Z |
2019-09-02T21:25:25.1153164Z LL | print!("̀àh?");
2019-09-02T21:25:25.1153456Z | ^^^^^ help: consider replacing the string with: `"̀àh?"`
2019-09-02T21:25:25.1153550Z |
2019-09-02T21:25:25.1153956Z = note: `-D clippy::unicode-not-nfc` implied by `-D warnings`
2019-09-02T21:25:25.1154013Z
2019-09-02T21:25:25.1154268Z error: literal non-ASCII character detected
2019-09-02T21:25:25.1154509Z --> $DIR/unicode.rs:15:12
2019-09-02T21:25:25.1154586Z |
2019-09-02T21:25:25.1154815Z LL | print!("Üben!");
2019-09-02T21:25:25.1154906Z | ^^^^^^^ help: consider replacing the string with: `"/u{dc}ben!"`
2019-09-02T21:25:25.1154987Z |
2019-09-02T21:25:25.1155268Z = note: `-D clippy::non-ascii-literal` implied by `-D warnings`
2019-09-02T21:25:25.1155323Z
2019-09-02T21:25:25.1155394Z error: aborting due to 3 previous errors
2019-09-02T21:25:25.1155440Z
2019-09-02T21:25:25.1155475Z
2019-09-02T21:25:25.1155518Z
2019-09-02T21:25:25.1155578Z expected stderr:
2019-09-02T21:25:25.1155835Z error: zero-width space detected
2019-09-02T21:25:25.1156076Z --> $DIR/unicode.rs:3:12
2019-09-02T21:25:25.1156156Z |
2019-09-02T21:25:25.1156415Z LL | print!("Here >​< is a ZWS, and ​another");
2019-09-02T21:25:25.1156514Z | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-02T21:25:25.1156753Z |
2019-09-02T21:25:25.1157023Z = note: `-D clippy::zero-width-space` implied by `-D warnings`
2019-09-02T21:25:25.1157074Z
2019-09-02T21:25:25.1157294Z error: non-nfc unicode sequence detected
2019-09-02T21:25:25.1157519Z --> $DIR/unicode.rs:9:12
2019-09-02T21:25:25.1157581Z |
2019-09-02T21:25:25.1158159Z LL | print!("̀àh?");
2019-09-02T21:25:25.1158234Z | ^^^^^
2019-09-02T21:25:25.1158304Z |
2019-09-02T21:25:25.1158576Z = note: `-D clippy::unicode-not-nfc` implied by `-D warnings`
2019-09-02T21:25:25.1158636Z
2019-09-02T21:25:25.1158866Z error: literal non-ASCII character detected
2019-09-02T21:25:25.1159097Z --> $DIR/unicode.rs:15:12
2019-09-02T21:25:25.1159261Z |
2019-09-02T21:25:25.1159526Z LL | print!("Üben!");
2019-09-02T21:25:25.1159595Z | ^^^^^^^
2019-09-02T21:25:25.1159661Z |
2019-09-02T21:25:25.1159916Z = note: `-D clippy::non-ascii-literal` implied by `-D warnings`
2019-09-02T21:25:25.1159981Z
2019-09-02T21:25:25.1160049Z error: aborting due to 3 previous errors
2019-09-02T21:25:25.1160092Z
2019-09-02T21:25:25.1160131Z
2019-09-02T21:25:25.1160166Z
2019-09-02T21:25:25.1160223Z diff of stderr:
2019-09-02T21:25:25.1160272Z
2019-09-02T21:25:25.1160496Z error: zero-width space detected
2019-09-02T21:25:25.1160730Z --> $DIR/unicode.rs:3:12
2019-09-02T21:25:25.1160797Z |
2019-09-02T21:25:25.1161046Z LL | print!("Here >​< is a ZWS, and ​another");
2019-09-02T21:25:25.1161295Z - | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-02T21:25:25.1161566Z + | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider replacing the string with: `"Here >/u{200B}< is a ZWS, and /u{200B}another"`
2019-09-02T21:25:25.1161666Z |
2019-09-02T21:25:25.1161926Z = note: `-D clippy::zero-width-space` implied by `-D warnings`
2019-09-02T21:25:25.1161999Z
2019-09-02T21:25:25.1162230Z error: non-nfc unicode sequence detected
2019-09-02T21:25:25.1162457Z --> $DIR/unicode.rs:9:12
2019-09-02T21:25:25.1162526Z |
2019-09-02T21:25:25.1162729Z LL | print!("̀àh?");
2019-09-02T21:25:25.1164047Z - | ^^^^^
2019-09-02T21:25:25.1164330Z + | ^^^^^ help: consider replacing the string with: `"̀àh?"`
2019-09-02T21:25:25.1164410Z |
2019-09-02T21:25:25.1164655Z = note: `-D clippy::unicode-not-nfc` implied by `-D warnings`
2019-09-02T21:25:25.1164733Z
2019-09-02T21:25:25.1164959Z error: literal non-ASCII character detected
2019-09-02T21:25:25.1165183Z --> $DIR/unicode.rs:15:12
2019-09-02T21:25:25.1165246Z |
2019-09-02T21:25:25.1165458Z LL | print!("Üben!");
2019-09-02T21:25:25.1165665Z - | ^^^^^^^
2019-09-02T21:25:25.1165762Z + | ^^^^^^^ help: consider replacing the string with: `"/u{dc}ben!"`
2019-09-02T21:25:25.1165837Z |
2019-09-02T21:25:25.1166092Z = note: `-D clippy::non-ascii-literal` implied by `-D warnings`
2019-09-02T21:25:25.1166165Z
2019-09-02T21:25:25.1166364Z error: aborting due to 3 previous errors 
