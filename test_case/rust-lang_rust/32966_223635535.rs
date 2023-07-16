
tmp0 = <expr-a>;
...
tmp25 = <expr-z>;
x = Foo { a: tmp0, ..., z: tmp25 } // "aggregate rvalue" in MIR
