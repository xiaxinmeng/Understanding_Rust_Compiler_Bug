
zmd@ReflectiveCoherence:~/Code/Misc/repro_paren/src$ RUSTFLAGS="-Z unstable-options -Z external-macro-backtrace -F warnings" cargo +nightly run 
   Compiling libc v0.2.36
   Compiling memchr v1.0.2
   Compiling nom v3.2.1
   Compiling repro_paren v0.1.0 (file:///home/zmd/Code/Misc/repro_paren)
error: unnecessary parentheses around function argument
   --> <do_parse macros>:2:36
    |
1   |         ( __impl $ i : expr , $ consumed : expr , ( $ ( $ rest : expr ) , * ) ) => (
    |       __-
    |      |__|
    |     ||__|
    |    |||__|
    |   ||||
2   |   ||||  $ crate :: IResult :: Done ( $ i , ( $ ( $ rest ) , * ) ) ) ; (
    |   ||||                                     ^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
3   |   ||||  __impl $ i : expr , $ consumed : expr , $ field : ident : $ submac : ident ! (
4   |   ||||  $ ( $ args : tt ) * ) ) => (
...     ||||
49  |   ||||  let i_ = i . clone (  ) ; do_parse ! (
    |  _||||____________________________-
50  | | ||||  __impl i_ , $ consumed + (
51  | | ||||  $ crate :: InputLength :: input_len ( & ( $ i ) ) - $ crate :: InputLength ::
52  | | ||||  input_len ( & i ) ) , $ ( $ rest ) * ) } , } } ) ; (
    | |_||||_______________________________________- in this macro invocation (#5)
...     ||||
71  |   ||||  let $ field = o ; let i_ = i . clone (  ) ; do_parse ! (
    |  _||||______________________________________________-
72  | | ||||  __impl i_ , $ consumed + (
73  | | ||||  $ crate :: InputLength :: input_len ( & ( $ i ) ) - $ crate :: InputLength ::
74  | | ||||  input_len ( & i ) ) , $ ( $ rest ) * ) } , } } ) ; (
    | |_||||_______________________________________- in this macro invocation (#4)
...     ||||
112 |   ||||  { do_parse ! ( __impl $ i , 0usize , $ ( $ rest ) * ) } ) ; (
    |   ||||    --------------------------------------------------- in this macro invocation (#3)
...     ||||
123 |   ||||  ) ; ) ; ( $ e : ident ! >> $ ( $ rest : tt ) * ) => (
124 |   ||||  do_parse ! ( call ! ( $ e ) >> $ ( $ rest ) * ) ; ) ;
    |   ||||                                                      -
    |   ||||______________________________________________________|
    |   |||_______________________________________________________in this expansion of `do_parse!` (#2)
    |   ||________________________________________________________in this expansion of `do_parse!` (#3)
    |   |_________________________________________________________in this expansion of `do_parse!` (#4)
    |                                                             in this expansion of `do_parse!` (#5)
    | 
   ::: <named macros>
    |
1   |       / ( # $ ( $ args : tt ) * ) => ( named_attr ! ( # $ ( $ args ) * ) ; ) ; (
2   |       | $ name : ident ( $ i : ty ) -> $ o : ty , $ submac : ident ! (
3   |       | $ ( $ args : tt ) * ) ) => (
4   |       | # [ allow ( unused_variables ) ] fn $ name ( i : $ i ) -> $ crate :: IResult <
...         |
41  |       | crate :: IResult < & [ u8 ] , & [ u8 ] , u32 > {
42  |       | $ submac ! ( i , $ ( $ args ) * ) } ) ;
    |       |_______________________________________- in this expansion of `named!` (#1)
    | 
   ::: src/main.rs
    |
4   |             named!(pub parse_string <String>, do_parse!(
    |  ___________-_________________________________-
    | | __________|
    | ||
5   | ||              value: map_res!(take_until!("\x00"), |value: &[u8]| String::from_utf8(value.to_vec())) >>
6   | ||                  tag!(b"\x00") >>
7   | ||                  (value)
8   | ||          ));
    | ||____________- in this macro invocation (#1)
...   |
    |
    = note: `-F unused-parens` implied by `-F warnings`

error: aborting due to previous error

error: Could not compile `repro_paren`.

To learn more, run the command again with --verbose.
