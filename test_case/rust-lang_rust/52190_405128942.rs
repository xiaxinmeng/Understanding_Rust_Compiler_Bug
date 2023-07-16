plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:42:10] 
[00:42:10] running 1567 tests
[00:42:13] ....................................F.......F..............F.FF...................................i.
[00:42:20] ....................................................................................................
[00:42:22] ....................................................................................................
[00:42:24] ....................................................................................................
[00:42:26] ....................................................................................................
[00:42:26] ....................................................................................................
[00:42:29] .................................F...........F........................................F.............
[00:42:32] .....................................................................................F.FF.F.........
[00:42:36] ......................................F.FF...FFF..F.F.....F....FF..F................................
[00:42:39] ....................................................................................................
[00:42:42] ........................................i........FF.FFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFF...F.
[00:42:45] F.FFFFF.FF.....FFFFFFFFFFFFF.Fi.....................................................................
[00:42:52] ....................................................................................................
[00:42:52] ....................................................................................................
[00:42:56] .................................................F...............F..............i...................
[00:42:59] failures:
[00:42:59] 
[00:42:59] ---- [ui] ui/borrowck/borrowck-closures-two-mut.rs stdout ----
[00:42:59] diff of stderr:
[00:42:59] diff of stderr:
[00:42:59] 
[00:42:59] 73 LL | }
[00:42:59] 74    | - first borrow ends here
[00:42:59] 75 
[00:42:59] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let c1 = to_fn_mut(|| x = 4);
[00:42:59] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:59] -    |                        first mutable borrow occurs here
[00:42:59] -    |                        first mutable borrow occurs here
[00:42:59] - LL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:59] -    |                        ^^ - borrow occurs due to use of `x` in closure
[00:42:59] -    |                        second mutable borrow occurs here
[00:42:59] -    |                        second mutable borrow occurs here
[00:42:59] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:59] - LL |     drop((c1, c2));
[00:42:59] -    |           -- borrow later used here
[00:42:59] - 
[00:42:59] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let c1 = to_fn_mut(|| set(&mut x));
[00:42:59] -    |                        --          - previous borrow occurs due to use of `x` in closure
[00:42:59] -    |                        first mutable borrow occurs here
[00:42:59] -    |                        first mutable borrow occurs here
[00:42:59] - LL |     let c2 = to_fn_mut(|| set(&mut x)); //~ ERROR cannot borrow `x` as mutable more than once
[00:42:59] -    |                        ^^          - borrow occurs due to use of `x` in closure
[00:42:59] -    |                        second mutable borrow occurs here
[00:42:59] -    |                        second mutable borrow occurs here
[00:42:59] - LL |     //~| ERROR cannot borrow `x` as mutable more than once
[00:42:59] - LL |     drop((c1, c2));
[00:42:59] -    |           -- borrow later used here
[00:42:59] - 
[00:42:59] - error[E0499]: cannot borrow `x` as mutable more than once at a time (Mir)
[00:42:59] -    |
[00:42:59] -    |
[00:42:59] - LL |     let c1 = to_fn_mut(|| x = 5);
[00:42:59] -    |                        -- - previous borrow occurs due to use of `x` in closure
[00:42:59] -    |     o_fn_mut(|| x = 4);","highlight_start":24,"highlight_end":26}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":850,"byte_end":852,"line_start":24,"line_end":24,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once","highlight_start":24,"highlight_end":26}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":994,"byte_end":995,"line_start":27,"line_end":27,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"first borrow ends here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":853,"byte_end":854,"line_start":24,"line_end":24,"column_start":27,"column_end":28,"is_primary":false,"text":[{"text":"    let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once","highlight_start":27,"highlight_end":28}],"label":"borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":819,"byte_end":820,"line_start":23,"line_end":23,"column_start":27,"column_end":28,"is_primary":false,"text":[{"text":"    let c1 = to_fn_mut(|| x = 4);","highlight_start":27,"highlight_end":28}],"label":"previous borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)\n  --> /checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs:24:24\n   |\nLL |     let c1 = to_fn_mut(|| x = 4);\n   |                        -- - previous borrow occurs due to use of `x` in closure\n   |                        |\n   |                        first mutable borrow occurs here\nLL |     let c2 = to_fn_mut(|| x = 5); //~ ERROR cannot borrow `x` as mutable more than once\n   |                        ^^ - borrow occurs due to use of `x` in closure\n   |                        |\n   |                        second mutable borrow occurs here\n...\nLL | }\n   | - first borrow ends here\n\n"}
[00:42:59] {"message":"cannot borrow `x` as mutable more than once at a time (Ast)","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n