\n"},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":104,"byte_end":106,"line_start":5,"line_end":5,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    42","highlight_start":5,"highlight_end":7}],"label":"expected type parameter `u32`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":78,"byte_end":81,"line_start":4,"line_end":4,"column_start":8,"column_end":11,"is_primary":false,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":8,"highlight_end":11}],"label":"this type parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":94,"byte_end":97,"line_start":4,"line_end":4,"column_start":24,"column_end":27,"is_primary":false,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":24,"highlight_end":27}],"label":"expected `u32` because of return type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `u32`\n   found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"type parameters must be constrained to match other types","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> tests/ui/builtin-type-shadow.rs:5:5\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |        ---             --- expected `u32` because of return type\n   |        |\n   |        this type parameter\nLL |     42\n   |     ^^ expected type parameter `u32`, found integer\n   |\n   = note: expected type `u32`\n              found type `{integer}`\n   = help: type parameters must be constrained to match other types\n   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters\n\n"}
2019-11-06T01:51:00.2844326Z {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
2019-11-06T01:51:00.2844459Z 
2019-11-06T01:51:00.2844936Z ------------------------------------------
2019-11-06T01:51:00.2844991Z 
---
2019-11-06T02:21:18.5986042Z 
2019-11-06T02:21:18.6256905Z error[E0599]: no method named `get` found for type `&rustc_mir::interpret::Memory<'mir, 'tcx, machine::Evaluator<'tcx>>` in the current scope
2019-11-06T02:21:18.6257437Z   --> src/tools/miri/src/intptrcast.rs:66:37
2019-11-06T02:21:18.6257734Z    |
2019-11-06T02:21:18.6258241Z 66 |                 if offset <= memory.get(alloc_id)?.size.bytes() {
2019-11-06T02:21:18.6258828Z    |                                     ^^^ method not found in `&rustc_mir::interpret::Memory<'mir, 'tcx, machine::Evaluator<'tcx>>`
2019-11-06T02:21:18.7481125Z error[E0599]: no method named `get_mut` found for type `rustc_mir::interpret::Memory<'_, '_, machine::Evaluator<'_>>` in the current scope
2019-11-06T02:21:18.7481555Z    --> src/tools/miri/src/eval.rs:167:36
2019-11-06T02:21:18.7481788Z     |
2019-11-06T02:21:18.7482497Z 167 |         let cmd_alloc = ecx.memory.get_mut(cmd_ptr.alloc_id)?;
---
2019-11-06T02:21:19.2972328Z   local time: Wed Nov  6 02:21:19 UTC 2019
2019-11-06T02:21:19.5710033Z   network time: Wed, 06 Nov 2019 02:21:19 GMT
2019-11-06T02:21:19.5711206Z == end clock drift check ==
2019-11-06T02:21:20.6332959Z 
2019-11-06T02:21:20.6452947Z ##[error]Bash exited with code '1'.
2019-11-06T02:21:20.6494054Z ##[section]Starting: Checkout
2019-11-06T02:21:20.6497086Z ==============================================================================
2019-11-06T02:21:20.6497181Z Task         : Get sources
2019-11-06T02:21:20.6497284Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
