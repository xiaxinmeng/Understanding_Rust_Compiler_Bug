(lldb) bt
* thread #1, name = 'match', stop reason = breakpoint 1.1
  * frame #0: match`match::foo::h96fa2745b04f623a(a=1, (null)=(__0 = 2, __1 = 3)) at match.rs:4
    frame #1: match`match::main::h27865b1a408d32d7 at match.rs:9

(lldb) frame v
(long) a = 1
((i64, i64))  = (__0 = 2, __1 = 3)
