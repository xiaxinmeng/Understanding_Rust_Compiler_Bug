plain
2019-09-30T23:16:23.8063883Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T23:16:23.8261177Z ##[command]git config gc.auto 0
2019-09-30T23:16:23.8340389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T23:16:23.8390375Z ##[command]git config --get-all http.proxy
2019-09-30T23:16:23.8547487Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64935/merge:refs/remotes/pull/64935/merge
---
2019-09-30T23:24:26.7473776Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-30T23:24:26.7837534Z error: incorrect close delimiter: `}`
2019-09-30T23:24:26.7838927Z     --> src/librustc_errors/emitter.rs:1308:17
2019-09-30T23:24:26.7839927Z      |
2019-09-30T23:24:26.7840343Z 1297 |                 for line in &annotated_file.lines {
2019-09-30T23:24:26.7840893Z      |                                                   - close delimiter possibly meant for this
2019-09-30T23:24:26.7841773Z 1298 |                     max_line_len = max(max_line_len, annotated_file.file
2019-09-30T23:24:26.7842236Z      |                                       - un-closed delimiter
2019-09-30T23:24:26.7842800Z 1308 |                 }
2019-09-30T23:24:26.7843153Z      |                 ^ incorrect close delimiter
2019-09-30T23:24:26.7843217Z 
2019-09-30T23:24:26.7905285Z error: expected identifier, found reserved keyword `override`
2019-09-30T23:24:26.7905285Z error: expected identifier, found reserved keyword `override`
2019-09-30T23:24:26.7906453Z     --> src/librustc_errors/emitter.rs:1060:44
2019-09-30T23:24:26.7906786Z      |
2019-09-30T23:24:26.7910054Z 1060 |         fn style_or_override(style: Style, override: Option<Style>) -> Style {
2019-09-30T23:24:26.7910948Z help: you can escape reserved keywords to use them as identifiers
2019-09-30T23:24:26.7911205Z      |
2019-09-30T23:24:26.7911205Z      |
2019-09-30T23:24:26.7911876Z 1060 |         fn style_or_override(style: Style, r#override: Option<Style>) -> Style {
2019-09-30T23:24:26.7918410Z 
2019-09-30T23:24:26.7942459Z error: expected expression, found reserved keyword `override`
2019-09-30T23:24:26.7942807Z     --> src/librustc_errors/emitter.rs:1061:27
2019-09-30T23:24:26.7943004Z      |
2019-09-30T23:24:26.7943004Z      |
2019-09-30T23:24:26.7943272Z 1061 |             match (style, override) {
2019-09-30T23:24:26.7944713Z 
2019-09-30T23:24:26.7982476Z error: expected identifier, found reserved keyword `override`
2019-09-30T23:24:26.7982824Z     --> src/librustc_errors/emitter.rs:1062:39
2019-09-30T23:24:26.7983044Z      |
2019-09-30T23:24:26.7983044Z      |
2019-09-30T23:24:26.7983555Z 1062 |                 (Style::NoStyle, Some(override)) => override,
2019-09-30T23:24:26.7984216Z help: you can escape reserved keywords to use them as identifiers
2019-09-30T23:24:26.7984410Z      |
2019-09-30T23:24:26.7984410Z      |
2019-09-30T23:24:26.7984675Z 1062 |                 (Style::NoStyle, Some(r#override)) => override,
2019-09-30T23:24:26.7989479Z 
2019-09-30T23:24:26.8021758Z error: expected expression, found reserved keyword `override`
2019-09-30T23:24:26.8022707Z     --> src/librustc_errors/emitter.rs:1062:53
2019-09-30T23:24:26.8023259Z      |
2019-09-30T23:24:26.8023259Z      |
2019-09-30T23:24:26.8023836Z 1062 |                 (Style::NoStyle, Some(override)) => override,
2019-09-30T23:24:26.8024493Z      |                                                  -- ^^^^^^^^ expected expression
2019-09-30T23:24:26.8025674Z      |                                                  while parsing the `match` arm starting here
2019-09-30T23:24:26.8027725Z 
2019-09-30T23:24:26.8027725Z 
2019-09-30T23:24:26.8101924Z error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
2019-09-30T23:24:26.8120460Z      |
2019-09-30T23:24:26.8120460Z      |
2019-09-30T23:24:26.8120934Z 1298 |                     max_line_len = max(max_line_len, annotated_file.file
2019-09-30T23:24:26.8121240Z      |                                       - unclosed delimiter
2019-09-30T23:24:26.8121532Z 1299 |                         .get_line(line.line_index - 1)
2019-09-30T23:24:26.8121797Z 1300 |                         .map_or(0, |s| s.len());
2019-09-30T23:24:26.8122097Z      |                                                ^ help: `)` may belong here
2019-09-30T23:24:26.8122390Z error: expected expression, found `)`
2019-09-30T23:24:26.8122621Z     --> src/librustc_errors/emitter.rs:1308:17
2019-09-30T23:24:26.8122829Z      |
2019-09-30T23:24:26.8123059Z 1308 |                 }
2019-09-30T23:24:26.8123059Z 1308 |                 }
2019-09-30T23:24:26.8123330Z      |                 ^ expected expression
2019-09-30T23:24:26.8123381Z 
2019-09-30T23:24:26.9447743Z error[E0425]: cannot find value `a` in this scope
2019-09-30T23:24:26.9448196Z    --> src/librustc_errors/emitter.rs:954:22
2019-09-30T23:24:26.9448458Z     |
2019-09-30T23:24:26.9449031Z 954 |             (Reverse(a.1.len()), a.1.is_primary)
2019-09-30T23:24:26.9449472Z     |                      ^ help: a local variable with a similar name exists: `p`
2019-09-30T23:24:26.9491823Z error[E0425]: cannot find value `a` in this scope
2019-09-30T23:24:26.9492122Z    --> src/librustc_errors/emitter.rs:954:34
2019-09-30T23:24:26.9492320Z     |
2019-09-30T23:24:26.9492320Z     |
2019-09-30T23:24:26.9492572Z 954 |             (Reverse(a.1.len()), a.1.is_primary)
2019-09-30T23:24:26.9492905Z     |                                  ^ help: a local variable with a similar name exists: `p`
2019-09-30T23:24:26.9492944Z 
2019-09-30T23:24:26.9526674Z error[E0425]: cannot find value `acc` in this scope
2019-09-30T23:24:26.9527981Z      |
2019-09-30T23:24:26.9527981Z      |
2019-09-30T23:24:26.9528324Z 1528 |                         .map(|ch| acc + unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1))
2019-09-30T23:24:26.9528728Z 
2019-09-30T23:24:26.9528728Z 
2019-09-30T23:24:27.3558947Z error[E0618]: expected function, found `{integer}`
2019-09-30T23:24:27.3559617Z      |
2019-09-30T23:24:27.3559888Z 1005 |         let mut max = 0;
2019-09-30T23:24:27.3559888Z 1005 |         let mut max = 0;
2019-09-30T23:24:27.3560284Z      |             ------- `{integer}` defined here
2019-09-30T23:24:27.3560501Z ...
2019-09-30T23:24:27.3560987Z 1009 |                 max = max(max, hi.line);
2019-09-30T23:24:27.3561292Z      |                       ^^^--------------
2019-09-30T23:24:27.3561825Z      |                       call expression requires function
2019-09-30T23:24:27.3561881Z 
2019-09-30T23:24:27.3561881Z 
2019-09-30T23:24:27.3647260Z error[E0618]: expected function, found `{integer}`
2019-09-30T23:24:27.3648207Z      |
2019-09-30T23:24:27.3648481Z 1005 |         let mut max = 0;
2019-09-30T23:24:27.3648481Z 1005 |         let mut max = 0;
2019-09-30T23:24:27.3648830Z      |             ------- `{integer}` defined here
2019-09-30T23:24:27.3649044Z ...
2019-09-30T23:24:27.3649348Z 1016 |                     max = max(max, hi.line);
2019-09-30T23:24:27.3649662Z      |                           ^^^--------------
2019-09-30T23:24:27.3650259Z      |                           call expression requires function
2019-09-30T23:24:27.3650528Z 
2019-09-30T23:24:27.5572104Z error: aborting due to 12 previous errors
2019-09-30T23:24:27.5572286Z 
---
2019-09-30T23:24:29.7764445Z == clock drift check ==
2019-09-30T23:24:29.7782288Z   local time: Mon Sep 30 23:24:29 UTC 2019
2019-09-30T23:24:29.9275385Z   network time: Mon, 30 Sep 2019 23:24:29 GMT
2019-09-30T23:24:29.9279220Z == end clock drift check ==
2019-09-30T23:24:31.1544533Z ##[error]Bash exited with code '1'.
2019-09-30T23:24:31.1586920Z ##[section]Starting: Checkout
2019-09-30T23:24:31.1589600Z ==============================================================================
2019-09-30T23:24:31.1589686Z Task         : Get sources
2019-09-30T23:24:31.1589741Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
