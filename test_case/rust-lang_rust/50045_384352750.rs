plain
[00:25:17]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:25:33]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:31:01]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:31:01]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:31:46] ERROR 2018-04-25T16:23:56Z: rustc::middle::liveness: block targeted by break: {
[00:31:46]     ::std::ops::Try::from_ok({
[00:31:46]                                  let mut file =
[00:31:46]                                      match ::std::ops::Try::into_result(pretty::create_dump_file(infcx.tcx,
[00:31:46]                                                                                                  "regioncx.dot",
[00:31:46]                                                                                                  "nll",
[00:31:46]                                                                                                  &0,
[00:31:46]                                                                                                  source))
[00:31:46]                                          {
[00:31:46]                                          {
[00:31:46]                                          ::std::result::Result::Err(err) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              break
[00:31:46]                                                  ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                                  ,
[00:31:46]                                          ::std::result::Result::Ok(val) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              val,
[00:31:46]                                      };
[00:31:46]                                  match ::std::ops::Try::into_result(regioncx.dump_graphviz(&mut file))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                              })
[00:31:46] }
[00:31:46] ERROR 2018-04-25T16:23:56Z: rustc::middle::liveness: block targeted by break: {
[00:31:46]     ::std::ops::Try::from_ok({
[00:31:46]                                  let mut file =
[00:31:46]                                      match ::std::ops::Try::into_result(create_dump_file(tcx,
[00:31:46]                                                                                          "dot",
[00:31:46]                                                                                          pass_num,
[00:31:46]                                                                                          pass_name,
[00:31:46]                                                                                          disambiguator,
[00:31:46]                                                                                          source))
[00:31:46]                                          {
[00:31:46]                                          ::std::result::Result::Err(err) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              break
[00:31:46]                                                  ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                                  ,
[00:31:46]                                          ::std::result::Result::Ok(val) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              val,
[00:31:46]                                      };
[00:31:46]                                  match ::std::ops::Try::into_result(write_mir_fn_graphviz(tcx,
[00:31:46]                                                                                           source.def_id,
[00:31:46]                                                                                           mir,
[00:31:46]                                                                                           &mut file))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                              })
[00:31:46] }
[00:31:46] ERROR 2018-04-25T16:23:56Z: rustc::middle::liveness: block targeted by break: {
[00:31:46]     ::std::ops::Try::from_ok({
[00:31:46]                                  let mut file =
[00:31:46]                                      match ::std::ops::Try::into_result(create_dump_file(tcx,
[00:31:46]                                                                                          "mir",
[00:31:46]                                                                                          pass_num,
[00:31:46]                                                                                          pass_name,
[00:31:46]                                                                                          disambiguator,
[00:31:46]                                                                                          source))
[00:31:46]                                          {
[00:31:46]                                          ::std::result::Result::Err(err) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              break
[00:31:46]                                                  ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                                  ,
[00:31:46]                                          ::std::result::Result::Ok(val) =>
[00:31:46]                                              #[allow(unreachable_code)]
[00:31:46]                                              val,
[00:31:46]                                      };
[00:31:46]                                  match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1_formatted(&["// MIR for `",
[00:31:46]                                                                                                                           "`\n"],
[00:31:46]                                                                                                                         &match (&node_path,)
[00:31:46]                                                                                                                              {
[00:31:46]                                                                                                                              (arg0,)
[00:31:46]                                                                                                                              =>
[00:31:46]                                                                                                                              [<::fmt::ArgumentV1>::new(arg0,
[00:31:46]                                                                                                                                                        ::fmt::Display::fmt)],
[00:31:46]                                                                                                                          },
[00:31:46]                                                                                                                         &[::fmt::rt::v1::Argument{position:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::Position::At(0usize),
[00:31:46]                                                                                                                                                   format:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::FormatSpec{fill:
[00:31:46]                                                                                                                                                                                     ' ',
[00:31:46]                                                                                                                                                                                 align:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Alignment::Unknown,
[00:31:46]                                                                                                                                                                                 flags:
[00:31:46]                                                                                                                                                                                     0u32,
[00:31:46]                                                                                                                                                                                 precision:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,
[00:31:46]                                                                                                                                                                                 width:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,},}])))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1_formatted(&["// source = ",
[00:31:46]                                                                                                                           "\n"],
[00:31:46]                                                                                                                         &match (&source,)
[00:31:46]                                                                                                                              {
[00:31:46]                                                                                                                              (arg0,)
[00:31:46]                                                                                                                              =>
[00:31:46]                                                                                                                              [<::fmt::ArgumentV1>::new(arg0,
[00:31:46]                                                                                                                                                        ::fmt::Debug::fmt)],
[00:31:46]                                                                                                                          },
[00:31:46]                                                                                                                         &[::fmt::rt::v1::Argument{position:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::Position::At(0usize),
[00:31:46]                                                                                                                                                   format:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::FormatSpec{fill:
[00:31:46]                                                                                                                                                                                     ' ',
[00:31:46]                                                                                                                                                                                 align:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Alignment::Unknown,
[00:31:46]                                                                                                                                                                                 flags:
[00:31:46]                                                                                                                                                                                     0u32,
[00:31:46]                                                                                                                                                                                 precision:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,
[00:31:46]                                                                                                                                                                                 width:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,},}])))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1_formatted(&["// pass_name = ",
[00:31:46]                                                                                                                           "\n"],
[00:31:46]                                                                                                                         &match (&pass_name,)
[00:31:46]                                                                                                                              {
[00:31:46]                                                                                                                              (arg0,)
[00:31:46]                                                                                                                              =>
[00:31:46]                                                                                                                              [<::fmt::ArgumentV1>::new(arg0,
[00:31:46]                                                                                                                                                        ::fmt::Display::fmt)],
[00:31:46]                                                                                                                          },
[00:31:46]                                                                                                                         &[::fmt::rt::v1::Argument{position:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::Position::At(0usize),
[00:31:46]                                                                                                                                                   format:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::FormatSpec{fill:
[00:31:46]                                                                                                                                                                                     ' ',
[00:31:46]                                                                                                                                                                                 align:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Alignment::Unknown,
[00:31:46]                                                                                                                                                                                 flags:
[00:31:46]                                                                                                                                                                                     0u32,
[00:31:46]                                                                                                                                                                                 precision:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,
[00:31:46]                                                                                                                                                                                 width:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,},}])))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1_formatted(&["// disambiguator = ",
[00:31:46]                                                                                                                           "\n"],
[00:31:46]                                                                                                                         &match (&disambiguator,)
[00:31:46]                                                                                                                              {
[00:31:46]                                                                                                                              (arg0,)
[00:31:46]                                                                                                                              =>
[00:31:46]                                                                                                                              [<::fmt::ArgumentV1>::new(arg0,
[00:31:46]                                                                                                                                                        ::fmt::Display::fmt)],
[00:31:46]                                                                                                                          },
[00:31:46]                                                                                                                         &[::fmt::rt::v1::Argument{position:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::Position::At(0usize),
[00:31:46]                                                                                                                                                   format:
[00:31:46]                                                                                                                                                       ::fmt::rt::v1::FormatSpec{fill:
[00:31:46]                                                                                                                                                                                     ' ',
[00:31:46]                                                                                                                                                                                 align:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Alignment::Unknown,
[00:31:46]                                                                                                                                                                                 flags:
[00:31:46]                                                                                                                                                                                     0u32,
[00:31:46]                                                                                                                                                                                 precision:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,
[00:31:46]                                                                                                                                                                                 width:
[00:31:46]                                                                                                                                                                                     ::fmt::rt::v1::Count::Implied,},}])))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match mir.generator_layout {
[00:31:46]                                      Some(ref layout) => {
[00:31:46]                                          match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1_formatted(&["// generator_layout = ",
[00:31:46]                                                                                                                                   "\n"],
[00:31:46]                                                                                                                                 &match (&layout,)
[00:31:46]                                                                                                                                      {
[00:31:46]                                                                                                                                      (arg0,)
[00:31:46]                                                                                                                                      =>
[00:31:46]                                                                                                                                      [<::fmt::ArgumentV1>::new(arg0,
[00:31:46]                                                                                                                                                                ::fmt::Debug::fmt)],
[00:31:46]                                                                                                                                  },
[00:31:46]                                                                                                                                 &[::fmt::rt::v1::Argument{position:
[00:31:46]                                                                                                                                                               ::fmt::rt::v1::Position::At(0usize),
[00:31:46]                                                                                                                                                           format:
[00:31:46]                                                                                                                                                               ::fmt::rt::v1::FormatSpec{fill:
[00:31:46]                                                                                                                                                                                             ' ',
[00:31:46]                                                                                                                                                                                         align:
[00:31:46]                                                                                                                                                                                             ::fmt::rt::v1::Alignment::Unknown,
[00:31:46]                                                                                                                                                                                         flags:
[00:31:46]                                                                                                                                                                                             0u32,
[00:31:46]                                                                                                                                                                                         precision:
[00:31:46]                                                                                                                                                                                             ::fmt::rt::v1::Count::Implied,
[00:31:46]                                                                                                                                                                                         width:
[00:31:46]                                                                                                                                                                                             ::fmt::rt::v1::Count::Implied,},}])))
[00:31:46]                                              {
[00:31:46]                                              ::std::result::Result::Err(err)
[00:31:46]                                              =>
[00:31:46]                                                  #[allow(unreachable_code)]
[00:31:46]                                                  break
[00:31:46]                                                      ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                                      ,
[00:31:46]                                              ::std::result::Result::Ok(val) =>
[00:31:46]                                                  #[allow(unreachable_code)]
[00:31:46]                                                  val,
[00:31:46]                                          };
[00:31:46]                                      }
[00:31:46]                                      }
[00:31:46]                                      _ => (),
[00:31:46]                                  }
[00:31:46]                                  match ::std::ops::Try::into_result(file.write_fmt(<::fmt::Arguments>::new_v1(&["\n"],
[00:31:46]                                                                                                               &match ()
[00:31:46]                                                                                                                    {
[00:31:46]                                                                                                                    ()
[00:31:46]                                                                                                                    =>
[00:31:46]                                                                                                                    [],
[00:31:46]                                                                                                                })))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(extra_data(PassWhere::BeforeCFG,
[00:31:46]                                                                                &mut file))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(write_mir_fn(tcx,
[00:31:46]                                                                                  source,
[00:31:46]                                                                                  mir,
[00:31:46]                                                                                  &mut extra_data,
[00:31:46]                                                                                  &mut file))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                                  match ::std::ops::Try::into_result(extra_data(PassWhere::AfterCFG,
[00:31:46]                                                                                &mut file))
[00:31:46]                                      {
[00:31:46]                                      ::std::result::Result::Err(err) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          break
[00:31:46]                                              ::std::ops::Try::from_error(::std::convert::From::from(err))
[00:31:46]                                              ,
[00:31:46]                                      ::std::result::Result::Ok(val) =>
[00:31:46]                                          #[allow(unreachable_code)]
[00:31:46]                                          val,
[00:31:46]                                  };
[00:31:46]                              })
[00:31:46] }
[00:33:23]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:33:29]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:33:31]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:33:31]    Compiling rustc_save_analysis v0.0.0 (file:///checkout/src/librustc_save_analysis)
---
[00:46:10] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:20] ....................................................................................................
[00:46:26] ....................................................................................................
[00:46:31] ...........FF...............................................................F.......................
[00:46:44] ...........i........................................................................................
[00:46:50] ...........................ii.......................................................................
[00:46:57] ....................................................................................................
[00:47:03] ........i....................................................................
[00:47:03] ........i....................................................................
[00:47:03] failures:
[00:47:03] 
[00:47:03] ---- [ui] ui/label_break_value_illegal_uses.rs stdout ----
[00:47:03]  normalized stderr:
[00:47:03] error: expected one of `extern`, `fn`, or `{`, found `'b`
[00:47:03]   --> $DIR/label_break_value_illegal_uses.rs:14:12
[00:47:03]    |
[00:47:03] LL |     unsafe 'b: {} //~ ERROR TOOODOOO
[00:47:03]    |            ^^ expected one of `extern`, `fn`, or `{` here
[00:47:03] 
[00:47:03] error: expected `{`, found `'b`
[00:47:03]   --> $DIR/label_break_value_illegal_uses.rs:18:13
[00:47:03]    |
[00:47:03] LL |     if true 'b: {} //~ ERROR TOOODOOO
[00:47:03]    |     --      ^^----
[00:47:03]    |     |       |
[00:47:03]    |     |       help: try placing this code inside a block: `{ 'b: { } }`
[00:47:03]    |     this `if` statement has a condition, but no block
[00:47:03] 
[00:47:03] error: expected `{`, found `'b`
[00:47:03]   --> $DIR/label_break_value_illegal_uses.rs:22:21
[00:47:03]    |
[00:47:03] LL |     if true {} else 'b: {} //~ ERROR TOOODOOO
[00:47:03]    |                     ^^----
[00:47:03]    |                     |
[00:47:03]    |                     help: try placing this code inside a block: `{ 'b: { } }`
[00:47:03] 
[00:47:03] error: expected one of `.`, `?`, `{`, or an operator, found `'b`
[00:47:03]   --> $DIR/label_break_value_illegal_uses.rs:26:17
[00:47:03]    |
[00:47:03] LL |     match false 'b: {} //~ ERROR TOOODOOO
[00:47:03]    |                 ^^ expected one of `.`, `?`, `{`, or an operator here
[00:47:03] error: aborting due to 4 previous errors
[00:47:03] 
[00:47:03] 
[00:47:03] 
[00:47:03] 
[00:47:03] 
[00:47:03] The actual stderr differed from the expected stderr.
[00:47:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_illegal_uses.stderr
[00:47:03] To update references, run this command from build directory:
[00:47:03] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'label_break_value_illegal_uses.rs'
[00:47:03] error: 1 errors occurred comparing output.
[00:47:03] status: exit code: 101
[00:47:03] status: exit code: 101
[00:47:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label_break_value_illegal_uses.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--targestart":18,"line_end":18,"column_start":5,"column_end":7,"is_primary":false,"text":[{"text":"    if true 'b: {} //~ ERROR TOOODOOO","highlight_start":5,"highlight_end":7}],"label":"this `if` statement has a condition, but no block","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/label_break_value_illegal_uses.rs","byte_start":625,"byte_end":627,"line_start":18,"line_end":18,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    if true 'b: {} //~ ERROR TOOODOOO","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try placing this code inside a block","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_illegal_uses.rs","byte_start":625,"byte_end":631,"line_start":18,"line_end":18,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    if true 'b: {} //~ ERROR TOOODOOO","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":"{ 'b: { } }","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected `{`, found `'b`\n  --> /checkout/src/test/ui/label_break_value_illegal_uses.rs:18:13\n   |\nLL |     if true 'b: {} //~ ERROR TOOODOOO\n   |     --      ^^----\n   |     |       |\n   |     |       help: try placing this code inside a block: `{ 'b: { } }`\n   |     this `if` statement has a condition, but no block\n\n"}
[00:47:03] {"message":"expected `{`, found `'b`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_illegal_uses.rs","byte_start":694,"byte_end":696,"line_start":22,"line_end":22,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"    if true {} else 'b: {} //~ ERROR TOOODOOO","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try placing this code inside a block","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_illegal_uses.rs","byte_start":694,"byte_end":700,"line_start":22,"line_end":22,"column_start":21,"column_end":27,"is_primary":true,"text":[{"text":"    if true {} else 'b: {} //~ ERROR TOOODOOO","highlight_start":21,"highlight_end":27}],"label":null,"suggested_replacement":"{ 'b: { } }","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected `{`, found `'b`\n  --> /checkout/src/test/ui/label_break_value_illegal_uses.rs:22:21\n   |\nLL |     if true {} else 'b: {} //~ ERROR TOOODOOO\n   |                     ^^----\n   |                     |\n   |                     help: try placing this code inside a block: `{ 'b: { } }`\n\n"}
[00:47:03] {"message":"expected one of `.`, `?`, `{`, or an operator, found `'b`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/label_break_value_illegal_uses.rs","byte_start":760,"byte_end":762,"line_start":26,"line_end":26,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    match false 'b: {} //~ ERROR TOOODOOO","highlight_start":17,"highlight_end":19}],"label":"expected one of `.`, `?`, `{`, or an operator here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: expected one of `.`, `?`, `{`, or an operator, found `'b`\n  --> /checkout/src/test/ui/label_break_value_illegal_uses.rs:26:17\n   |\nLL |     match false 'b: {} //~ ERROR TOOODOOO\n   |                 ^^ expected one of `.`, `?`, `{`, or an operator here\n\n"}
[00:47:03] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:03] ------------------------------------------
[00:47:03] 
[00:47:03] 
[00:47:03] thread '[ui] ui/label_break_value_illegal_uses.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[00:47:03] 
[00:47:03] 
[00:47:03] ---- [ui] ui/label_break_value_unlabeled_break.rs stdout ----
[00:47:03]  
[00:47:03] error: /checkout/src/test/ui/label_break_value_unlabeled_break.rs:14: expected error not found: TOOODOOO
[00:47:03] 
[00:47:03] error: /checkout/src/test/ui/label_break_value_unlabeled_break.rs:22: expected error not found: TOOODOOO
[00:47:03] 
[00:47:03] error: 0 unexpected errors found, 2 expected errors not found
[00:47:03] status: exit code: 101
[00:47:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label_break_value_unlabeled_break.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_unlabeled_break.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label_break_value_unlabeled_break.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:03] not found errors (from test file): [
[00:47:03]     Error {
[00:47:03]         line_num: 14,
[00:47:03]         kind: Some(
[00:47:03]             Error
[00:47:03]         ),
[00:47:03]         msg: "TOOODOOO"
[00:47:03]     Error {
[00:47:03]         line_num: 22,
[00:47:03]         kind: Some(
[00:47:03]             Error
[00:47:03]             Error
[00:47:03]         ),
[00:47:03]         msg: "TOOODOOO"
[00:47:03] ]
[00:47:03] 
[00:47:03] 
[00:47:03] thread '[ui] ui/label_break_value_unlabeled_break.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1274:13
[00:47:03] ---- [ui] ui/loop-break-value-no-repeat.rs stdout ----
[00:47:03]  diff of stderr:
[00:47:03] 
[00:47:03] 2   --> $DIR/loop-break-value-no-repeat.rs:22:9
[00:47:03] 2   --> $DIR/loop-break-value-no-repeat.rs:22:9
[00:47:03] 3    |
[00:47:03] 4 LL |         break 22 //~ ERROR `break` with value from a `for` loop
[00:47:03] -    |         ^^^^^^^^ can only break with a value inside `loop`
[00:47:03] +    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
[00:47:03] 6 help: instead, use `break` on its own without a value inside this `for` loop
[00:47:03] 7    |
[00:47:03] 8 LL |         break //~ ERROR `break` with value from a `for` loop
[00:47:03] 
[00:47:03] The actual stderr differed from the expected stderr.
[00:47:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loop-break-value-no-repeat.stderr
[00:47:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loop-break-value-no-repeat.stderr
[00:47:03] To inux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:03] 
[00:47:03] 
[00:47:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:03] Build completed unsuccessfully in 0:02:25
[00:47:03] Build completed unsuccessfully in 0:02:25
[00:47:03] Makefile:58: recipe for target 'check' failed
[00:47:03] make: *** [check] Error 1
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:156d6d58:start=1524674353905893303,finish=1524674353912293346,duration=6400043
travis_fold:end:after_failure.3
