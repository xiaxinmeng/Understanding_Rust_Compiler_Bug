
error[E0412]: cannot find type `ExpnId` in this scope
    --> src\libsyntax\ast.rs:1592:18
     |
1592 |     pub expn_id: ExpnId,
     |                  ^^^^^^ not found in this scope
error[E0412]: cannot find type `ExpnId` in this scope
    --> src\libsyntax\ast.rs:1592:18
     |
1592 |     pub expn_id: ExpnId,
     |                  ^^^^^^ not found in this scope
error[E0412]: cannot find type `ExpnId` in this scope
    --> src\libsyntax\ast.rs:1592:18
     |
1592 |     pub expn_id: ExpnId,
     |                  ^^^^^^ not found in this scope
   Compiling rls-span v0.1.0
error[E0204]: the trait `Copy` may not be implemented for this type
    --> src\libsyntax\ast.rs:1589:77
     |
1589 | #[derive(Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash, Debug, Copy)]
     |                                                                             ^^^^
...
1592 |     pub expn_id: ExpnId,
     |     ------------------- this field does not implement `Copy`
error: aborting due to previous error
   Compiling arena v0.0.0 (file:///C:/projects/rust/src/libarena)
error: Could not compile `syntax`.
Build failed, waiting for other jobs to finish...
error: build failed
