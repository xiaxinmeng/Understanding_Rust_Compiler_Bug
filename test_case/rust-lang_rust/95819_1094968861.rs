
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:7 ~ repeat_to_run_dtor_twice[6a18]::main) (NoSolution): could not prove Binder(TraitPredicate(<Foo as std::marker::Copy>, polarity:Positive), [])
  --> /home/ubuntu/rust5/src/test/ui/repeat-to-run-dtor-twice.rs:17:13
   |
LL |     let _ = [ a; 5 ];
   |             ^^^^^^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:149:13
