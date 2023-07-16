
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:4:14: error: cannot strip expression in this position - outer attributes not allowed
    4 |   let _rse = r"";
      |              ^
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:5:15: error: cannot strip expression in this position - outer attributes not allowed
    5 |   let _rse2 = r##""##;
      |               ^
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:13:15: error: cannot strip expression in this position - outer attributes not allowed
   13 |   let _rbse = br"";
      |               ^
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:14:16: error: cannot strip expression in this position - outer attributes not allowed
   14 |   let _rbse2 = br##""##;
      |                ^
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:39:19: error: cannot strip expression in this position - outer attributes not allowed
   39 |   let _strhes80 = "\x80"; // { dg-error "out of range" }
      |                   ^
/home/nirmal/src/gccrs/gcc/testsuite/rust/compile/rawbytestring.rs:40:19: error: cannot strip expression in this position - outer attributes not allowed
   40 |   let _strhesff = "\xfF"; // { dg-error "out of range" }
      |                   ^
