plain
2019-08-31T17:22:45.9848478Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-31T17:22:46.0039478Z ##[command]git config gc.auto 0
2019-08-31T17:22:46.0128477Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-31T17:22:46.0175651Z ##[command]git config --get-all http.proxy
2019-08-31T17:22:46.0321791Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64041/merge:refs/remotes/pull/64041/merge
---
2019-08-31T17:31:31.8538027Z 
2019-08-31T17:31:31.8554092Z error: constant item is never used: `OPTIONS`
2019-08-31T17:31:31.8554864Z   --> src/libsyntax_ext/asm.rs:42:1
2019-08-31T17:31:31.8555407Z    |
2019-08-31T17:31:31.8556070Z 42 | const OPTIONS: &[Symbol] = &[sym::volatile, sym::alignstack, sym::intel];
2019-08-31T17:31:31.8557367Z 
2019-08-31T17:31:31.8557367Z 
2019-08-31T17:31:31.8624580Z error: function is never used: `expand_asm`
2019-08-31T17:31:31.8625662Z   --> src/libsyntax_ext/asm.rs:44:1
2019-08-31T17:31:31.8626241Z    |
2019-08-31T17:31:31.8626842Z 44 | / pub fn expand_asm<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.8627761Z 45 | |                        sp: Span,
2019-08-31T17:31:31.8628330Z 46 | |                        tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.8629406Z 47 | |                        -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.8630602Z 68 | |     }))
2019-08-31T17:31:31.8631195Z 69 | | }
2019-08-31T17:31:31.8631586Z    | |_^
2019-08-31T17:31:31.8631730Z 
2019-08-31T17:31:31.8631730Z 
2019-08-31T17:31:31.8709311Z error: function is never used: `parse_inline_asm`
2019-08-31T17:31:31.8711936Z    --> src/libsyntax_ext/asm.rs:71:1
2019-08-31T17:31:31.8712589Z     |
2019-08-31T17:31:31.8713128Z 71  | / fn parse_inline_asm<'a>(
2019-08-31T17:31:31.8714027Z 72  | |     cx: &mut ExtCtxt<'a>,
2019-08-31T17:31:31.8714558Z 73  | |     sp: Span,
2019-08-31T17:31:31.8715026Z 74  | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.8715922Z 280 | |     }))
2019-08-31T17:31:31.8716355Z 281 | | }
2019-08-31T17:31:31.8716771Z     | |_^
2019-08-31T17:31:31.8717083Z 
2019-08-31T17:31:31.8717083Z 
2019-08-31T17:31:31.8717443Z error: function is never used: `expand_cfg`
2019-08-31T17:31:31.8717847Z   --> src/libsyntax_ext/cfg.rs:14:1
2019-08-31T17:31:31.8718205Z    |
2019-08-31T17:31:31.8718668Z 14 | / pub fn expand_cfg(
2019-08-31T17:31:31.8719099Z 15 | |     cx: &mut ExtCtxt<'_>,
2019-08-31T17:31:31.8719781Z 16 | |     sp: Span,
2019-08-31T17:31:31.8720250Z 17 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.8721047Z 30 | |     }
2019-08-31T17:31:31.8721451Z 31 | | }
2019-08-31T17:31:31.8721932Z    | |_^
2019-08-31T17:31:31.8722134Z 
2019-08-31T17:31:31.8722134Z 
2019-08-31T17:31:31.8722523Z error: function is never used: `parse_cfg`
2019-08-31T17:31:31.8722906Z   --> src/libsyntax_ext/cfg.rs:33:1
2019-08-31T17:31:31.8723269Z    |
2019-08-31T17:31:31.8724114Z 33 | / fn parse_cfg<'a>(
2019-08-31T17:31:31.8724605Z 34 | |     cx: &mut ExtCtxt<'a>,
2019-08-31T17:31:31.8725048Z 35 | |     sp: Span,
2019-08-31T17:31:31.8725542Z 36 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.8726370Z 54 | |     Ok(cfg)
2019-08-31T17:31:31.8726823Z 55 | | }
2019-08-31T17:31:31.8727361Z    | |_^
2019-08-31T17:31:31.8727540Z 
2019-08-31T17:31:31.8727540Z 
2019-08-31T17:31:31.8727909Z error: function is never used: `expand_compile_error`
2019-08-31T17:31:31.8728296Z   --> src/libsyntax_ext/compile_error.rs:7:1
2019-08-31T17:31:31.8728661Z    |
2019-08-31T17:31:31.8729122Z 7  | / pub fn expand_compile_error<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.8729570Z 8  | |                               sp: Span,
2019-08-31T17:31:31.8730252Z 9  | |                               tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.8730755Z 10 | |                               -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.8731161Z ...  |
2019-08-31T17:31:31.8731624Z 18 | |     DummyResult::any(sp)
2019-08-31T17:31:31.8732572Z    | |_^
2019-08-31T17:31:31.8732733Z 
2019-08-31T17:31:31.8733108Z error: function is never used: `expand_syntax_ext`
2019-08-31T17:31:31.8733888Z   --> src/libsyntax_ext/concat.rs:8:1
2019-08-31T17:31:31.8733888Z   --> src/libsyntax_ext/concat.rs:8:1
2019-08-31T17:31:31.8734342Z    |
2019-08-31T17:31:31.8734800Z 8  | / pub fn expand_syntax_ext(
2019-08-31T17:31:31.8735283Z 9  | |     cx: &mut base::ExtCtxt<'_>,
2019-08-31T17:31:31.8735933Z 10 | |     sp: syntax_pos::Span,
2019-08-31T17:31:31.8736431Z 11 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.8736824Z ...  |
2019-08-31T17:31:31.8737455Z 63 | |     base::MacEager::expr(cx.expr_str(sp, Symbol::intern(&accumulator)))
2019-08-31T17:31:31.8738415Z    | |_^
2019-08-31T17:31:31.8738593Z 
2019-08-31T17:31:31.8871138Z error: function is never used: `expand_syntax_ext`
2019-08-31T17:31:31.8872809Z   --> src/libsyntax_ext/concat_idents.rs:11:1
2019-08-31T17:31:31.8872809Z   --> src/libsyntax_ext/concat_idents.rs:11:1
2019-08-31T17:31:31.8874699Z    |
2019-08-31T17:31:31.8876539Z 11 | / pub fn expand_syntax_ext<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.8876910Z 12 | |                               sp: Span,
2019-08-31T17:31:31.8879912Z 13 | |                               tts: &[TokenTree])
2019-08-31T17:31:31.8880539Z 14 | |                               -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.8880984Z ...  |
2019-08-31T17:31:31.8881458Z 65 | |     Box::new(ConcatIdentsResult { ident })
2019-08-31T17:31:31.8882606Z    | |_^
2019-08-31T17:31:31.8882766Z 
2019-08-31T17:31:31.8882766Z 
2019-08-31T17:31:31.8883162Z error: function is never used: `expand_option_env`
2019-08-31T17:31:31.8883993Z   --> src/libsyntax_ext/env.rs:14:1
2019-08-31T17:31:31.8884427Z    |
2019-08-31T17:31:31.8884956Z 14 | / pub fn expand_option_env<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.8885464Z 15 | |                               sp: Span,
2019-08-31T17:31:31.8885979Z 16 | |                               tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.8886484Z 17 | |                               -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.8887069Z ...  |
2019-08-31T17:31:31.8887513Z 43 | |     MacEager::expr(e)
2019-08-31T17:31:31.8888364Z    | |_^
2019-08-31T17:31:31.8888515Z 
2019-08-31T17:31:31.8888515Z 
2019-08-31T17:31:31.8888885Z error: function is never used: `expand_env`
2019-08-31T17:31:31.8889303Z   --> src/libsyntax_ext/env.rs:46:1
2019-08-31T17:31:31.8889956Z    |
2019-08-31T17:31:31.8890428Z 46 | / pub fn expand_env<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.8890888Z 47 | |                        sp: Span,
2019-08-31T17:31:31.8891359Z 48 | |                        tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.8891995Z 49 | |                        -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.8893326Z ...  |
2019-08-31T17:31:31.8894276Z 85 | |     MacEager::expr(e)
2019-08-31T17:31:31.8895169Z    | |_^
2019-08-31T17:31:31.8895355Z 
2019-08-31T17:31:31.8895765Z error: variant is never constructed: `Placeholder`
2019-08-31T17:31:31.8896182Z   --> src/libsyntax_ext/format.rs:23:5
2019-08-31T17:31:31.8896182Z   --> src/libsyntax_ext/format.rs:23:5
2019-08-31T17:31:31.8896559Z    |
2019-08-31T17:31:31.8896983Z 23 |     Placeholder(String),
2019-08-31T17:31:31.8897767Z 
2019-08-31T17:31:31.8898158Z error: variant is never constructed: `Count`
2019-08-31T17:31:31.8898560Z   --> src/libsyntax_ext/format.rs:24:5
2019-08-31T17:31:31.8898933Z    |
2019-08-31T17:31:31.8898933Z    |
2019-08-31T17:31:31.8899330Z 24 |     Count,
2019-08-31T17:31:31.8900178Z    |     ^^^^^
2019-08-31T17:31:31.8900362Z 
2019-08-31T17:31:31.8900752Z error: enum is never used: `Position`
2019-08-31T17:31:31.8901139Z   --> src/libsyntax_ext/format.rs:27:1
2019-08-31T17:31:31.8901492Z    |
2019-08-31T17:31:31.8901876Z 27 | enum Position {
2019-08-31T17:31:31.8902465Z 
2019-08-31T17:31:31.8902822Z error: struct is never constructed: `Context`
2019-08-31T17:31:31.8903211Z   --> src/libsyntax_ext/format.rs:32:1
2019-08-31T17:31:31.8903965Z    |
2019-08-31T17:31:31.8903965Z    |
2019-08-31T17:31:31.8904442Z 32 | struct Context<'a, 'b> {
2019-08-31T17:31:31.8915925Z    | ^^^^^^^^^^^^^^^^^^^^^^
2019-08-31T17:31:31.8918120Z 
2019-08-31T17:31:31.8919509Z error: function is never used: `parse_args`
2019-08-31T17:31:31.8925452Z    --> src/libsyntax_ext/format.rs:126:1
2019-08-31T17:31:31.8942817Z     |
2019-08-31T17:31:31.8944026Z 126 | / fn parse_args<'a>(
2019-08-31T17:31:31.8944730Z 127 | |     ecx: &mut ExtCtxt<'a>,
2019-08-31T17:31:31.8945207Z 128 | |     sp: Span,
2019-08-31T17:31:31.8946690Z 129 | |     tts: &[tokenstream::TokenTree]
2019-08-31T17:31:31.8946959Z ...   |
2019-08-31T17:31:31.8947264Z 194 | |     Ok((fmtstr, args, names))
2019-08-31T17:31:31.8947813Z     | |_^
2019-08-31T17:31:31.8947851Z 
2019-08-31T17:31:31.8947851Z 
2019-08-31T17:31:31.8948228Z error: method is never used: `resolve_name_inplace`
2019-08-31T17:31:31.8948557Z    --> src/libsyntax_ext/format.rs:198:5
2019-08-31T17:31:31.8948775Z     |
2019-08-31T17:31:31.8949221Z 198 |     fn resolve_name_inplace(&self, p: &mut parse::Piece<'_>) {
2019-08-31T17:31:31.8949574Z 
2019-08-31T17:31:31.8949574Z 
2019-08-31T17:31:31.8966696Z error: method is never used: `verify_piece`
2019-08-31T17:31:31.8967159Z    --> src/libsyntax_ext/format.rs:222:5
2019-08-31T17:31:31.8967410Z     |
2019-08-31T17:31:31.8967703Z 222 |     fn verify_piece(&mut self, p: &parse::Piece<'_>) {
2019-08-31T17:31:31.8968070Z 
2019-08-31T17:31:31.8968070Z 
2019-08-31T17:31:31.8983912Z error: method is never used: `verify_count`
2019-08-31T17:31:31.8985899Z    --> src/libsyntax_ext/format.rs:245:5
2019-08-31T17:31:31.8986179Z     |
2019-08-31T17:31:31.8986498Z 245 |     fn verify_count(&mut self, c: parse::Count) {
2019-08-31T17:31:31.8986836Z 
2019-08-31T17:31:31.8986836Z 
2019-08-31T17:31:31.9002459Z error: method is never used: `describe_num_args`
2019-08-31T17:31:31.9002794Z    --> src/libsyntax_ext/format.rs:258:5
2019-08-31T17:31:31.9003058Z     |
2019-08-31T17:31:31.9003872Z 258 |     fn describe_num_args(&self) -> Cow<'_, str> {
2019-08-31T17:31:31.9004325Z 
2019-08-31T17:31:31.9004325Z 
2019-08-31T17:31:31.9024801Z error: method is never used: `report_invalid_references`
2019-08-31T17:31:31.9025162Z    --> src/libsyntax_ext/format.rs:270:5
2019-08-31T17:31:31.9025421Z     |
2019-08-31T17:31:31.9025736Z 270 |     fn report_invalid_references(&self, numbered_position_args: bool) {
2019-08-31T17:31:31.9026111Z 
2019-08-31T17:31:31.9026111Z 
2019-08-31T17:31:31.9026383Z error: method is never used: `verify_arg_type`
2019-08-31T17:31:31.9026647Z    --> src/libsyntax_ext/format.rs:403:5
2019-08-31T17:31:31.9027041Z     |
2019-08-31T17:31:31.9028178Z 403 |     fn verify_arg_type(&mut self, arg: Position, ty: ArgumentType) {
2019-08-31T17:31:31.9029115Z 
2019-08-31T17:31:31.9029115Z 
2019-08-31T17:31:31.9066087Z error: method is never used: `build_index_map`
2019-08-31T17:31:31.9066491Z    --> src/libsyntax_ext/format.rs:454:5
2019-08-31T17:31:31.9066774Z     |
2019-08-31T17:31:31.9067064Z 454 |     fn build_index_map(&mut self) {
2019-08-31T17:31:31.9067632Z 
2019-08-31T17:31:31.9067632Z 
2019-08-31T17:31:31.9067922Z error: method is never used: `rtpath`
2019-08-31T17:31:31.9068181Z    --> src/libsyntax_ext/format.rs:473:5
2019-08-31T17:31:31.9068431Z     |
2019-08-31T17:31:31.9068744Z 473 |     fn rtpath(ecx: &ExtCtxt<'_>, s: &str) -> Vec<ast::Ident> {
2019-08-31T17:31:31.9069093Z 
2019-08-31T17:31:31.9069359Z error: method is never used: `build_count`
2019-08-31T17:31:31.9069732Z    --> src/libsyntax_ext/format.rs:477:5
2019-08-31T17:31:31.9069982Z     |
2019-08-31T17:31:31.9069982Z     |
2019-08-31T17:31:31.9070330Z 477 |     fn build_count(&self, c: parse::Count) -> P<ast::Expr> {
2019-08-31T17:31:31.9070706Z 
2019-08-31T17:31:31.9071000Z error: method is never used: `build_literal_string`
2019-08-31T17:31:31.9071285Z    --> src/libsyntax_ext/format.rs:504:5
2019-08-31T17:31:31.9071518Z     |
2019-08-31T17:31:31.9071518Z     |
2019-08-31T17:31:31.9071851Z 504 |     fn build_literal_string(&mut self) -> P<ast::Expr> {
2019-08-31T17:31:31.9072218Z 
2019-08-31T17:31:31.9072218Z 
2019-08-31T17:31:31.9191173Z error: method is never used: `build_piece`
2019-08-31T17:31:31.9191735Z    --> src/libsyntax_ext/format.rs:513:5
2019-08-31T17:31:31.9192350Z 513 | /     fn build_piece(
2019-08-31T17:31:31.9192695Z 514 | |         &mut self,
2019-08-31T17:31:31.9192695Z 514 | |         &mut self,
2019-08-31T17:31:31.9193050Z 515 | |         piece: &parse::Piece<'a>,
2019-08-31T17:31:31.9193756Z 516 | |         arg_index_consumed: &mut Vec<usize>,
2019-08-31T17:31:31.9194382Z 634 | |         }
2019-08-31T17:31:31.9194670Z 635 | |     }
2019-08-31T17:31:31.9194918Z     | |_____^
2019-08-31T17:31:31.9194972Z 
2019-08-31T17:31:31.9194972Z 
2019-08-31T17:31:31.9195473Z error: method is never used: `into_expr`
2019-08-31T17:31:31.9195755Z    --> src/libsyntax_ext/format.rs:639:5
2019-08-31T17:31:31.9196009Z     |
2019-08-31T17:31:31.9196318Z 639 |     fn into_expr(self) -> P<ast::Expr> {
2019-08-31T17:31:31.9196663Z 
2019-08-31T17:31:31.9196946Z error: method is never used: `format_arg`
2019-08-31T17:31:31.9197327Z    --> src/libsyntax_ext/format.rs:740:5
2019-08-31T17:31:31.9197593Z     |
2019-08-31T17:31:31.9197593Z     |
2019-08-31T17:31:31.9197928Z 740 | /     fn format_arg(
2019-08-31T17:31:31.9198404Z 741 | |         ecx: &ExtCtxt<'_>,
2019-08-31T17:31:31.9198951Z 742 | |         macsp: Span,
2019-08-31T17:31:31.9199274Z 743 | |         mut sp: Span,
2019-08-31T17:31:31.9199609Z ...   |
2019-08-31T17:31:31.9199987Z 790 | |         ecx.expr_call_global(macsp, path, vec![arg, ecx.expr_path(format_fn)])
2019-08-31T17:31:31.9200548Z     | |_____^
2019-08-31T17:31:31.9200601Z 
2019-08-31T17:31:31.9200861Z error: function is never used: `expand_format_args_impl`
2019-08-31T17:31:31.9201132Z    --> src/libsyntax_ext/format.rs:794:1
2019-08-31T17:31:31.9201132Z    --> src/libsyntax_ext/format.rs:794:1
2019-08-31T17:31:31.9201526Z     |
2019-08-31T17:31:31.9201881Z 794 | / fn expand_format_args_impl<'cx>(
2019-08-31T17:31:31.9202340Z 795 | |     ecx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.9202992Z 796 | |     mut sp: Span,
2019-08-31T17:31:31.9203314Z 797 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.9204237Z 809 | |     }
2019-08-31T17:31:31.9204528Z 810 | | }
2019-08-31T17:31:31.9204768Z     | |_^
2019-08-31T17:31:31.9204818Z 
2019-08-31T17:31:31.9204818Z 
2019-08-31T17:31:31.9205073Z error: function is never used: `expand_format_args`
2019-08-31T17:31:31.9205330Z    --> src/libsyntax_ext/format.rs:812:1
2019-08-31T17:31:31.9205561Z     |
2019-08-31T17:31:31.9205872Z 812 | / pub fn expand_format_args<'cx>(
2019-08-31T17:31:31.9206184Z 813 | |     ecx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.9206495Z 814 | |     sp: Span,
2019-08-31T17:31:31.9207119Z 815 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.9207415Z 816 | | ) -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.9207736Z 817 | |     expand_format_args_impl(ecx, sp, tts, false)
2019-08-31T17:31:31.9208307Z     | |_^
2019-08-31T17:31:31.9208346Z 
2019-08-31T17:31:31.9208346Z 
2019-08-31T17:31:31.9208609Z error: function is never used: `expand_format_args_nl`
2019-08-31T17:31:31.9208848Z    --> src/libsyntax_ext/format.rs:820:1
2019-08-31T17:31:31.9209067Z     |
2019-08-31T17:31:31.9209363Z 820 | / pub fn expand_format_args_nl<'cx>(
2019-08-31T17:31:31.9209653Z 821 | |     ecx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.9209943Z 822 | |     sp: Span,
2019-08-31T17:31:31.9210233Z 823 | |     tts: &[tokenstream::TokenTree],
2019-08-31T17:31:31.9210550Z 824 | | ) -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.9210857Z 825 | |     expand_format_args_impl(ecx, sp, tts, true)
2019-08-31T17:31:31.9211375Z     | |_^
2019-08-31T17:31:31.9211406Z 
2019-08-31T17:31:31.9211406Z 
2019-08-31T17:31:31.9351442Z error: function is never used: `expand_preparsed_format_args`
2019-08-31T17:31:31.9352232Z     --> src/libsyntax_ext/format.rs:830:1
2019-08-31T17:31:31.9352858Z      |
2019-08-31T17:31:31.9354045Z 830  | / pub fn expand_preparsed_format_args(
2019-08-31T17:31:31.9354881Z 831  | |     ecx: &mut ExtCtxt<'_>,
2019-08-31T17:31:31.9355473Z 832  | |     sp: Span,
2019-08-31T17:31:31.9355933Z 833  | |     efmt: P<ast::Expr>,
2019-08-31T17:31:31.9356582Z ...    |
2019-08-31T17:31:31.9357121Z 1197 | |     cx.into_expr()
2019-08-31T17:31:31.9363005Z      | |_^
2019-08-31T17:31:31.9363228Z 
2019-08-31T17:31:31.9363228Z 
2019-08-31T17:31:31.9363962Z error: method is never used: `as_str`
2019-08-31T17:31:31.9364283Z   --> src/libsyntax_ext/format_foreign.rs:15:9
2019-08-31T17:31:31.9364810Z 15 |         pub fn as_str(&self) -> &str {
2019-08-31T17:31:31.9365092Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-31T17:31:31.9365329Z 
2019-08-31T17:31:31.9365630Z error: method is never used: `translate`
---
2019-08-31T17:31:31.9368656Z 
2019-08-31T17:31:31.9368913Z error: method is never used: `translate`
2019-08-31T17:31:31.9369210Z    --> src/libsyntax_ext/format_foreign.rs:253:9
2019-08-31T17:31:31.9369441Z     |
2019-08-31T17:31:31.9369768Z 253 |         fn translate(&self, s: &mut String) -> std::fmt::Result {
2019-08-31T17:31:31.9370155Z 
2019-08-31T17:31:31.9370415Z error: function is never used: `iter_subs`
2019-08-31T17:31:31.9370717Z    --> src/libsyntax_ext/format_foreign.rs:267:5
2019-08-31T17:31:31.9370954Z     |
2019-08-31T17:31:31.9370954Z     |
2019-08-31T17:31:31.9371280Z 267 |     pub fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
2019-08-31T17:31:31.9371667Z 
2019-08-31T17:31:31.9371667Z 
2019-08-31T17:31:31.9371937Z error: struct is never constructed: `Substitutions`
2019-08-31T17:31:31.9372224Z    --> src/libsyntax_ext/format_foreign.rs:275:5
2019-08-31T17:31:31.9372766Z 275 |     pub struct Substitutions<'a> {
2019-08-31T17:31:31.9373072Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-31T17:31:31.9373119Z 
2019-08-31T17:31:31.9373119Z 
2019-08-31T17:31:31.9373378Z error: method is never used: `as_str`
2019-08-31T17:31:31.9374067Z    --> src/libsyntax_ext/format_foreign.rs:623:9
2019-08-31T17:31:31.9374596Z 623 |         pub fn as_str(&self) -> String {
2019-08-31T17:31:31.9374882Z     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-31T17:31:31.9374936Z 
2019-08-31T17:31:31.9375194Z error: method is never used: `translate`
---
2019-08-31T17:31:31.9376563Z 
2019-08-31T17:31:31.9377006Z error: function is never used: `iter_subs`
2019-08-31T17:31:31.9377265Z    --> src/libsyntax_ext/format_foreign.rs:657:5
2019-08-31T17:31:31.9377476Z     |
2019-08-31T17:31:31.9377871Z 657 |     pub fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
2019-08-31T17:31:31.9378242Z 
2019-08-31T17:31:31.9378242Z 
2019-08-31T17:31:31.9378505Z error: struct is never constructed: `Substitutions`
2019-08-31T17:31:31.9378771Z    --> src/libsyntax_ext/format_foreign.rs:665:5
2019-08-31T17:31:31.9379267Z 665 |     pub struct Substitutions<'a> {
2019-08-31T17:31:31.9379535Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-31T17:31:31.9379572Z 
2019-08-31T17:31:31.9379572Z 
2019-08-31T17:31:31.9379832Z error: function is never used: `expand_global_asm`
2019-08-31T17:31:31.9380243Z   --> src/libsyntax_ext/global_asm.rs:22:1
2019-08-31T17:31:31.9380469Z    |
2019-08-31T17:31:31.9380804Z 22 | / pub fn expand_global_asm<'cx>(cx: &'cx mut ExtCtxt<'_>,
2019-08-31T17:31:31.9381113Z 23 | |                               sp: Span,
2019-08-31T17:31:31.9381477Z 24 | |                               tts: &[tokenstream::TokenTree]) -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.9381815Z 25 | |     match parse_global_asm(cx, sp, tts) {
2019-08-31T17:31:31.9382350Z 42 | |     }
2019-08-31T17:31:31.9382629Z 43 | | }
2019-08-31T17:31:31.9382863Z    | |_^
2019-08-31T17:31:31.9382899Z 
2019-08-31T17:31:31.9382899Z 
2019-08-31T17:31:31.9423985Z error: function is never used: `parse_global_asm`
2019-08-31T17:31:31.9424327Z   --> src/libsyntax_ext/global_asm.rs:45:1
2019-08-31T17:31:31.9424583Z    |
2019-08-31T17:31:31.9424894Z 45 | / fn parse_global_asm<'a>(
2019-08-31T17:31:31.9425197Z 46 | |     cx: &mut ExtCtxt<'a>,
2019-08-31T17:31:31.9425506Z 47 | |     sp: Span,
2019-08-31T17:31:31.9425831Z 48 | |     tts: &[tokenstream::TokenTree]
2019-08-31T17:31:31.9426081Z ...  |
2019-08-31T17:31:31.9426408Z 64 | |     Ok(Some(ast::GlobalAsm { asm }))
2019-08-31T17:31:31.9428141Z    | |_^
2019-08-31T17:31:31.9428200Z 
2019-08-31T17:31:31.9467259Z error: function is never used: `expand_syntax_ext`
2019-08-31T17:31:31.9467706Z   --> src/libsyntax_ext/log_syntax.rs:6:1
2019-08-31T17:31:31.9467706Z   --> src/libsyntax_ext/log_syntax.rs:6:1
2019-08-31T17:31:31.9467944Z    |
2019-08-31T17:31:31.9468436Z 6  | / pub fn expand_syntax_ext<'cx>(_cx: &'cx mut base::ExtCtxt<'_>,
2019-08-31T17:31:31.9468803Z 7  | |                               sp: syntax_pos::Span,
2019-08-31T17:31:31.9469341Z 8  | |                               tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9469683Z 9  | |                               -> Box<dyn base::MacResult + 'cx> {
2019-08-31T17:31:31.9470093Z ...  |
2019-08-31T17:31:31.9470471Z 13 | |     base::DummyResult::any_valid(sp)
2019-08-31T17:31:31.9470981Z    | |_^
2019-08-31T17:31:31.9471016Z 
2019-08-31T17:31:31.9471016Z 
2019-08-31T17:31:31.9552565Z error: function is never used: `expand_line`
2019-08-31T17:31:31.9553161Z   --> src/libsyntax_ext/source_util.rs:19:1
2019-08-31T17:31:31.9553954Z    |
2019-08-31T17:31:31.9554564Z 19 | / pub fn expand_line(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9555094Z 20 | |                    -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9555579Z 21 | |     base::check_zero_tts(cx, sp, tts, "line!");
2019-08-31T17:31:31.9556439Z ...  |
2019-08-31T17:31:31.9556439Z ...  |
2019-08-31T17:31:31.9556946Z 26 | |     base::MacEager::expr(cx.expr_u32(topmost, loc.line as u32))
2019-08-31T17:31:31.9557920Z    | |_^
2019-08-31T17:31:31.9558088Z 
2019-08-31T17:31:31.9558088Z 
2019-08-31T17:31:31.9558456Z error: function is never used: `expand_column`
2019-08-31T17:31:31.9558705Z   --> src/libsyntax_ext/source_util.rs:30:1
2019-08-31T17:31:31.9559091Z    |
2019-08-31T17:31:31.9559573Z 30 | / pub fn expand_column(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9560071Z 31 | |                   -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9560534Z 32 | |     base::check_zero_tts(cx, sp, tts, "column!");
2019-08-31T17:31:31.9561778Z ...  |
2019-08-31T17:31:31.9561778Z ...  |
2019-08-31T17:31:31.9562270Z 37 | |     base::MacEager::expr(cx.expr_u32(topmost, loc.col.to_usize() as u32 + 1))
2019-08-31T17:31:31.9563115Z    | |_^
2019-08-31T17:31:31.9563264Z 
2019-08-31T17:31:31.9564352Z error: function is never used: `expand_file`
2019-08-31T17:31:31.9564968Z   --> src/libsyntax_ext/source_util.rs:43:1
2019-08-31T17:31:31.9564968Z   --> src/libsyntax_ext/source_util.rs:43:1
2019-08-31T17:31:31.9565376Z    |
2019-08-31T17:31:31.9565900Z 43 | / pub fn expand_file(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9566413Z 44 | |                    -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9567076Z 45 | |     base::check_zero_tts(cx, sp, tts, "file!");
2019-08-31T17:31:31.9567898Z ...  |
2019-08-31T17:31:31.9567898Z ...  |
2019-08-31T17:31:31.9568410Z 49 | |     base::MacEager::expr(cx.expr_str(topmost, Symbol::intern(&loc.file.name.to_string())))
2019-08-31T17:31:31.9569265Z    | |_^
2019-08-31T17:31:31.9569415Z 
2019-08-31T17:31:31.9569415Z 
2019-08-31T17:31:31.9569794Z error: function is never used: `expand_stringify`
2019-08-31T17:31:31.9570056Z   --> src/libsyntax_ext/source_util.rs:52:1
2019-08-31T17:31:31.9570418Z    |
2019-08-31T17:31:31.9570935Z 52 | / pub fn expand_stringify(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9571708Z 53 | |                         -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9572215Z 54 | |     let s = pprust::tts_to_string(tts);
2019-08-31T17:31:31.9572709Z 55 | |     base::MacEager::expr(cx.expr_str(sp, Symbol::intern(&s)))
2019-08-31T17:31:31.9574063Z    | |_^
2019-08-31T17:31:31.9574247Z 
2019-08-31T17:31:31.9574247Z 
2019-08-31T17:31:31.9574614Z error: function is never used: `expand_mod`
2019-08-31T17:31:31.9574901Z   --> src/libsyntax_ext/source_util.rs:58:1
2019-08-31T17:31:31.9575279Z    |
2019-08-31T17:31:31.9575821Z 58 | / pub fn expand_mod(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9576323Z 59 | |                   -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9577015Z 60 | |     base::check_zero_tts(cx, sp, tts, "module_path!");
2019-08-31T17:31:31.9577656Z 61 | |     let mod_path = &cx.current_expansion.module.mod_path;
2019-08-31T17:31:31.9578098Z ...  |
2019-08-31T17:31:31.9578642Z 64 | |     base::MacEager::expr(cx.expr_str(sp, Symbol::intern(&string)))
2019-08-31T17:31:31.9579680Z    | |_^
2019-08-31T17:31:31.9579843Z 
2019-08-31T17:31:31.9580377Z error: function is never used: `expand_include`
2019-08-31T17:31:31.9580698Z    --> src/libsyntax_ext/source_util.rs:70:1
2019-08-31T17:31:31.9580698Z    --> src/libsyntax_ext/source_util.rs:70:1
2019-08-31T17:31:31.9581112Z     |
2019-08-31T17:31:31.9581656Z 70  | / pub fn expand_include<'cx>(cx: &'cx mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9582189Z 71  | |                            -> Box<dyn base::MacResult+'cx> {
2019-08-31T17:31:31.9582718Z 72  | |     let file = match get_single_str_from_tts(cx, sp, tts, "include!") {
2019-08-31T17:31:31.9583221Z 73  | |         Some(f) => f,
2019-08-31T17:31:31.9584323Z ...   |
2019-08-31T17:31:31.9585434Z 104 | |     Box::new(ExpandResult { p })
2019-08-31T17:31:31.9621106Z     | |_^
2019-08-31T17:31:31.9621168Z 
2019-08-31T17:31:31.9621168Z 
2019-08-31T17:31:31.9621414Z error: function is never used: `expand_include_str`
2019-08-31T17:31:31.9621661Z    --> src/libsyntax_ext/source_util.rs:108:1
2019-08-31T17:31:31.9621900Z     |
2019-08-31T17:31:31.9622245Z 108 | / pub fn expand_include_str(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9622566Z 109 | |                           -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9622926Z 110 | |     let file = match get_single_str_from_tts(cx, sp, tts, "include_str!") {
2019-08-31T17:31:31.9623379Z 111 | |         Some(f) => f,
2019-08-31T17:31:31.9625028Z 130 | |     }
2019-08-31T17:31:31.9625323Z 131 | | }
2019-08-31T17:31:31.9625965Z     | |_^
2019-08-31T17:31:31.9626008Z 
2019-08-31T17:31:31.9626008Z 
2019-08-31T17:31:31.9626440Z error: function is never used: `expand_include_bytes`
2019-08-31T17:31:31.9627176Z    --> src/libsyntax_ext/source_util.rs:133:1
2019-08-31T17:31:31.9627591Z     |
2019-08-31T17:31:31.9628287Z 133 | / pub fn expand_include_bytes(cx: &mut ExtCtxt<'_>, sp: Span, tts: &[tokenstream::TokenTree])
2019-08-31T17:31:31.9628816Z 134 | |                             -> Box<dyn base::MacResult+'static> {
2019-08-31T17:31:31.9629422Z 135 | |     let file = match get_single_str_from_tts(cx, sp, tts, "include_bytes!") {
2019-08-31T17:31:31.9629945Z 136 | |         Some(f) => f,
2019-08-31T17:31:31.9630782Z 148 | |     }
2019-08-31T17:31:31.9631223Z 149 | | }
2019-08-31T17:31:31.9631607Z     | |_^
2019-08-31T17:31:31.9631775Z 
2019-08-31T17:31:31.9631775Z 
2019-08-31T17:31:31.9632148Z error: function is never used: `expand_trace_macros`
2019-08-31T17:31:31.9632407Z   --> src/libsyntax_ext/trace_macros.rs:6:1
2019-08-31T17:31:31.9632797Z    |
2019-08-31T17:31:31.9633274Z 6  | / pub fn expand_trace_macros(cx: &mut ExtCtxt<'_>,
2019-08-31T17:31:31.9634159Z 7  | |                            sp: Span,
2019-08-31T17:31:31.9634707Z 8  | |                            tt: &[TokenTree])
2019-08-31T17:31:31.9635228Z 9  | |                            -> Box<dyn base::MacResult + 'static> {
2019-08-31T17:31:31.9635657Z ...  |
2019-08-31T17:31:31.9636124Z 20 | |     base::DummyResult::any_valid(sp)
2019-08-31T17:31:31.9637150Z    | |_^
2019-08-31T17:31:31.9637308Z 
2019-08-31T17:31:32.0090730Z error: aborting due to 56 previous errors
2019-08-31T17:31:32.0091051Z 
---
2019-08-31T17:32:22.5355541Z == clock drift check ==
2019-08-31T17:32:22.5375622Z   local time: Sat Aug 31 17:32:22 UTC 2019
2019-08-31T17:32:22.6860805Z   network time: Sat, 31 Aug 2019 17:32:22 GMT
2019-08-31T17:32:22.6866488Z == end clock drift check ==
2019-08-31T17:32:23.6541432Z ##[error]Bash exited with code '1'.
2019-08-31T17:32:23.6579051Z ##[section]Starting: Checkout
2019-08-31T17:32:23.6582169Z ==============================================================================
2019-08-31T17:32:23.6582222Z Task         : Get sources
2019-08-31T17:32:23.6582269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
