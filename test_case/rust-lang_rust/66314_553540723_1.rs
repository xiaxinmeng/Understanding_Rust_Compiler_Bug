
2019-11-13T18:34:56.0483901Z // For the purposes of this explanation, all of these
2019-11-13T18:34:56.0483901Z // For the purposes of this explanation, all of these
2019-11-13T18:34:56.0484071Z // different kinds of `fn` declarations are equivalent:
2019-11-13T18:34:56.0484745Z struct S;
2019-11-13T18:34:56.0485287Z fn foo(x: S) { /* ... */ }
2019-11-13T18:34:56.0485672Z extern "C" { fn foo(x: S); }
2019-11-13T18:34:56.0485811Z impl S { fn foo(self) { /* ... */ } }
2019-11-13T18:34:56.0486080Z 
2019-11-13T18:34:56.0486080Z 
2019-11-13T18:34:56.0486219Z the type of `foo` is **not** `fn(S)`, as one might expect.
2019-11-13T18:34:56.0486732Z Rather, it is a unique, zero-sized marker type written here as `typeof(foo)`.
2019-11-13T18:34:56.0486940Z However, `typeof(foo)` can be _coerced_ to a function pointer `fn(S)`,
2019-11-13T18:34:56.0487082Z so you rarely notice this:
2019-11-13T18:34:56.0487358Z 