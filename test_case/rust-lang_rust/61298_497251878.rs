plain
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:30:28
[01:34:51]    |
[01:34:51] LL | pub fn test5(foo: &mut Box<Any>) {
[01:34:51]    |                            ^^^ help: use `dyn`: `dyn Any`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:35:19
[01:34:51]    |
[01:34:51] LL |     let foo: &Box<Any>;
---
[01:34:51] 
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:43:22
[01:34:51]    |
[01:34:51] LL |     fn test8(a: &Box<Any>);
[01:34:51]    |                      ^^^ help: use `dyn`: `dyn Any`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:47:22
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn test8(a: &Box<Any>) {
[01:34:51]    |                      ^^^ help: use `dyn`: `dyn Any`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:52:28
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | pub fn test9(foo: &mut Box<Any + Send + Sync>) {
[01:34:51]    |                            ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:57:19
[01:34:51]    |
[01:34:51] LL |     let foo: &Box<Any + Send + 'static>;
---
[01:34:51] 
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:65:22
[01:34:51]    |
[01:34:51] LL |     fn test4(a: &Box<Any + 'static>);
[01:34:51]    |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:69:22
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn test4(a: &Box<Any + 'static>) {
[01:34:51]    |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:77:40
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     test5(&mut (Box::new(false) as Box<Any>));
[01:34:51]    |                                        ^^^ help: use `dyn`: `dyn Any`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/borrow_box.rs:79:40
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     test9(&mut (Box::new(false) as Box<Any + Send + Sync>));
[01:34:51]    |                                        ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[01:34:51] error: aborting due to 12 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | pub fn test1(foo: &mut Box<bool>) {
[01:34:51]    |                   ^^^^^^^^^^^^^^ help: try: `&mut bool`
[01:34:51] note: lint level defined here
[01:34:51]   --> $DIR/borrow_box.rs:1:9
[01:34:51]    |
[01:34:51] LL | #![deny(clippy::borrowed_box)]
[01:34:51] LL | #![deny(clippy::borrowed_box)]
[01:34:51]    |         ^^^^^^^^^^^^^^^^^^^^
[01:34:51] 
[01:34:51] error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51]    |
[01:34:51] LL |     let foo: &Box<bool>;
[01:34:51]    |              ^^^^^^^^^^ help: try: `&bool`
[01:34:51] 
[01:34:51] 
[01:34:51] error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51]    |
[01:34:51] LL |     foo: &'a Box<bool>,
[01:34:51]    |          ^^^^^^^^^^^^^ help: try: `&'a bool`
[01:34:51] 
[01:34:51] 
[01:34:51] error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn test4(a: &Box<bool>);
[01:34:51]    |                 ^^^^^^^^^^ help: try: `&bool`
[01:34:51] error: aborting due to 4 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] diff of stderr:
[01:34:51] 
[01:34:51] -error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:30:28
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | pub fn test1(foo: &mut Box<bool>) {
[01:34:51] -   |                   ^^^^^^^^^^^^^^ help: try: `&mut bool`
[01:34:51] +LL | pub fn test5(foo: &mut Box<Any>) {
[01:34:51] +   |                            ^^^ help: use `dyn`: `dyn Any`
[01:34:51] -note: lint level defined here
[01:34:51] -  --> $DIR/borrow_box.rs:1:9
[01:34:51] -  --> $DIR/borrow_box.rs:1:9
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:35:19
[01:34:51]     |
[01:34:51] -LL | #![deny(clippy::borrowed_box)]
[01:34:51] -LL | #![deny(clippy::borrowed_box)]
[01:34:51] -   |         ^^^^^^^^^^^^^^^^^^^^
[01:34:51] +LL |     let foo: &Box<Any>;
[01:34:51] +   |                   ^^^ help: use `dyn`: `dyn Any`
[01:34:51]  
[01:34:51] -error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:39:18
[01:34:51]     |
[01:34:51] -LL |     let foo: &Box<bool>;
[01:34:51] -LL |     let foo: &Box<bool>;
[01:34:51] -   |              ^^^^^^^^^^ help: try: `&bool`
[01:34:51] +LL |     foo: &'a Box<Any>,
[01:34:51] +   |                  ^^^ help: use `dyn`: `dyn Any`
[01:34:51]  
[01:34:51] -error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:43:22
[01:34:51]     |
[01:34:51] -LL |     foo: &'a Box<bool>,
[01:34:51] -LL |     foo: &'a Box<bool>,
[01:34:51] -   |          ^^^^^^^^^^^^^ help: try: `&'a bool`
[01:34:51] +LL |     fn test8(a: &Box<Any>);
[01:34:51] +   |                      ^^^ help: use `dyn`: `dyn Any`
[01:34:51]  
[01:34:51] -error: you seem to be trying to use `&Box<T>`. Consider using just `&T`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:47:22
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL |     fn test4(a: &Box<bool>);
[01:34:51] -   |                 ^^^^^^^^^^ help: try: `&bool`
[01:34:51] +LL |     fn test8(a: &Box<Any>) {
[01:34:51] +   |                      ^^^ help: use `dyn`: `dyn Any`
[01:34:51] -error: aborting due to 4 previous errors
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:52:28
[01:34:51] +   |
[01:34:51] +   |
[01:34:51] +LL | pub fn test9(foo: &mut Box<Any + Send + Sync>) {
[01:34:51] +   |                            ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:57:19
[01:34:51] +   |
[01:34:51] +LL |     let foo: &Box<Any + Send + 'static>;
---
[01:34:51] +
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:65:22
[01:34:51] +   |
[01:34:51] +LL |     fn test4(a: &Box<Any + 'static>);
[01:34:51] +   |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:69:22
[01:34:51] +   |
[01:34:51] +   |
[01:34:51] +LL |     fn test4(a: &Box<Any + 'static>) {
[01:34:51] +   |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:77:40
[01:34:51] +   |
[01:34:51] +   |
[01:34:51] +LL |     test5(&mut (Box::new(false) as Box<Any>));
[01:34:51] +   |                                        ^^^ help: use `dyn`: `dyn Any`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/borrow_box.rs:79:40
[01:34:51] +   |
[01:34:51] +   |
[01:34:51] +LL |     test9(&mut (Box::new(false) as Box<Any + Send + Sync>));
[01:34:51] +   |                                        ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[01:34:51] +error: aborting due to 12 previous errors
[01:34:51]  
[01:34:51]  
[01:34:51] 
[01:34:51] 
[01:34:51] The actual stderr differed from the expected stderr.
[01:34:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/borrow_box.stderr
[01:34:51] To update references, run this command from build directory:
[01:34:51] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'borrow_box.rs'
[01:34:51] 
[01:34:51] error: 1 errors occurred comparing output.
[01:34:51] status: exit code: 1
[01:34:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/borrow_box.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/borrow_box.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/borrow_box.stage-id.aux" "-A" "unused"
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] stderr:
[01:34:51] stderr:
[01:34:51] ------------------------------------------
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":462,"byte_end":465,"line_start":30,"line_end":30,"column_start":28,"column_end":31,"is_primary":true,"text":[{"text":"pub fn test5(foo: &mut Box<Any>) {","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":462,"byte_end":465,"line_start":30,"line_end":30,"column_start":28,"column_end":31,"is_primary":true,"text":[{"text":"pub fn test5(foo: &mut Box<Any>) {","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:30:28\n   |\nLL | pub fn test5(foo: &mut Box<Any>) {\n   |                            ^^^ help: use `dyn`: `dyn Any`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":534,"byte_end":537,"line_start":35,"line_end":35,"column_start":19,"column_end":22,"is_primary":true,"text":[{"text":"    let foo: &Box<Any>;","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":534,"byte_end":537,"line_start":35,"line_end":35,"column_start":19,"column_end":22,"is_primary":true,"text":[{"text":"    let foo: &Box<Any>;","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:35:19\n   |\nLL |     let foo: &Box<Any>;\n   |                   ^^^ help: use `dyn`: `dyn Any`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":579,"byte_end":582,"line_start":39,"line_end":39,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    foo: &'a Box<Any>,","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":579,"byte_end":582,"line_start":39,"line_end":39,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    foo: &'a Box<Any>,","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:39:18\n   |\nLL |     foo: &'a Box<Any>,\n   |                  ^^^ help: use `dyn`: `dyn Any`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":623,"byte_end":626,"line_start":43,"line_end":43,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"    fn test8(a: &Box<Any>);","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":623,"byte_end":626,"line_start":43,"line_end":43,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"    fn test8(a: &Box<Any>);","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:43:22\n   |\nLL |     fn test8(a: &Box<Any>);\n   |                      ^^^ help: use `dyn`: `dyn Any`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":685,"byte_end":688,"line_start":47,"line_end":47,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"    fn test8(a: &Box<Any>) {","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":685,"byte_end":688,"line_start":47,"line_end":47,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"    fn test8(a: &Box<Any>) {","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:47:22\n   |\nLL |     fn test8(a: &Box<Any>) {\n   |                      ^^^ help: use `dyn`: `dyn Any`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":755,"byte_end":772,"line_start":52,"line_end":52,"column_start":28,"column_end":45,"is_primary":true,"text":[{"text":"pub fn test9(foo: &mut Box<Any + Send + Sync>) {","highlight_start":28,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":755,"byte_end":772,"line_start":52,"line_end":52,"column_start":28,"column_end":45,"is_primary":true,"text":[{"text":"pub fn test9(foo: &mut Box<Any + Send + Sync>) {","highlight_start":28,"highlight_end":45}],"label":null,"suggested_replacement":"dyn Any + Send + Sync","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:52:28\n   |\nLL | pub fn test9(foo: &mut Box<Any + Send + Sync>) {\n   |                            ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":833,"byte_end":853,"line_start":57,"line_end":57,"column_start":19,"column_end":39,"is_primary":true,"text":[{"text":"    let foo: &Box<Any + Send + 'static>;","highlight_start":19,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":833,"byte_end":853,"line_start":57,"line_end":57,"column_start":19,"column_end":39,"is_primary":true,"text":[{"text":"    let foo: &Box<Any + Send + 'static>;","highlight_start":19,"highlight_end":39}],"label":null,"suggested_replacement":"dyn Any + Send + 'static","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:57:19\n   |\nLL |     let foo: &Box<Any + Send + 'static>;\n   |                   ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + 'static`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":896,"byte_end":906,"line_start":61,"line_end":61,"column_start":18,"column_end":28,"is_primary":true,"text":[{"text":"    foo: &'a Box<Any + Send>,","highlight_start":18,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":896,"byte_end":906,"line_start":61,"line_end":61,"column_start":18,"column_end":28,"is_primary":true,"text":[{"text":"    foo: &'a Box<Any + Send>,","highlight_start":18,"highlight_end":28}],"label":null,"suggested_replacement":"dyn Any + Send","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:61:18\n   |\nLL |     foo: &'a Box<Any + Send>,\n   |                  ^^^^^^^^^^ help: use `dyn`: `dyn Any + Send`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":948,"byte_end":961,"line_start":65,"line_end":65,"column_start":22,"column_end":35,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<Any + 'static>);","highlight_start":22,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":948,"byte_end":961,"line_start":65,"line_end":65,"column_start":22,"column_end":35,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<Any + 'static>);","highlight_start":22,"highlight_end":35}],"label":null,"suggested_replacement":"dyn Any + 'static","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:65:22\n   |\nLL |     fn test4(a: &Box<Any + 'static>);\n   |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1022,"byte_end":1035,"line_start":69,"line_end":69,"column_start":22,"column_end":35,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<Any + 'static>) {","highlight_start":22,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1022,"byte_end":1035,"line_start":69,"line_end":69,"column_start":22,"column_end":35,"is_primary":true,"text":[{"text":"    fn test4(a: &Box<Any + 'static>) {","highlight_start":22,"highlight_end":35}],"label":null,"suggested_replacement":"dyn Any + 'static","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:69:22\n   |\nLL |     fn test4(a: &Box<Any + 'static>) {\n   |                      ^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + 'static`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1172,"byte_end":1175,"line_start":77,"line_end":77,"column_start":40,"column_end":43,"is_primary":true,"text":[{"text":"    test5(&mut (Box::new(false) as Box<Any>));","highlight_start":40,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1172,"byte_end":1175,"line_start":77,"line_end":77,"column_start":40,"column_end":43,"is_primary":true,"text":[{"text":"    test5(&mut (Box::new(false) as Box<Any>));","highlight_start":40,"highlight_end":43}],"label":null,"suggested_replacement":"dyn Any","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:77:40\n   |\nLL |     test5(&mut (Box::new(false) as Box<Any>));\n   |                                        ^^^ help: use `dyn`: `dyn Any`\n\n"}
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1232,"byte_end":1249,"line_start":79,"line_end":79,"column_start":40,"column_end":57,"is_primary":true,"text":[{"text":"    test9(&mut (Box::new(false) as Box<Any + Send + Sync>));","highlight_start":40,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_box.rs","byte_start":1232,"byte_end":1249,"line_start":79,"line_end":79,"column_start":40,"column_end":57,"is_primary":true,"text":[{"text":"    test9(&mut (Box::new(false) as Box<Any + Send + Sync>));","highlight_start":40,"highlight_end":57}],"label":null,"suggested_replacement":"dyn Any + Send + Sync","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/borrow_box.rs:79:40\n   |\nLL |     test9(&mut (Box::new(false) as Box<Any + Send + Sync>));\n   |                                        ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`\n\n"}
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] thread '[ui] ui/borrow_box.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] thread '[ui] ui/borrow_box.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:51] 
[01:34:51] ---- [ui] ui/box_vec.rs stdout ----
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/box_vec.rs:18:23
[01:34:51]    |
[01:34:51] LL | pub fn test2(foo: Box<Fn(Vec<u32>)>) {
[01:34:51]    |                       ^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(Vec<u32>)`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: aborting due to previous error
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: you seem to be trying to use `Box<Vec<T>>`. Consider using just `Vec<T>`
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | pub fn test(foo: Box<Vec<bool>>) {
[01:34:51]    |
[01:34:51]    |
[01:34:51]    = note: `-D clippy::box-vec` implied by `-D warnings`
[01:34:51]    = help: `Vec<T>` is already on the heap, `Box<Vec<T>>` makes an extra allocation.
[01:34:51] error: aborting due to previous error
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] diff of stderr:
[01:34:51] 
[01:34:51] -error: you seem to be trying to use `Box<Vec<T>>`. Consider using just `Vec<T>`
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/box_vec.rs:18:23
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | pub fn test(foo: Box<Vec<bool>>) {
[01:34:51] -   |                  ^^^^^^^^^^^^^^
[01:34:51] +LL | pub fn test2(foo: Box<Fn(Vec<u32>)>) {
[01:34:51] +   |                       ^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(Vec<u32>)`
[01:34:51]     |
[01:34:51] -   = note: `-D clippy::box-vec` implied by `-D warnings`
[01:34:51] -   = help: `Vec<T>` is already on the heap, `Box<Vec<T>>` makes an extra allocation.
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51]  error: aborting due to previous error
[01:34:51]  
[01:34:51]  
[01:34:51] 
[01:34:51] 
[01:34:51] The actual stderr differed from the expected stderr.
[01:34:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/box_vec.stderr
[01:34:51] To update references, run this command from build directory:
[01:34:51] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'box_vec.rs'
[01:34:51] 
[01:34:51] error: 1 errors occurred comparing output.
[01:34:51] status: exit code: 1
[01:34:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/box_vec.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/box_vec.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/box_vec.stage-id.aux" "-A" "unused"
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] stderr:
[01:34:51] stderr:
[01:34:51] ------------------------------------------
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/box_vec.rs","byte_start":370,"byte_end":382,"line_start":18,"line_end":18,"column_start":23,"column_end":35,"is_primary":true,"text":[{"text":"pub fn test2(foo: Box<Fn(Vec<u32>)>) {","highlight_start":23,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/box_vec.rs","byte_start":370,"byte_end":382,"line_start":18,"line_end":18,"column_start":23,"column_end":35,"is_primary":true,"text":[{"text":"pub fn test2(foo: Box<Fn(Vec<u32>)>) {","highlight_start":23,"highlight_end":35}],"label":null,"suggested_replacement":"dyn Fn(Vec<u32>)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/box_vec.rs:18:23\n   |\nLL | pub fn test2(foo: Box<Fn(Vec<u32>)>) {\n   |                       ^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(Vec<u32>)`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] thread '[ui] ui/box_vec.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] thread '[ui] ui/box_vec.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] 
[01:34:51] ---- [ui] ui/escape_analysis.rs stdout ----
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/escape_analysis.rs:24:35
[01:34:51]    |
[01:34:51] LL | fn ok_box_trait(boxed_trait: &Box<Z>) {
[01:34:51]    |                                   ^ help: use `dyn`: `dyn Z`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: aborting due to previous error
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: local variable doesn't need to be boxed here
[01:34:51]   --> $DIR/escape_analysis.rs:34:13
[01:34:51]    |
[01:34:51] LL | fn warn_arg(x: Box<A>) {
[01:34:51]    |
[01:34:51]    = note: `-D clippy::boxed-local` implied by `-D warnings`
[01:34:51] 
[01:34:51] error: local variable doesn't need to be boxed here
[01:34:51] error: local variable doesn't need to be boxed here
[01:34:51]   --> $DIR/escape_analysis.rs:125:12
[01:34:51]    |
[01:34:51] LL | pub fn new(_needs_name: Box<PeekableSeekable<&()>>) -> () {}
[01:34:51] 
[01:34:51] error: local variable doesn't need to be boxed here
[01:34:51]   --> $DIR/escape_analysis.rs:165:23
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn closure_borrow(x: Box<A>) {
[01:34:51] 
[01:34:51] error: aborting due to 3 previous errors
[01:34:51] 
[01:34:51] 
---
[01:34:51] -  --> $DIR/escape_analysis.rs:34:13
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/escape_analysis.rs:24:35
[01:34:51]     |
[01:34:51] -LL | fn warn_arg(x: Box<A>) {
[01:34:51] -   |             ^
[01:34:51] +LL | fn ok_box_trait(boxed_trait: &Box<Z>) {
[01:34:51] +   |                                   ^ help: use `dyn`: `dyn Z`
[01:34:51] -   = note: `-D clippy::boxed-local` implied by `-D warnings`
[01:34:51] -   = note: `-D clippy::boxed-local` implied by `-D warnings`
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] -error: local variable doesn't need to be boxed here
[01:34:51] -  --> $DIR/escape_analysis.rs:125:12
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | pub fn new(_needs_name: Box<PeekableSeekable<&()>>) -> () {}
[01:34:51] -
[01:34:51] -error: local variable doesn't need to be boxed here
[01:34:51] -  --> $DIR/escape_analysis.rs:165:23
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     fn closure_borrow(x: Box<A>) {
[01:34:51] -
[01:34:51] -error: aborting due to 3 previous errors
[01:34:51] +error: aborting due to previous error
[01:34:51]  
---
[01:34:51] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'escape_analysis.rs'
[01:34:51] 
[01:34:51] error: 1 errors occurred comparing output.
[01:34:51] status: exit code: 1
[01:34:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/escape_analysis.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/escape_analysis.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/escape_analysis.stage-id.aux" "-A" "unused"
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] stderr:
[01:34:51] stderr:
[01:34:51] ------------------------------------------
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/escape_analysis.rs","byte_start":343,"byte_end":344,"line_start":24,"line_end":24,"column_start":35,"column_end":36,"is_primary":true,"text":[{"text":"fn ok_box_trait(boxed_trait: &Box<Z>) {","highlight_start":35,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/escape_analysis.rs","byte_start":343,"byte_end":344,"line_start":24,"line_end":24,"column_start":35,"column_end":36,"is_primary":true,"text":[{"text":"fn ok_box_trait(boxed_trait: &Box<Z>) {","highlight_start":35,"highlight_end":36}],"label":null,"suggested_replacement":"dyn Z","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/escape_analysis.rs:24:35\n   |\nLL | fn ok_box_trait(boxed_trait: &Box<Z>) {\n   |                                   ^ help: use `dyn`: `dyn Z`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] thread '[ui] ui/escape_analysis.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] thread '[ui] ui/escape_analysis.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] 
[01:34:51] ---- [ui] ui/eta.rs stdout ----
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/eta.rs:35:23
[01:34:51]    |
[01:34:51] LL |     let a: Option<Box<::std::ops::Deref<Target = [i32]>>> =
[01:34:51]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn (::std::ops::Deref<Target = [i32]>)`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/eta.rs:36:44
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |         Some(vec![1i32, 2]).map(|v| -> Box<::std::ops::Deref<Target = [i32]>> { Box::new(v) });
[01:34:51]    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn (::std::ops::Deref<Target = [i32]>)`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/eta.rs:111:21
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | struct Thunk<T>(Box<FnMut() -> T>);
[01:34:51]    |                     ^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> T`
[01:34:51] error: aborting due to 3 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:21:27
[01:34:51]    |
[01:34:51] LL |     let a = Some(1u8).map(|a| foo(a));
[01:34:51]    |                           ^^^^^^^^^^ help: remove closure as shown: `foo`
[01:34:51]    = note: `-D clippy::redundant-closure` implied by `-D warnings`
[01:34:51] 
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:22:10
[01:34:51]   --> $DIR/eta.rs:22:10
[01:34:51]    |
[01:34:51] LL |     meta(|a| foo(a));
[01:34:51]    |          ^^^^^^^^^^ help: remove closure as shown: `foo`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:23:27
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let c = Some(1u8).map(|a| {1+2; foo}(a));
[01:34:51]    |                           ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `{1+2; foo}`
[01:34:51] error: this expression borrows a reference that is immediately dereferenced by the compiler
[01:34:51]   --> $DIR/eta.rs:25:21
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
[01:34:51]    |                     ^^^ help: change this to: `&2`
[01:34:51]    = note: `-D clippy::needless-borrow` implied by `-D warnings`
[01:34:51] 
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:32:27
[01:34:51]   --> $DIR/eta.rs:32:27
[01:34:51]    |
[01:34:51] LL |     let e = Some(1u8).map(|a| generic(a));
[01:34:51]    |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `generic`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:75:51
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
[01:34:51]    |                                                   ^^^^^^^^^^^ help: remove closure as shown: `TestStruct::foo`
[01:34:51]    = note: `-D clippy::redundant-closure-for-method-calls` implied by `-D warnings`
[01:34:51] 
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:77:51
[01:34:51]   --> $DIR/eta.rs:77:51
[01:34:51]    |
[01:34:51] LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
[01:34:51]    |                                                   ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `TestTrait::trait_foo`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:80:42
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
[01:34:51]    |                                          ^^^^^^^^^^^^^ help: remove closure as shown: `std::vec::Vec::clear`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:85:29
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let e = Some("str").map(|s| s.to_string());
[01:34:51]    |                             ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `std::string::ToString::to_string`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:87:27
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let e = Some('a').map(|s| s.to_uppercase());
[01:34:51]    |                           ^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_uppercase`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:90:65
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
[01:34:51]    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_ascii_uppercase`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:108:50
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let _: Vec<_> = arr.iter().map(|x| x.map_err(|e| some.take().unwrap()(e))).collect();
[01:34:51]    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `some.take().unwrap()`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:173:27
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let a = Some(1u8).map(|a| foo_ptr(a));
[01:34:51]    |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `foo_ptr`
[01:34:51] error: redundant closure found
[01:34:51]   --> $DIR/eta.rs:178:27
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let a = Some(1u8).map(|a| closure(a));
[01:34:51]    |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `closure`
[01:34:51] error: aborting due to 14 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] diff of stderr:
[01:34:51] 
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:21:27
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/eta.rs:35:23
[01:34:51]     |
[01:34:51] -LL |     let a = Some(1u8).map(|a| foo(a));
[01:34:51] -   |                           ^^^^^^^^^^ help: remove closure as shown: `foo`
[01:34:51] +LL |     let a: Option<Box<::std::ops::Deref<Target = [i32]>>> =
[01:34:51] +   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn (::std::ops::Deref<Target = [i32]>)`
[01:34:51] -   = note: `-D clippy::redundant-closure` implied by `-D warnings`
[01:34:51] -   = note: `-D clippy::redundant-closure` implied by `-D warnings`
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:22:10
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/eta.rs:36:44
[01:34:51] +  --> $DIR/eta.rs:36:44
[01:34:51]     |
[01:34:51] -LL |     meta(|a| foo(a));
[01:34:51] -   |          ^^^^^^^^^^ help: remove closure as shown: `foo`
[01:34:51] +LL |         Some(vec![1i32, 2]).map(|v| -> Box<::std::ops::Deref<Target = [i32]>> { Box::new(v) });
[01:34:51] +   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn (::std::ops::Deref<Target = [i32]>)`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:23:27
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/eta.rs:111:21
[01:34:51] +  --> $DIR/eta.rs:111:21
[01:34:51]     |
[01:34:51] -LL |     let c = Some(1u8).map(|a| {1+2; foo}(a));
[01:34:51] -   |                           ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `{1+2; foo}`
[01:34:51] +LL | struct Thunk<T>(Box<FnMut() -> T>);
[01:34:51] +   |                     ^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> T`
[01:34:51] -error: this expression borrows a reference that is immediately dereferenced by the compiler
[01:34:51] -  --> $DIR/eta.rs:25:21
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
[01:34:51] -   |                     ^^^ help: change this to: `&2`
[01:34:51] -   = note: `-D clippy::needless-borrow` implied by `-D warnings`
[01:34:51] -
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:32:27
[01:34:51] -  --> $DIR/eta.rs:32:27
[01:34:51] -   |
[01:34:51] -LL |     let e = Some(1u8).map(|a| generic(a));
[01:34:51] -   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `generic`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:75:51
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
[01:34:51] -   |                                                   ^^^^^^^^^^^ help: remove closure as shown: `TestStruct::foo`
[01:34:51] -   = note: `-D clippy::redundant-closure-for-method-calls` implied by `-D warnings`
[01:34:51] -
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:77:51
[01:34:51] -  --> $DIR/eta.rs:77:51
[01:34:51] -   |
[01:34:51] -LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
[01:34:51] -   |                                                   ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `TestTrait::trait_foo`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:80:42
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
[01:34:51] -   |                                          ^^^^^^^^^^^^^ help: remove closure as shown: `std::vec::Vec::clear`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:85:29
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let e = Some("str").map(|s| s.to_string());
[01:34:51] -   |                             ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `std::string::ToString::to_string`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:87:27
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let e = Some('a').map(|s| s.to_uppercase());
[01:34:51] -   |                           ^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_uppercase`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:90:65
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
[01:34:51] -   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_ascii_uppercase`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:108:50
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let _: Vec<_> = arr.iter().map(|x| x.map_err(|e| some.take().unwrap()(e))).collect();
[01:34:51] -   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `some.take().unwrap()`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:173:27
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let a = Some(1u8).map(|a| foo_ptr(a));
[01:34:51] -   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `foo_ptr`
[01:34:51] -error: redundant closure found
[01:34:51] -  --> $DIR/eta.rs:178:27
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     let a = Some(1u8).map(|a| closure(a));
[01:34:51] -   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `closure`
[01:34:51] -error: aborting due to 14 previous errors
[01:34:51] +error: aborting due to 3 previous errors
[01:34:51]  
[01:34:51]  
[01:34:51]  
[01:34:51] 
[01:34:51] The actual stderr differed from the expected stderr.
[01:34:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/eta.stderr
[01:34:51] normalized fixed:
[01:34:51] // run-rustfix
[01:34:51] 
[01:34:51] #![allow(
[01:34:51]     unused,
[01:34:51]     clippy::no_effect,
[01:34:51]     clippy::redundant_closure_call,
[01:34:51]     clippy::many_single_char_names,
[01:34:51]     clippy::needless_pass_by_value,
[01:34:51]     clippy::option_map_unit_fn,
[01:34:51]     clippy::trivially_copy_pass_by_ref
[01:34:51] )]
[01:34:51] #![warn(
[01:34:51]     clippy::redundant_closure,
[01:34:51]     clippy::redundant_closure_for_method_calls,
[01:34:51]     clippy::needless_borrow
[01:34:51] )]
[01:34:51] use std::path::PathBuf;
[01:34:51] 
[01:34:51] fn main() {
[01:34:51] fn main() {
[01:34:51]     let a = Some(1u8).map(|a| foo(a));
[01:34:51]     meta(|a| foo(a));
[01:34:51]     let c = Some(1u8).map(|a| {1+2; foo}(a));
[01:34:51]     let d = Some(1u8).map(|a| foo((|b| foo2(b))(a))); //is adjusted?
[01:34:51]     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
[01:34:51]     unsafe {
[01:34:51]         Some(1u8).map(|a| unsafe_fn(a)); // unsafe fn
[01:34:51] 
[01:34:51]     // See #815
[01:34:51]     // See #815
[01:34:51]     let e = Some(1u8).map(|a| divergent(a));
[01:34:51]     let e = Some(1u8).map(|a| generic(a));
[01:34:51]     let e = Some(1u8).map(generic);
[01:34:51]     // See #515
[01:34:51]     let a: Option<Box<dyn (::std::ops::Deref<Target = [i32]>)>> =
[01:34:51]         Some(vec![1i32, 2]).map(|v| -> Box<dyn (::std::ops::Deref<Target = [i32]>)> { Box::new(v) });
[01:34:51] }
[01:34:51] trait TestTrait {
[01:34:51] trait TestTrait {
[01:34:51]     fn trait_foo(self) -> bool;
[01:34:51]     fn trait_foo_ref(&self) -> bool;
[01:34:51] }
[01:34:51] struct TestStruct<'a> {
[01:34:51]     some_ref: &'a i32,
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> TestStruct<'a> {
[01:34:51]     fn foo(self) -> bool {
[01:34:51]     }
[01:34:51]     }
[01:34:51]     unsafe fn foo_unsafe(self) -> bool {
[01:34:51]         true
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> TestTrait for TestStruct<'a> {
[01:34:51]     fn trait_foo(self) -> bool {
[01:34:51]     }
[01:34:51]     }
[01:34:51]     fn trait_foo_ref(&self) -> bool {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> std::ops::Deref for TestStruct<'a> {
[01:34:51]     type Target = char;
[01:34:51]     fn deref(&self) -> &char {
[01:34:51]         &'a'
[01:34:51] }
[01:34:51] 
[01:34:51] fn test_redundant_closures_containing_method_calls() {
[01:34:51]     let i = 10;
[01:34:51]     let i = 10;
[01:34:51]     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
[01:34:51]     let e = Some(TestStruct { some_ref: &i }).map(TestStruct::foo);
[01:34:51]     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
[01:34:51]     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo_ref());
[01:34:51]     let e = Some(TestStruct { some_ref: &i }).map(TestTrait::trait_foo);
[01:34:51]     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
[01:34:51]     let e = Some(&mut vec![1, 2, 3]).map(std::vec::Vec::clear);
[01:34:51]     unsafe {
[01:34:51]         let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo_unsafe());
[01:34:51]     }
[01:34:51]     let e = Some("str").map(|s| s.to_string());
[01:34:51]     let e = Some("str").map(str::to_string);
[01:34:51]     let e = Some('a').map(|s| s.to_uppercase());
[01:34:51]     let e = Some('a').map(char::to_uppercase);
[01:34:51]     let e: std::vec::Vec<usize> = vec!['a', 'b', 'c'].iter().map(|c| c.len_utf8()).collect();
[01:34:51]     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
[01:34:51]     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(char::to_ascii_uppercase).collect();
[01:34:51]     let p = Some(PathBuf::new());
[01:34:51]     let e = p.as_ref().and_then(|s| s.to_str());
[01:34:51]     let c = Some(TestStruct { some_ref: &i })
[01:34:51]         .as_ref()
[01:34:51]         .map(|c| c.to_ascii_uppercase());
[01:34:51] 
[01:34:51]     fn test_different_borrow_levels<T>(t: &[&T])
[01:34:51]         T: TestTrait,
[01:34:51]     {
[01:34:51]     {
[01:34:51]         t.iter().filter(|x| x.trait_foo_ref());
[01:34:51]         t.iter().map(|x| x.trait_foo_ref());
[01:34:51] 
[01:34:51] 
[01:34:51]     let mut some = Some(|x| x * x);
[01:34:51]     let arr = [Ok(1), Err(2)];
[01:34:51]     let _: Vec<_> = arr.iter().map(|x| x.map_err(|e| some.take().unwrap()(e))).collect();
[01:34:51] }
[01:34:51] 
[01:34:51] struct Thunk<T>(Box<dyn FnMut() -> T>);
[01:34:51] 
[01:34:51] impl<T> Thunk<T> {
[01:34:51]     fn new<F: 'static + FnOnce() -> T>(f: F) -> Thunk<T> {
[01:34:51]         let mut option = Some(f);
[01:34:51]         // This should not trigger redundant_closure (#1439)
[01:34:51]         Thunk(Box::new(move || option.take().unwrap()()))
[01:34:51] 
[01:34:51] 
[01:34:51]     fn unwrap(self) -> T {
[01:34:51]         let Thunk(mut f) = self;
[01:34:51]         f()
[01:34:51] }
[01:34:51] 
[01:34:51] fn foobar() {
[01:34:51] fn foobar() {
[01:34:51]     let thunk = Thunk::new(|| println!("Hello, world!"));
[01:34:51]     thunk.unwrap()
[01:34:51] }
[01:34:51] 
[01:34:51] fn meta<F>(f: F)
[01:34:51]     F: Fn(u8),
[01:34:51] {
[01:34:51]     f(1u8)
[01:34:51] }
[01:34:51] }
[01:34:51] 
[01:34:51] fn foo(_: u8) {}
[01:34:51] 
[01:34:51] fn foo2(_: u8) -> u8 {
[01:34:51]     1u8
[01:34:51] }
[01:34:51] 
[01:34:51] fn all<X, F>(x: &[X], y: &X, f: F) -> bool
[01:34:51] where
[01:34:51]     F: Fn(&X, &X) -> bool,
[01:34:51] {
[01:34:51]     x.iter().all(|e| f(e, y))
[01:34:51] }
[01:34:51] 
[01:34:51] fn below(x: &u8, y: &u8) -> bool {
[01:34:51]     x < y
[01:34:51] }
[01:34:51] 
[01:34:51] unsafe fn unsafe_fn(_: u8) {}
[01:34:51] 
[01:34:51] fn divergent(_: u8) -> ! {
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] fn generic<T>(_: T) -> u8 {
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] fn passes_fn_mut(mut x: Box<dyn FnMut()>) {
[01:34:51]     requires_fn_once(|| x());
[01:34:51] }
[01:34:51] fn requires_fn_once<T: FnOnce()>(_: T) {}
[01:34:51] fn test_redundant_closure_with_function_pointer() {
[01:34:51] fn test_redundant_closure_with_function_pointer() {
[01:34:51]     type FnPtrType = fn(u8);
[01:34:51]     let foo_ptr: FnPtrType = foo;
[01:34:51]     let a = Some(1u8).map(|a| foo_ptr(a));
[01:34:51] }
[01:34:51] fn test_redundant_closure_with_another_closure() {
[01:34:51] fn test_redundant_closure_with_another_closure() {
[01:34:51]     let closure = |a| println!("{}", a);
[01:34:51]     let a = Some(1u8).map(|a| closure(a));
[01:34:51] }
[01:34:51] 
[01:34:51] expected fixed:
[01:34:51] // run-rustfix
[01:34:51] 
[01:34:51] 
[01:34:51] #![allow(
[01:34:51]     unused,
[01:34:51]     clippy::no_effect,
[01:34:51]     clippy::redundant_closure_call,
[01:34:51]     clippy::many_single_char_names,
[01:34:51]     clippy::needless_pass_by_value,
[01:34:51]     clippy::option_map_unit_fn,
[01:34:51]     clippy::trivially_copy_pass_by_ref
[01:34:51] )]
[01:34:51] #![warn(
[01:34:51]     clippy::redundant_closure,
[01:34:51]     clippy::redundant_closure_for_method_calls,
[01:34:51]     clippy::needless_borrow
[01:34:51] )]
[01:34:51] use std::path::PathBuf;
[01:34:51] 
[01:34:51] fn main() {
[01:34:51] fn main() {
[01:34:51]     let a = Some(1u8).map(foo);
[01:34:51]     meta(foo);
[01:34:51]     let c = Some(1u8).map({1+2; foo});
[01:34:51]     let d = Some(1u8).map(|a| foo((|b| foo2(b))(a))); //is adjusted?
[01:34:51]     all(&[1, 2, 3], &2, |x, y| below(x, y)); //is adjusted
[01:34:51]     unsafe {
[01:34:51]         Some(1u8).map(|a| unsafe_fn(a)); // unsafe fn
[01:34:51] 
[01:34:51]     // See #815
[01:34:51]     // See #815
[01:34:51]     let e = Some(1u8).map(|a| divergent(a));
[01:34:51]     let e = Some(1u8).map(generic);
[01:34:51]     let e = Some(1u8).map(generic);
[01:34:51]     // See #515
[01:34:51]     let a: Option<Box<::std::ops::Deref<Target = [i32]>>> =
[01:34:51]         Some(vec![1i32, 2]).map(|v| -> Box<::std::ops::Deref<Target = [i32]>> { Box::new(v) });
[01:34:51] }
[01:34:51] trait TestTrait {
[01:34:51] trait TestTrait {
[01:34:51]     fn trait_foo(self) -> bool;
[01:34:51]     fn trait_foo_ref(&self) -> bool;
[01:34:51] }
[01:34:51] struct TestStruct<'a> {
[01:34:51]     some_ref: &'a i32,
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> TestStruct<'a> {
[01:34:51]     fn foo(self) -> bool {
[01:34:51]     }
[01:34:51]     }
[01:34:51]     unsafe fn foo_unsafe(self) -> bool {
[01:34:51]         true
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> TestTrait for TestStruct<'a> {
[01:34:51]     fn trait_foo(self) -> bool {
[01:34:51]     }
[01:34:51]     }
[01:34:51]     fn trait_foo_ref(&self) -> bool {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] impl<'a> std::ops::Deref for TestStruct<'a> {
---
[01:34:51] -  --> $DIR/extra_unused_lifetimes.rs:14:14
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/extra_unused_lifetimes.rs:28:39
[01:34:51]     |
[01:34:51] -LL | fn unused_lt<'a>(x: u8) {}
[01:34:51] -   |              ^^
[01:34:51] +LL | fn unused_lt_blergh<'a>(x: Option<Box<Send + 'a>>) {}
[01:34:51] +   |                                       ^^^^^^^^^ help: use `dyn`: `dyn Send + 'a`
[01:34:51] -   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`
[01:34:51] -   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] -error: this lifetime isn't used in the function definition
[01:34:51] -  --> $DIR/extra_unused_lifetimes.rs:16:25
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | fn unused_lt_transitive<'a, 'b: 'a>(x: &'b u8) {
[01:34:51] -
[01:34:51] -error: this lifetime isn't used in the function definition
[01:34:51] -  --> $DIR/extra_unused_lifetimes.rs:41:10
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     fn x<'a>(&self) {}
[01:34:51] -
[01:34:51] -error: aborting due to 3 previous errors
[01:34:51] +error: aborting due to previous error
[01:34:51]  
---
[01:34:51] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'extra_unused_lifetimes.rs'
[01:34:51] 
[01:34:51] error: 1 errors occurred comparing output.
[01:34:51] status: exit code: 1
[01:34:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/extra_unused_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/extra_unused_lifetimes.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/extra_unused_lifetimes.stage-id.aux" "-A" "unused"
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] stderr:
[01:34:51] stderr:
[01:34:51] ------------------------------------------
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/extra_unused_lifetimes.rs","byte_start":524,"byte_end":533,"line_start":28,"line_end":28,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"fn unused_lt_blergh<'a>(x: Option<Box<Send + 'a>>) {}","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/extra_unused_lifetimes.rs","byte_start":524,"byte_end":533,"line_start":28,"line_end":28,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"fn unused_lt_blergh<'a>(x: Option<Box<Send + 'a>>) {}","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":"dyn Send + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/extra_unused_lifetimes.rs:28:39\n   |\nLL | fn unused_lt_blergh<'a>(x: Option<Box<Send + 'a>>) {}\n   |                                       ^^^^^^^^^ help: use `dyn`: `dyn Send + 'a`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] thread '[ui] ui/extra_unused_lifetimes.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] thread '[ui] ui/extra_unused_lifetimes.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] 
[01:34:51] ---- [ui] ui/len_zero.rs stdout ----
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/len_zero.rs:73:13
[01:34:51]    |
[01:34:51] LL |     let z: &TraitsToo = &y;
[01:34:51]    |             ^^^^^^^^^ help: use `dyn`: `dyn TraitsToo`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/len_zero.rs:128:25
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     let with_is_empty: &WithIsEmpty = &Wither;
[01:34:51]    |                         ^^^^^^^^^^^ help: use `dyn`: `dyn WithIsEmpty`
[01:34:51] error: aborting due to 2 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:61:8
[01:34:51]    |
[01:34:51] LL |     if x.len() == 0 {
[01:34:51]    |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `x.is_empty()`
[01:34:51]    = note: `-D clippy::len-zero` implied by `-D warnings`
[01:34:51] 
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:65:8
[01:34:51]   --> $DIR/len_zero.rs:65:8
[01:34:51]    |
[01:34:51] LL |     if "".len() == 0 {}
[01:34:51]    |        ^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `"".is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:80:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if has_is_empty.len() == 0 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:83:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if has_is_empty.len() != 0 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:86:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if has_is_empty.len() > 0 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to one
[01:34:51]   --> $DIR/len_zero.rs:89:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if has_is_empty.len() < 1 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] error: length comparison to one
[01:34:51]   --> $DIR/len_zero.rs:92:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if has_is_empty.len() >= 1 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:103:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if 0 == has_is_empty.len() {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:106:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if 0 != has_is_empty.len() {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:109:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if 0 < has_is_empty.len() {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to one
[01:34:51]   --> $DIR/len_zero.rs:112:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if 1 <= has_is_empty.len() {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] error: length comparison to one
[01:34:51]   --> $DIR/len_zero.rs:115:8
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     if 1 > has_is_empty.len() {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:129:8
[01:34:51]    |
[01:34:51] LL |     if with_is_empty.len() == 0 {
[01:34:51] LL |     if with_is_empty.len() == 0 {
[01:34:51]    |        ^^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `with_is_empty.is_empty()`
[01:34:51] error: length comparison to zero
[01:34:51]   --> $DIR/len_zero.rs:142:8
[01:34:51]    |
[01:34:51] LL |     if b.len() != 0 {}
[01:34:51] LL |     if b.len() != 0 {}
[01:34:51]    |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!b.is_empty()`
[01:34:51] error: aborting due to 14 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] diff of stderr:
[01:34:51] 
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:61:8
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/len_zero.rs:73:13
[01:34:51]     |
[01:34:51] -LL |     if x.len() == 0 {
[01:34:51] -   |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `x.is_empty()`
[01:34:51] +LL |     let z: &TraitsToo = &y;
[01:34:51] +   |             ^^^^^^^^^ help: use `dyn`: `dyn TraitsToo`
[01:34:51] -   = note: `-D clippy::len-zero` implied by `-D warnings`
[01:34:51] -   = note: `-D clippy::len-zero` implied by `-D warnings`
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:65:8
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/len_zero.rs:128:25
[01:34:51] +  --> $DIR/len_zero.rs:128:25
[01:34:51]     |
[01:34:51] -LL |     if "".len() == 0 {}
[01:34:51] -   |        ^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `"".is_empty()`
[01:34:51] +LL |     let with_is_empty: &WithIsEmpty = &Wither;
[01:34:51] +   |                         ^^^^^^^^^^^ help: use `dyn`: `dyn WithIsEmpty`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:80:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if has_is_empty.len() == 0 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:83:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if has_is_empty.len() != 0 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:86:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if has_is_empty.len() > 0 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to one
[01:34:51] -  --> $DIR/len_zero.rs:89:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if has_is_empty.len() < 1 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] -error: length comparison to one
[01:34:51] -  --> $DIR/len_zero.rs:92:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if has_is_empty.len() >= 1 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:103:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if 0 == has_is_empty.len() {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:106:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if 0 != has_is_empty.len() {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:109:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if 0 < has_is_empty.len() {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to one
[01:34:51] -  --> $DIR/len_zero.rs:112:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if 1 <= has_is_empty.len() {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!has_is_empty.is_empty()`
[01:34:51] -error: length comparison to one
[01:34:51] -  --> $DIR/len_zero.rs:115:8
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL |     if 1 > has_is_empty.len() {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `has_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:129:8
[01:34:51] -   |
[01:34:51] -LL |     if with_is_empty.len() == 0 {
[01:34:51] -LL |     if with_is_empty.len() == 0 {
[01:34:51] -   |        ^^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `with_is_empty.is_empty()`
[01:34:51] -error: length comparison to zero
[01:34:51] -  --> $DIR/len_zero.rs:142:8
[01:34:51] -   |
[01:34:51] -LL |     if b.len() != 0 {}
[01:34:51] -LL |     if b.len() != 0 {}
[01:34:51] -   |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `!b.is_empty()`
[01:34:51] -error: aborting due to 14 previous errors
[01:34:51] +error: aborting due to 2 previous errors
[01:34:51]  
[01:34:51]  
[01:34:51]  
[01:34:51] 
[01:34:51] The actual stderr differed from the expected stderr.
[01:34:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/len_zero.stderr
[01:34:51] normalized fixed:
[01:34:51] // run-rustfix
[01:34:51] 
[01:34:51] #![warn(clippy::len_zero)]
[01:34:51] #![allow(dead_code, unused, clippy::len_without_is_empty)]
[01:34:51] pub struct One;
[01:34:51] struct Wither;
[01:34:51] 
[01:34:51] trait TraitsToo {
[01:34:51] trait TraitsToo {
[01:34:51]     fn len(self: &Self) -> isize;
[01:34:51]     // No error; `len` is private; see issue #1085.
[01:34:51] }
[01:34:51] 
[01:34:51] impl TraitsToo for One {
[01:34:51]     fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] pub struct HasIsEmpty;
[01:34:51] 
[01:34:51] impl HasIsEmpty {
[01:34:51]     pub fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51]     fn is_empty(self: &Self) -> bool {
[01:34:51]         false
[01:34:51]         false
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] pub struct HasWrongIsEmpty;
[01:34:51] 
[01:34:51] impl HasWrongIsEmpty {
[01:34:51]     pub fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51] 
[01:34:51]     pub fn is_empty(self: &Self, x: u32) -> bool {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] pub trait WithIsEmpty {
[01:34:51]     fn len(self: &Self) -> isize;
[01:34:51]     fn is_empty(self: &Self) -> bool;
[01:34:51] }
[01:34:51] 
[01:34:51] impl WithIsEmpty for Wither {
[01:34:51]     fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51]     fn is_empty(self: &Self) -> bool {
[01:34:51]         false
[01:34:51]         false
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] fn main() {
[01:34:51]     let x = [1, 2];
[01:34:51]     if x.len() == 0 {
[01:34:51]         println!("This should not happen!");
[01:34:51] 
[01:34:51] 
[01:34:51]     if "".len() == 0 {}
[01:34:51]     let y = One;
[01:34:51]     if y.len() == 0 {
[01:34:51]     if y.len() == 0 {
[01:34:51]         // No error; `One` does not have `.is_empty()`.
[01:34:51]         println!("This should not happen either!");
[01:34:51] 
[01:34:51] 
[01:34:51]     let z: &dyn TraitsToo = &y;
[01:34:51]     if z.len() > 0 {
[01:34:51]         // No error; `TraitsToo` has no `.is_empty()` method.
[01:34:51]         println!("Nor should this!");
[01:34:51] 
[01:34:51] 
[01:34:51]     let has_is_empty = HasIsEmpty;
[01:34:51]     if has_is_empty.len() == 0 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() != 0 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() > 0 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() < 1 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() >= 1 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() > 1 {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     }
[01:34:51]     if has_is_empty.len() <= 1 {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     }
[01:34:51]     if 0 == has_is_empty.len() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 0 != has_is_empty.len() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 0 < has_is_empty.len() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 1 <= has_is_empty.len() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 1 > has_is_empty.len() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 1 < has_is_empty.len() {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     }
[01:34:51]     if 1 >= has_is_empty.len() {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     }
[01:34:51]     assert!(!has_is_empty.is_empty());
[01:34:51] 
[01:34:51]     let with_is_empty: &dyn WithIsEmpty = &Wither;
[01:34:51]     if with_is_empty.len() == 0 {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     assert!(!with_is_empty.is_empty());
[01:34:51] 
[01:34:51]     let has_wrong_is_empty = HasWrongIsEmpty;
[01:34:51]     if has_wrong_is_empty.len() == 0 {
[01:34:51]         // No error; `HasWrongIsEmpty` does not have `.is_empty()`.
[01:34:51]         println!("Or this!");
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] fn test_slice(b: &[u8]) {
[01:34:51]     if b.len() != 0 {}
[01:34:51] }
[01:34:51] 
[01:34:51] expected fixed:
[01:34:51] // run-rustfix
[01:34:51] 
[01:34:51] 
[01:34:51] #![warn(clippy::len_zero)]
[01:34:51] #![allow(dead_code, unused, clippy::len_without_is_empty)]
[01:34:51] pub struct One;
[01:34:51] struct Wither;
[01:34:51] 
[01:34:51] trait TraitsToo {
[01:34:51] trait TraitsToo {
[01:34:51]     fn len(self: &Self) -> isize;
[01:34:51]     // No error; `len` is private; see issue #1085.
[01:34:51] }
[01:34:51] 
[01:34:51] impl TraitsToo for One {
[01:34:51]     fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] pub struct HasIsEmpty;
[01:34:51] 
[01:34:51] impl HasIsEmpty {
[01:34:51]     pub fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51]     fn is_empty(self: &Self) -> bool {
[01:34:51]         false
[01:34:51]         false
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] pub struct HasWrongIsEmpty;
[01:34:51] 
[01:34:51] impl HasWrongIsEmpty {
[01:34:51]     pub fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51] 
[01:34:51]     pub fn is_empty(self: &Self, x: u32) -> bool {
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] 
[01:34:51] pub trait WithIsEmpty {
[01:34:51]     fn len(self: &Self) -> isize;
[01:34:51]     fn is_empty(self: &Self) -> bool;
[01:34:51] }
[01:34:51] 
[01:34:51] impl WithIsEmpty for Wither {
[01:34:51]     fn len(self: &Self) -> isize {
[01:34:51]     }
[01:34:51] 
[01:34:51]     fn is_empty(self: &Self) -> bool {
[01:34:51]         false
[01:34:51]         false
[01:34:51]     }
[01:34:51] }
[01:34:51] 
[01:34:51] fn main() {
[01:34:51]     let x = [1, 2];
[01:34:51]     if x.is_empty() {
[01:34:51]         println!("This should not happen!");
[01:34:51] 
[01:34:51] 
[01:34:51]     if "".is_empty() {}
[01:34:51]     let y = One;
[01:34:51]     if y.len() == 0 {
[01:34:51]     if y.len() == 0 {
[01:34:51]         // No error; `One` does not have `.is_empty()`.
[01:34:51]         println!("This should not happen either!");
[01:34:51] 
[01:34:51] 
[01:34:51]     let z: &TraitsToo = &y;
[01:34:51]     if z.len() > 0 {
[01:34:51]         // No error; `TraitsToo` has no `.is_empty()` method.
[01:34:51]         println!("Nor should this!");
[01:34:51] 
[01:34:51] 
[01:34:51]     let has_is_empty = HasIsEmpty;
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if has_is_empty.len() > 1 {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     }
[01:34:51]     if has_is_empty.len() <= 1 {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if !has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]     if has_is_empty.is_empty() {
[01:34:51]         println!("Or this!");
[01:34:51]     }
[01:34:51]     if 1 < has_is_empty.len() {
[01:34:51]         println!("This can happen.");
[01:34:51]     }
---
[01:34:51] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'needless_borrow.rs'
[01:34:51] 
[01:34:51] error: 1 errors occurred comparing output.
[01:34:51] status: exit code: 1
[01:34:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_borrow.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/needless_borrow.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/needless_borrow.stage-id.aux" "-A" "unused"
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] stderr:
[01:34:51] stderr:
[01:34:51] ------------------------------------------
[01:34:51] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow.rs","byte_start":1028,"byte_end":1033,"line_start":44,"line_end":44,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"fn h(_: &Trait) {}","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_borrow.rs","byte_start":1028,"byte_end":1033,"line_start":44,"line_end":44,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"fn h(_: &Trait) {}","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":"dyn Trait","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/needless_borrow.rs:44:10\n   |\nLL | fn h(_: &Trait) {}\n   |          ^^^^^ help: use `dyn`: `dyn Trait`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:51] 
[01:34:51] ------------------------------------------
[01:34:51] 
[01:34:51] thread '[ui] ui/needless_borrow.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] thread '[ui] ui/needless_borrow.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:51] 
[01:34:51] ---- [ui] ui/needless_lifetimes.rs stdout ----
[01:34:51] normalized stderr:
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/needless_lifetimes.rs:169:30
[01:34:51]    |
[01:34:51] LL | type WithLifetimeAlias<'a> = WithLifetime<'a>;
[01:34:51]    |                              ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime<'a>`
[01:34:51]    |
[01:34:51]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/needless_lifetimes.rs:172:35
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | fn trait_obj_elided<'a>(_arg: &'a WithLifetime) -> &'a str {
[01:34:51]    |                                   ^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/needless_lifetimes.rs:178:36
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {
[01:34:51]    |                                    ^^^^ help: use `dyn`: `dyn Drop`
[01:34:51] error: trait objects without an explicit `dyn` are deprecated
[01:34:51]   --> $DIR/needless_lifetimes.rs:229:34
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
[01:34:51]    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item = usize> + 'a`
[01:34:51] error: aborting due to 4 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] expected stderr:
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
[01:34:51]    |
[01:34:51]    = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
[01:34:51] LL | |     Ok(x)
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
[01:34:51] LL | | where
[01:34:51] LL | |     T: Copy,
[01:34:51] LL | | {
[01:34:51] LL | |     Ok(x)
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
[01:34:51] LL | | where
[01:34:51] LL | |     for<'x> F: Fn(Lt<'x, I>) -> Lt<'x, I>,
[01:34:51] LL | | {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | /     fn self_and_out<'s>(&'s self) -> &'s u8 {
[01:34:51] LL | |         &self.x
[01:34:51]    | |_____^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn named_input_elided_output<'a>(_arg: &'a str) -> &str {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] 
[01:34:51] error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51]    |
[01:34:51]    |
[01:34:51] LL | / fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {
[01:34:51] LL | | }
[01:34:51]    | |_^
[01:34:51] 
[01:34:51] error: aborting due to 15 previous errors
[01:34:51] error: aborting due to 15 previous errors
[01:34:51] 
[01:34:51] 
[01:34:51] 
[01:34:51] diff of stderr:
[01:34:51] 
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/needless_lifetimes.rs:169:30
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
[01:34:51] -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:51] +LL | type WithLifetimeAlias<'a> = WithLifetime<'a>;
[01:34:51] +   |                              ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime<'a>`
[01:34:51] -   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
[01:34:51] -   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
[01:34:51] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:51]  
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/needless_lifetimes.rs:172:35
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
[01:34:51] -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:51] +LL | fn trait_obj_elided<'a>(_arg: &'a WithLifetime) -> &'a str {
[01:34:51] +   |                                   ^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime`
[01:34:51]  
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/needless_lifetimes.rs:178:36
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | / fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
[01:34:51] -LL | | }
[01:34:51] -   | |_^
[01:34:51] -   | |_^
[01:34:51] +LL | fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {
[01:34:51] +   |                                    ^^^^ help: use `dyn`: `dyn Drop`
[01:34:51]  
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] +error: trait objects without an explicit `dyn` are deprecated
[01:34:51] +  --> $DIR/needless_lifetimes.rs:229:34
[01:34:51]     |
[01:34:51]     |
[01:34:51] -LL | / fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
[01:34:51] -LL | |     Ok(x)
[01:34:51] -   | |_^
[01:34:51] -   | |_^
[01:34:51] +LL |     fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
[01:34:51] +   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item = usize> + 'a`
[01:34:51]  
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | / fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
[01:34:51] -LL | | where
[01:34:51] -LL | |     T: Copy,
[01:34:51] -LL | | {
[01:34:51] -LL | |     Ok(x)
[01:34:51] -   | |_^
[01:34:51] -
[01:34:51] -
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
[01:34:51] -
[01:34:51] -
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | / fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
[01:34:51] -LL | | where
[01:34:51] -LL | |     for<'x> F: Fn(Lt<'x, I>) -> Lt<'x, I>,
[01:34:51] -LL | | {
[01:34:51] -LL | | }
[01:34:51] -   | |_^
[01:34:51] -
[01:34:51] -
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:51] -   |
[01:34:51] -   |
[01:34:51] -LL | /     fn self_and_out<'s>(&'s self) -> &'s u8 {
[01:34:51] -LL | |         &self.x
[01:34:51] -   | |_____^
[01:34:51] -
[01:34:51] -
[01:34:51] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52]    Compiling nodrop v0.1.12
[01:34:52]    Compiling memoffset v0.2.1
[01:34:52]    Compiling rustc-rayon-core v0.1.2
[01:34:52]    Compiling rustc-rayon v0.1.2
[01:34:52]    Compiling rustc-rayon v0.1.2
[01:34:52] [RUSTC-TIMING] memoffset test:false 0.273
[01:34:52]    Compiling rustc-ap-graphviz v407.0.0
[01:34:51] -  --> $DIR/needless_lifetimes.rs:129:5
[01:34:52] -   |
[01:34:52] -LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn named_input_elided_output<'a>(_arg: &'a str) -> &str {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | / fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {
[01:34:52] -LL | | }
[01:34:52] -   | |_^
[01:34:52] -
[01:34:52] -error: aborting due to 15 previous errors
---
[01:34:52] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'needless_lifetimes.rs'
[01:34:52] 
[01:34:52] error: 1 errors occurred comparing output.
[01:34:52] status: exit code: 1
[01:34:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/needless_lifetimes.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/needless_lifetimes.stage-id.aux" "-A" "unused"
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] stderr:
[01:34:52] stderr:
[01:34:52] ------------------------------------------
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3770,"byte_end":3786,"line_start":169,"line_end":169,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"type WithLifetimeAlias<'a> = WithLifetime<'a>;","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3770,"byte_end":3786,"line_start":169,"line_end":169,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"type WithLifetimeAlias<'a> = WithLifetime<'a>;","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":"dyn WithLifetime<'a>","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/needless_lifetimes.rs:169:30\n   |\nLL | type WithLifetimeAlias<'a> = WithLifetime<'a>;\n   |                              ^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime<'a>`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3887,"byte_end":3899,"line_start":172,"line_end":172,"column_start":35,"column_end":47,"is_primary":true,"text":[{"text":"fn trait_obj_elided<'a>(_arg: &'a WithLifetime) -> &'a str {","highlight_start":35,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3887,"byte_end":3899,"line_start":172,"line_end":172,"column_start":35,"column_end":47,"is_primary":true,"text":[{"text":"fn trait_obj_elided<'a>(_arg: &'a WithLifetime) -> &'a str {","highlight_start":35,"highlight_end":47}],"label":null,"suggested_replacement":"dyn WithLifetime","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/needless_lifetimes.rs:172:35\n   |\nLL | fn trait_obj_elided<'a>(_arg: &'a WithLifetime) -> &'a str {\n   |                                   ^^^^^^^^^^^^ help: use `dyn`: `dyn WithLifetime`\n\n"}
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4087,"byte_end":4091,"line_start":178,"line_end":178,"column_start":36,"column_end":40,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {","highlight_start":36,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4087,"byte_end":4091,"line_start":178,"line_end":178,"column_start":36,"column_end":40,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {","highlight_start":36,"highlight_end":40}],"label":null,"suggested_replacement":"dyn Drop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/needless_lifetimes.rs:178:36\n   |\nLL | fn trait_obj_elided2<'a>(_arg: &'a Drop) -> &'a str {\n   |                                    ^^^^ help: use `dyn`: `dyn Drop`\n\n"}
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5243,"byte_end":5270,"line_start":229,"line_end":229,"column_start":34,"column_end":61,"is_primary":true,"text":[{"text":"    fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {","highlight_start":34,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5243,"byte_end":5270,"line_start":229,"line_end":229,"column_start":34,"column_end":61,"is_primary":true,"text":[{"text":"    fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {","highlight_start":34,"highlight_end":61}],"label":null,"suggested_replacement":"dyn Iterator<Item = usize> + 'a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/needless_lifetimes.rs:229:34\n   |\nLL |     fn iter<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {\n   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item = usize> + 'a`\n\n"}
[01:34:52] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] 
[01:34:52] thread '[ui] ui/needless_lifetimes.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:52] 
[01:34:52] ---- [ui] ui/non_copy_const.rs stdout ----
[01:34:52] normalized stderr:
[01:34:52] error: trait objects without an explicit `dyn` are deprecated
[01:34:52]   --> $DIR/non_copy_const.rs:30:16
[01:34:52]    |
[01:34:52] LL | const NO_ANN: &Display = &70;
[01:34:52]    |                ^^^^^^^ help: use `dyn`: `dyn Display`
[01:34:52]    |
[01:34:52]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52] error: aborting due to previous error
[01:34:52] 
[01:34:52] 
[01:34:52] 
[01:34:52] 
[01:34:52] expected stderr:
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52] LL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable
[01:34:52]    | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52]    | |
[01:34:52]    | |
[01:34:52]    | help: make this a static item: `static`
[01:34:52]    |
[01:34:52]    = note: #[deny(clippy::declare_interior_mutable_const)] on by default
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL | const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable
[01:34:52]    | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52]    | help: make this a static item: `static`
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL | const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);
[01:34:52]    | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52]    | help: make this a static item: `static`
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |         const $name: $ty = $e;
[01:34:52] ...
[01:34:52] ...
[01:34:52] LL | declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable
[01:34:52]    | ------------------------------------------ in this macro invocation
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52] LL |     const ATOMIC: AtomicUsize; //~ ERROR interior mutable
[01:34:52]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52] LL |     const INPUT: T;
[01:34:52]    |     ^^^^^^^^^^^^^^^
[01:34:52]    |
[01:34:52]    |
[01:34:52] help: consider requiring `T` to be `Copy`
[01:34:52]   --> $DIR/non_copy_const.rs:45:18
[01:34:52]    |
[01:34:52] LL |     const INPUT: T;
[01:34:52]    |                  ^
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const ASSOC: Self::NonCopyType;
[01:34:52]    |
[01:34:52]    |
[01:34:52] help: consider requiring `<Self as Trait<T>>::NonCopyType` to be `Copy`
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const ASSOC: Self::NonCopyType;
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const AN_INPUT: T = Self::INPUT;
[01:34:52]    |
[01:34:52] help: consider requiring `T` to be `Copy`
[01:34:52]   --> $DIR/non_copy_const.rs:52:21
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const AN_INPUT: T = Self::INPUT;
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |         const $name: $ty = $e;
[01:34:52] ...
[01:34:52] ...
[01:34:52] LL |     declare_const!(ANOTHER_INPUT: T = Self::INPUT); //~ ERROR interior mutable
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52] LL |     const SELF_2: Self;
[01:34:52]    |     ^^^^^^^^^^^^^^^^^^^
[01:34:52]    |
[01:34:52]    |
[01:34:52] help: consider requiring `Self` to be `Copy`
[01:34:52]   --> $DIR/non_copy_const.rs:61:19
[01:34:52]    |
[01:34:52] LL |     const SELF_2: Self;
[01:34:52]    |                   ^^^^
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const ASSOC_3: AtomicUsize = AtomicUsize::new(14); //~ ERROR interior mutable
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const U_SELF: U = U::SELF_2;
[01:34:52]    |
[01:34:52] help: consider requiring `U` to be `Copy`
[01:34:52]   --> $DIR/non_copy_const.rs:85:19
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const U_SELF: U = U::SELF_2;
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item should never be interior mutable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const T_ASSOC: T::NonCopyType = T::ASSOC;
[01:34:52]    |
[01:34:52]    |
[01:34:52] help: consider requiring `<T as Trait<u32>>::NonCopyType` to be `Copy`
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     const T_ASSOC: T::NonCopyType = T::ASSOC;
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = note: #[deny(clippy::borrow_interior_mutable_const)] on by default
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _once_ref = &ONCE_INIT; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52]    Compiling typenum v1.10.0
[01:34:52]    Compiling typenum v1.10.0
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = &(&&&&ATOMIC_TUPLE).0; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = &ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = ATOMIC_TUPLE.0[0].load(Ordering::SeqCst); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _ = ATOMIC_TUPLE.0[0]; //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52] LL |     CELL.set(2); //~ ERROR interior mutability
[01:34:52]    |     ^^^^
[01:34:52]    |
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     assert_eq!(CELL.get(), 6); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     u64::ATOMIC.store(5, Ordering::SeqCst); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] 
[01:34:52] error: a const item with interior mutability should not be borrowed
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     assert_eq!(u64::ATOMIC.load(Ordering::SeqCst), 9); //~ ERROR interior mutability
[01:34:52]    |
[01:34:52]    = help: assign this const to a local or static variable, and use the variable here
[01:34:52] 
[01:34:52] error: aborting due to 29 previous errors
[01:34:52] error: aborting due to 29 previous errors
[01:34:52] 
[01:34:52] 
[01:34:52] 
[01:34:52] diff of stderr:
[01:34:52] 
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] +error: trait objects without an explicit `dyn` are deprecated
[01:34:52] +  --> $DIR/non_copy_const.rs:30:16
[01:34:52]     |
[01:34:52] -LL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable
[01:34:52] -LL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable
[01:34:52] -   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52] -   | |
[01:34:52] -   | help: make this a static item: `static`
[01:34:52] +LL | const NO_ANN: &Display = &70;
[01:34:52] +   |                ^^^^^^^ help: use `dyn`: `dyn Display`
[01:34:52] -   = note: #[deny(clippy::declare_interior_mutable_const)] on by default
[01:34:52] -   = note: #[deny(clippy::declare_interior_mutable_const)] on by default
[01:34:52] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52]  
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable
[01:34:52] -   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52] -   | help: make this a static item: `static`
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL | const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);
[01:34:52] -   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52] -   | help: make this a static item: `static`
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |         const $name: $ty = $e;
[01:34:52] -...
[01:34:52] -...
[01:34:52] -LL | declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable
[01:34:52] -   | ------------------------------------------ in this macro invocation
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -LL |     const ATOMIC: AtomicUsize; //~ ERROR interior mutable
[01:34:52] -   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -LL |     const INPUT: T;
[01:34:52] -   |     ^^^^^^^^^^^^^^^
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -help: consider requiring `T` to be `Copy`
[01:34:52] -  --> $DIR/non_copy_const.rs:45:18
[01:34:52] -   |
[01:34:52] -LL |     const INPUT: T;
[01:34:52] -   |                  ^
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const ASSOC: Self::NonCopyType;
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -help: consider requiring `<Self as Trait<T>>::NonCopyType` to be `Copy`
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const ASSOC: Self::NonCopyType;
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const AN_INPUT: T = Self::INPUT;
[01:34:52] -   |
[01:34:52] -help: consider requiring `T` to be `Copy`
[01:34:52] -  --> $DIR/non_copy_const.rs:52:21
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const AN_INPUT: T = Self::INPUT;
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |         const $name: $ty = $e;
[01:34:52] -...
[01:34:52] -...
[01:34:52] -LL |     declare_const!(ANOTHER_INPUT: T = Self::INPUT); //~ ERROR interior mutable
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -LL |     const SELF_2: Self;
[01:34:52] -   |     ^^^^^^^^^^^^^^^^^^^
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -help: consider requiring `Self` to be `Copy`
[01:34:52] -  --> $DIR/non_copy_const.rs:61:19
[01:34:52] -   |
[01:34:52] -LL |     const SELF_2: Self;
[01:34:52] -   |                   ^^^^
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const ASSOC_3: AtomicUsize = AtomicUsize::new(14); //~ ERROR interior mutable
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const U_SELF: U = U::SELF_2;
[01:34:52] -   |
[01:34:52] -help: consider requiring `U` to be `Copy`
[01:34:52] -  --> $DIR/non_copy_const.rs:85:19
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const U_SELF: U = U::SELF_2;
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item should never be interior mutable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const T_ASSOC: T::NonCopyType = T::ASSOC;
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -help: consider requiring `<T as Trait<u32>>::NonCopyType` to be `Copy`
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     const T_ASSOC: T::NonCopyType = T::ASSOC;
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     ATOMIC.store(1, Ordering::SeqCst); //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = note: #[deny(clippy::borrow_interior_mutable_const)] on by default
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     assert_eq!(ATOMIC.load(Ordering::SeqCst), 5); //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _once_ref = &ONCE_INIT; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _once_ref_2 = &&ONCE_INIT; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _once_ref_4 = &&&&ONCE_INIT; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _once_mut = &mut ONCE_INIT; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _ = &ATOMIC_TUPLE; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _ = &ATOMIC_TUPLE.0; //~ ERROR interior mutability
[01:34:52] -   |
[01:34:52] -   = help: assign this const to a local or static variable, and use the variable here
[01:34:52] -
[01:34:52] -
[01:34:52] -error: a const item with interior mutability should not be borrowed
---
[01:34:52] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'non_copy_const.rs'
[01:34:52] 
[01:34:52] error: 1 errors occurred comparing output.
[01:34:52] status: exit code: 1
[01:34:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/non_copy_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/non_copy_const.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/non_copy_const.stage-id.aux" "-A" "unused"
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] stderr:
[01:34:52] stderr:
[01:34:52] ------------------------------------------
[01:34:52]    Compiling curl v0.4.21
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_copy_const.rs","byte_start":979,"byte_end":986,"line_start":30,"line_end":30,"column_start":16,"column_end":23,"is_primary":true,"text":[{"text":"const NO_ANN: &Display = &70;","highlight_start":16,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/non_copy_const.rs","byte_start":979,"byte_end":986,"line_start":30,"line_end":30,"column_start":16,"column_end":23,"is_primary":true,"text":[{"text":"const NO_ANN: &Display = &70;","highlight_start":16,"highlight_end":23}],"label":null,"suggested_replacement":"dyn Display","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/non_copy_const.rs:30:16\n   |\nLL | const NO_ANN: &Display = &70;\n   |                ^^^^^^^ help: use `dyn`: `dyn Display`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] thread '[ui] ui/non_copy_const.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:52] thread '[ui] ui/non_copy_const.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:52] 
[01:34:52] ---- [ui] ui/unnecessary_clone.rs stdout ----
[01:34:52] normalized stderr:
[01:34:52] error: trait objects without an explicit `dyn` are deprecated
[01:34:52]   --> $DIR/unnecessary_clone.rs:47:16
[01:34:52]    |
[01:34:52] LL |     let _: Arc<SomeTrait> = x.clone();
[01:34:52]    |                ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
[01:34:52]    |
[01:34:52]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52] error: aborting due to previous error
[01:34:52] 
[01:34:52] 
[01:34:52] 
---
[01:34:52] 
[01:34:52] error: using `clone` on a `Copy` type
[01:34:52]   --> $DIR/unnecessary_clone.rs:21:5
[01:34:52]    |
[01:34:52] LL |     (&42).clone();
[01:34:52]    |     ^^^^^^^^^^^^^ help: try dereferencing it: `*(&42)`
[01:34:52] error: using `clone` on a `Copy` type
[01:34:52]   --> $DIR/unnecessary_clone.rs:24:5
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     rc.borrow().clone();
[01:34:52]    |     ^^^^^^^^^^^^^^^^^^^ help: try dereferencing it: `*rc.borrow()`
[01:34:52] 
[01:34:52] error: using '.clone()' on a ref-counted pointer
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     rc.clone();
[01:34:52]    |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`
[01:34:52]    = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`
[01:34:52] 
[01:34:52] 
[01:34:52] error: using '.clone()' on a ref-counted pointer
[01:34:52]    |
[01:34:52] LL |     arc.clone();
[01:34:52] LL |     arc.clone();
[01:34:52]    |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`
[01:34:52] 
[01:34:52] error: using '.clone()' on a ref-counted pointer
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     rcweak.clone();
[01:34:52]    |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`
[01:34:52] 
[01:34:52] error: using '.clone()' on a ref-counted pointer
[01:34:52]    |
[01:34:52] LL |     arc_weak.clone();
[01:34:52] LL |     arc_weak.clone();
[01:34:52]    |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`
[01:34:52] 
[01:34:52] error: using '.clone()' on a ref-counted pointer
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _: Arc<SomeTrait> = x.clone();
[01:34:52]    |                             ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`
[01:34:52] error: using `clone` on a `Copy` type
[01:34:52]   --> $DIR/unnecessary_clone.rs:51:5
[01:34:52]    |
[01:34:52] LL |     t.clone();
[01:34:52] LL |     t.clone();
[01:34:52]    |     ^^^^^^^^^ help: try removing the `clone` call: `t`
[01:34:52] 
[01:34:52] error: using `clone` on a `Copy` type
[01:34:52]   --> $DIR/unnecessary_clone.rs:53:5
[01:34:52]    |
[01:34:52] LL |     Some(t).clone();
[01:34:52]    |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`
[01:34:52] error: using `clone` on a double-reference; this will copy the reference instead of cloning the inner type
[01:34:52]   --> $DIR/unnecessary_clone.rs:59:22
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let z: &Vec<_> = y.clone();
[01:34:52]    |
[01:34:52]    = note: #[deny(clippy::clone_double_ref)] on by default
[01:34:52] help: try dereferencing it
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let z: &Vec<_> = &(*y).clone();
[01:34:52]    |                      ^^^^^^^^^^^^^
[01:34:52] help: or try being explicit about what type to clone
[01:34:52]    |
[01:34:52] LL |     let z: &Vec<_> = &std::vec::Vec<i32>::clone(y);
[01:34:52] 
[01:34:52] 
[01:34:52] error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let v2: Vec<isize> = v.iter().cloned().collect();
[01:34:52]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `.to_vec()`
[01:34:52]    = note: `-D clippy::iter-cloned-collect` implied by `-D warnings`
[01:34:52] 
[01:34:52] 
[01:34:52] error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |     let _: Vec<isize> = vec![1, 2, 3].iter().cloned().collect();
[01:34:52]    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `.to_vec()`
[01:34:52] 
[01:34:52] error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52]    |
[01:34:52] LL |               .to_bytes()
[01:34:52]    |  ________________________^
[01:34:52] LL | |             .iter()
[01:34:52] LL | |             .iter()
[01:34:52] LL | |             .cloned()
[01:34:52] LL | |             .collect();
[01:34:52]    | |______________________^ help: try: `.to_vec()`
[01:34:52] 
[01:34:52] error: using `clone` on a `Copy` type
[01:34:52]   --> $DIR/unnecessary_clone.rs:114:20
[01:34:52]    |
[01:34:52] LL |         let _: E = a.clone();
[01:34:52]    |                    ^^^^^^^^^ help: try dereferencing it: `*****a`
[01:34:52] error: aborting due to 15 previous errors
[01:34:52] 
[01:34:52] 
[01:34:52] 
---
[01:34:52] -
[01:34:52] -error: using `clone` on a `Copy` type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:21:5
[01:34:52] -   |
[01:34:52] -LL |     (&42).clone();
[01:34:52] -   |     ^^^^^^^^^^^^^ help: try dereferencing it: `*(&42)`
[01:34:52] -error: using `clone` on a `Copy` type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:24:5
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     rc.borrow().clone();
[01:34:52] -   |     ^^^^^^^^^^^^^^^^^^^ help: try dereferencing it: `*rc.borrow()`
[01:34:52] -
[01:34:52] -error: using '.clone()' on a ref-counted pointer
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     rc.clone();
[01:34:52] -   |     ^^^^^^^^^^ help: try this: `Rc::<bool>::clone(&rc)`
[01:34:52] -   = note: `-D clippy::clone-on-ref-ptr` implied by `-D warnings`
[01:34:52] -
[01:34:52] -
[01:34:52] -error: using '.clone()' on a ref-counted pointer
[01:34:52] -   |
[01:34:52] -LL |     arc.clone();
[01:34:52] -LL |     arc.clone();
[01:34:52] -   |     ^^^^^^^^^^^ help: try this: `Arc::<bool>::clone(&arc)`
[01:34:52] -
[01:34:52] -error: using '.clone()' on a ref-counted pointer
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     rcweak.clone();
[01:34:52] -   |     ^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&rcweak)`
[01:34:52] -
[01:34:52] -error: using '.clone()' on a ref-counted pointer
[01:34:52] -   |
[01:34:52] -LL |     arc_weak.clone();
[01:34:52] -LL |     arc_weak.clone();
[01:34:52] -   |     ^^^^^^^^^^^^^^^^ help: try this: `Weak::<bool>::clone(&arc_weak)`
[01:34:52] -
[01:34:52] -error: using '.clone()' on a ref-counted pointer
[01:34:52] -   |
[01:34:52] -   |
[01:34:52]  LL |     let _: Arc<SomeTrait> = x.clone();
[01:34:52] -   |                             ^^^^^^^^^ help: try this: `Arc::<SomeImpl>::clone(&x)`
[01:34:52] -error: using `clone` on a `Copy` type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:51:5
[01:34:52] +   |                ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
[01:34:52]     |
[01:34:52]     |
[01:34:52] -LL |     t.clone();
[01:34:52] -   |     ^^^^^^^^^ help: try removing the `clone` call: `t`
[01:34:52] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52] -error: using `clone` on a `Copy` type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:53:5
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     Some(t).clone();
[01:34:52] -   |     ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `Some(t)`
[01:34:52] -error: using `clone` on a double-reference; this will copy the reference instead of cloning the inner type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:59:22
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let z: &Vec<_> = y.clone();
[01:34:52] -   |
[01:34:52] -   = note: #[deny(clippy::clone_double_ref)] on by default
[01:34:52] -help: try dereferencing it
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let z: &Vec<_> = &(*y).clone();
[01:34:52] -   |                      ^^^^^^^^^^^^^
[01:34:52] -help: or try being explicit about what type to clone
[01:34:52] -   |
[01:34:52] -LL |     let z: &Vec<_> = &std::vec::Vec<i32>::clone(y);
[01:34:52] -
[01:34:52] -
[01:34:52] -error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let v2: Vec<isize> = v.iter().cloned().collect();
[01:34:52] -   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `.to_vec()`
[01:34:52] -   = note: `-D clippy::iter-cloned-collect` implied by `-D warnings`
[01:34:52] -
[01:34:52] -
[01:34:52] -error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52] -   |
[01:34:52] -   |
[01:34:52] -LL |     let _: Vec<isize> = vec![1, 2, 3].iter().cloned().collect();
[01:34:52] -   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `.to_vec()`
[01:34:52] -
[01:34:52] -error: called `iter().cloned().collect()` on a slice to create a `Vec`. Calling `to_vec()` is both faster and more readable
[01:34:52] -   |
[01:34:52] -LL |               .to_bytes()
[01:34:52] -   |  ________________________^
[01:34:52] -LL | |             .iter()
[01:34:52] -LL | |             .iter()
[01:34:52] -LL | |             .cloned()
[01:34:52] -LL | |             .collect();
[01:34:52] -   | |______________________^ help: try: `.to_vec()`
[01:34:52] -
[01:34:52] -error: using `clone` on a `Copy` type
[01:34:52] -  --> $DIR/unnecessary_clone.rs:114:20
[01:34:52] -   |
[01:34:52] -LL |         let _: E = a.clone();
[01:34:52] -   |                    ^^^^^^^^^ help: try dereferencing it: `*****a`
[01:34:52] -error: aborting due to 15 previous errors
[01:34:52] +error: aborting due to previous error
[01:34:52]  
[01:34:52]  
---
[01:34:52] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'unnecessary_clone.rs'
[01:34:52] 
[01:34:52] error: 1 errors occurred comparing output.
[01:34:52] status: exit code: 1
[01:34:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unnecessary_clone.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unnecessary_clone.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unnecessary_clone.stage-id.aux" "-A" "unused"
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] stderr:
[01:34:52] stderr:
[01:34:52] ------------------------------------------
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":881,"byte_end":890,"line_start":47,"line_end":47,"column_start":16,"column_end":25,"is_primary":true,"text":[{"text":"    let _: Arc<SomeTrait> = x.clone();","highlight_start":16,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_clone.rs","byte_start":881,"byte_end":890,"line_start":47,"line_end":47,"column_start":16,"column_end":25,"is_primary":true,"text":[{"text":"    let _: Arc<SomeTrait> = x.clone();","highlight_start":16,"highlight_end":25}],"label":null,"suggested_replacement":"dyn SomeTrait","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/unnecessary_clone.rs:47:16\n   |\nLL |     let _: Arc<SomeTrait> = x.clone();\n   |                ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] thread '[ui] ui/unnecessary_clone.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:52] thread '[ui] ui/unnecessary_clone.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:34:52] 
[01:34:52] ---- [ui] ui/unused_unit.rs stdout ----
[01:34:52] normalized stderr:
[01:34:52] error: trait objects without an explicit `dyn` are deprecated
[01:34:52]   --> $DIR/unused_unit.rs:21:18
[01:34:52]    |
[01:34:52] LL |         let _y: &Fn() -> () = &f;
[01:34:52]    |                  ^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> ()`
[01:34:52]    |
[01:34:52]    = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:18:59
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL |       pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->
[01:34:52] LL | |         ()
[01:34:52] LL | |         ()
[01:34:52]    | |__________^ help: remove the `-> ()`
[01:34:52] note: lint level defined here
[01:34:52]   --> $DIR/unused_unit.rs:12:9
[01:34:52]    |
[01:34:52] LL | #![deny(clippy::unused_unit)]
[01:34:52] LL | #![deny(clippy::unused_unit)]
[01:34:52]    |         ^^^^^^^^^^^^^^^^^^^
[01:34:52] 
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:28:19
[01:34:52]    |
[01:34:52] LL |     fn into(self) -> () {
[01:34:52]    |                   ^^^^^ help: remove the `-> ()`
[01:34:52] error: unneeded unit expression
[01:34:52]   --> $DIR/unused_unit.rs:29:9
[01:34:52]    |
[01:34:52] LL |         ()
[01:34:52] LL |         ()
[01:34:52]    |         ^^ help: remove the final `()`
[01:34:52] 
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:33:18
[01:34:52]    |
[01:34:52] LL | fn return_unit() -> () { () }
[01:34:52]    |                  ^^^^^ help: remove the `-> ()`
[01:34:52] error: unneeded unit expression
[01:34:52]   --> $DIR/unused_unit.rs:33:26
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL | fn return_unit() -> () { () }
[01:34:52]    |                          ^^ help: remove the final `()`
[01:34:52] error: unneeded `()`
[01:34:52]   --> $DIR/unused_unit.rs:42:14
[01:34:52]    |
[01:34:52] LL |         break();
---
[01:34:52] expected stderr:
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:18:59
[01:34:52]    |
[01:34:52] LL |       pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->
[01:34:52] LL | |         ()
[01:34:52] LL | |         ()
[01:34:52]    | |__________^ help: remove the `-> ()`
[01:34:52] note: lint level defined here
[01:34:52]   --> $DIR/unused_unit.rs:12:9
[01:34:52]    |
[01:34:52] LL | #![deny(clippy::unused_unit)]
[01:34:52] LL | #![deny(clippy::unused_unit)]
[01:34:52]    |         ^^^^^^^^^^^^^^^^^^^
[01:34:52] 
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:28:19
[01:34:52]    |
[01:34:52] LL |     fn into(self) -> () {
[01:34:52]    |                   ^^^^^ help: remove the `-> ()`
[01:34:52] error: unneeded unit expression
[01:34:52]   --> $DIR/unused_unit.rs:29:9
[01:34:52]    |
[01:34:52] LL |         ()
[01:34:52] LL |         ()
[01:34:52]    |         ^^ help: remove the final `()`
[01:34:52] 
[01:34:52] error: unneeded unit return type
[01:34:52]   --> $DIR/unused_unit.rs:33:18
[01:34:52]    |
[01:34:52] LL | fn return_unit() -> () { () }
[01:34:52]    |                  ^^^^^ help: remove the `-> ()`
[01:34:52] error: unneeded unit expression
[01:34:52]   --> $DIR/unused_unit.rs:33:26
[01:34:52]    |
[01:34:52]    |
[01:34:52] LL | fn return_unit() -> () { () }
[01:34:52]    |                          ^^ help: remove the final `()`
[01:34:52] error: unneeded `()`
[01:34:52]   --> $DIR/unused_unit.rs:42:14
[01:34:52]    |
[01:34:52] LL |         break();
---
[01:34:52] 
[01:34:52] +error: trait objects without an explicit `dyn` are deprecated
[01:34:52] +  --> $DIR/unused_unit.rs:21:18
[01:34:52] +   |
[01:34:52] +LL |         let _y: &Fn() -> () = &f;
[01:34:52] +   |                  ^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> ()`
[01:34:52] +   |
[01:34:52] +   = note: `-D bare-trait-objects` implied by `-D warnings`
[01:34:52]  error: unneeded unit return type
[01:34:52]    --> $DIR/unused_unit.rs:18:59
[01:34:52]     |
[01:34:52]     |
[01:34:52]  LL |       pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->
[01:34:52]  LL | |         ()
[01:34:52]  LL | |         ()
[01:34:52]     | |__________^ help: remove the `-> ()`
[01:34:52]  note: lint level defined here
[01:34:52]    --> $DIR/unused_unit.rs:12:9
[01:34:52]     |
[01:34:52]  LL | #![deny(clippy::unused_unit)]
[01:34:52]  LL | #![deny(clippy::unused_unit)]
[01:34:52]     |         ^^^^^^^^^^^^^^^^^^^
[01:34:52]  
[01:34:52]  error: unneeded unit return type
[01:34:52]    --> $DIR/unused_unit.rs:28:19
[01:34:52]     |
[01:34:52]  LL |     fn into(self) -> () {
[01:34:52]     |                   ^^^^^ help: remove the `-> ()`
[01:34:52]  error: unneeded unit expression
[01:34:52]    --> $DIR/unused_unit.rs:29:9
[01:34:52]     |
[01:34:52]  LL |         ()
[01:34:52]  LL |         ()
[01:34:52]     |         ^^ help: remove the final `()`
[01:34:52]  
[01:34:52]  error: unneeded unit return type
[01:34:52]    --> $DIR/unused_unit.rs:33:18
[01:34:52]     |
[01:34:52]  LL | fn return_unit() -> () { () }
[01:34:52]     |                  ^^^^^ help: remove the `-> ()`
[01:34:52]  error: unneeded unit expression
[01:34:52]    --> $DIR/unused_unit.rs:33:26
[01:34:52]     |
[01:34:52]     |
[01:34:52]  LL | fn return_unit() -> () { () }
[01:34:52]     |                          ^^ help: remove the final `()`
[01:34:52]  error: unneeded `()`
[01:34:52]    --> $DIR/unused_unit.rs:42:14
[01:34:52]     |
[01:34:52]  LL |         break();
---
[01:34:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unused_unit.stderr
[01:34:52] normalized fixed:
[01:34:52] // run-rustfix
[01:34:52] 
[01:34:52] // The output for humans should just highlight the whole span without showing
[01:34:52] // the suggested replacement, but we also want to test that suggested
[01:34:52] // replacement only removes one set of parentheses, rather than naïvely
[01:34:52] // stripping away any starting or ending parenthesis characters—hence this
[01:34:52] // test of the JSON error format.
[01:34:52] #![feature(custom_inner_attributes)]
[01:34:52] #![feature(custom_inner_attributes)]
[01:34:52] #![rustfmt::skip]
[01:34:52] 
[01:34:52] #![deny(clippy::unused_unit)]
[01:34:52] struct Unitter;
[01:34:52] impl Unitter {
[01:34:52] impl Unitter {
[01:34:52]     // try to disorient the lint with multiple unit returns and newlines
[01:34:52]     #[allow(clippy::no_effect)]
[01:34:52]     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) 
[01:34:52]     where G: Fn() -> () {
[01:34:52]         let _y: &dyn Fn() -> () = &f;
[01:34:52]         (); // this should not lint, as it's not in return type position
[01:34:52] }
[01:34:52] 
[01:34:52] 
[01:34:52] impl Into<()> for Unitter {
[01:34:52]     fn into(self)  {
[01:34:52]         
[01:34:52]     }
[01:34:52] }
[01:34:52] }
[01:34:52] 
[01:34:52] fn return_unit()  {  }
[01:34:52] 
[01:34:52] #[allow(clippy::needless_return)]
[01:34:52] #[allow(clippy::never_loop)]
[01:34:52] fn main() {
[01:34:52]     let u = Unitter;
[01:34:52]     assert_eq!(u.get_unit(|| {}, return_unit), u.into());
[01:34:52]     return_unit();
[01:34:52]     loop {
[01:34:52]     }
[01:34:52]     return;
[01:34:52] }
[01:34:52] 
[01:34:52] 
[01:34:52] 
[01:34:52] expected fixed:
[01:34:52] // run-rustfix
[01:34:52] 
[01:34:52] // The output for humans should just highlight the whole span without showing
[01:34:52] // the suggested replacement, but we also want to test that suggested
[01:34:52] // replacement only removes one set of parentheses, rather than naïvely
[01:34:52] // stripping away any starting or ending parenthesis characters—hence this
[01:34:52] // test of the JSON error format.
[01:34:52] #![feature(custom_inner_attributes)]
[01:34:52] #![feature(custom_inner_attributes)]
[01:34:52] #![rustfmt::skip]
[01:34:52] 
[01:34:52] #![deny(clippy::unused_unit)]
[01:34:52] struct Unitter;
[01:34:52] impl Unitter {
[01:34:52] impl Unitter {
[01:34:52]     // try to disorient the lint with multiple unit returns and newlines
[01:34:52]     #[allow(clippy::no_effect)]
[01:34:52]     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) 
[01:34:52]     where G: Fn() -> () {
[01:34:52]         let _y: &Fn() -> () = &f;
[01:34:52]         (); // this should not lint, as it's not in return type position
[01:34:52] }
[01:34:52] 
[01:34:52] 
[01:34:52] impl Into<()> for Unitter {
[01:34:52]     fn into(self)  {
[01:34:52]         
[01:34:52]     }
[01:34:52] }
[01:34:52] }
[01:34:52] 
[01:34:52] fn return_unit()  {  }
[01:34:52] 
[01:34:52] #[allow(clippy::needless_return)]
[01:34:52] #[allow(clippy::never_loop)]
[01:34:52] fn main() {
[01:34:52]     let u = Unitter;
[01:34:52]     assert_eq!(u.get_unit(|| {}, return_unit), u.into());
[01:34:52]     return_unit();
[01:34:52]     loop {
[01:34:52]     }
[01:34:52]     return;
[01:34:52] }
[01:34:52] 
[01:34:52] 
[01:34:52] 
[01:34:52] diff of fixed:
[01:34:52] 
[01:34:52]  // run-rustfix
[01:34:52]  
[01:34:52]  // The output for humans should just highlight the whole span without showing
[01:34:52]  // the suggested replacement, but we also want to test that suggested
[01:34:52]  // replacement only removes one set of parentheses, rather than naïvely
[01:34:52]  // stripping away any starting or ending parenthesis characters—hence this
[01:34:52]  // test of the JSON error format.
[01:34:52]  #![feature(custom_inner_attributes)]
[01:34:52]  #![feature(custom_inner_attributes)]
[01:34:52]  #![rustfmt::skip]
[01:34:52]  
[01:34:52]  #![deny(clippy::unused_unit)]
[01:34:52]  struct Unitter;
[01:34:52]  impl Unitter {
[01:34:52]  impl Unitter {
[01:34:52]      // try to disorient the lint with multiple unit returns and newlines
[01:34:52]      #[allow(clippy::no_effect)]
[01:34:52]      pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) 
[01:34:52]      where G: Fn() -> () {
[01:34:52] -        let _y: &Fn() -> () = &f;
[01:34:52] +        let _y: &dyn Fn() -> () = &f;
[01:34:52]          (); // this should not lint, as it's not in return type position
[01:34:52]  }
[01:34:52]  
[01:34:52]  
[01:34:52]  impl Into<()> for Unitter {
[01:34:52]      fn into(self)  {
[01:34:52]          
[01:34:52]      }
[01:34:52]  }
[01:34:52]  }
[01:34:52]  
[01:34:52]  fn return_unit()  {  }
[01:34:52]  
[01:34:52]  #[allow(clippy::needless_return)]
[01:34:52]  #[allow(clippy::never_loop)]
[01:34:52]  fn main() {
[01:34:52]      let u = Unitter;
[01:34:52]      assert_eq!(u.get_unit(|| {}, return_unit), u.into());
[01:34:52]      return_unit();
[01:34:52]      loop {
[01:34:52]      }
[01:34:52]      return;
[01:34:52]  }
[01:34:52]  
[01:34:52]  
[01:34:52] 
[01:34:52] The actual fixed differed from the expected fixed.
[01:34:52] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unused_unit.fixed
[01:34:52] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base' 'unused_unit.rs'
[01:34:52] 
[01:34:52] error: 2 errors occurred comparing output.
[01:34:52] status: exit code: 1
[01:34:52] status: exit code: 1
[01:34:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unused_unit.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unused_unit.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-6cb8e92dd9cb3b3c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-a4f0af23d7f3dcf8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-c35ac21f4e6017c4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-48b14578da8f1083/out/test_build_base/unused_unit.stage-id.aux" "-A" "unused"
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] stderr:
[01:34:52] stderr:
[01:34:52] ------------------------------------------
[01:34:52] {"message":"trait objects without an explicit `dyn` are deprecated","code":{"code":"bare_trait_objects","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":687,"byte_end":697,"line_start":21,"line_end":21,"column_start":18,"column_end":28,"is_primary":true,"text":[{"text":"        let _y: &Fn() -> () = &f;","highlight_start":18,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D bare-trait-objects` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `dyn`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":687,"byte_end":697,"line_start":21,"line_end":21,"column_start":18,"column_end":28,"is_primary":true,"text":[{"text":"        let _y: &Fn() -> () = &f;","highlight_start":18,"highlight_end":28}],"label":null,"suggested_replacement":"dyn Fn() -> ()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: trait objects without an explicit `dyn` are deprecated\n  --> tests/ui/unused_unit.rs:21:18\n   |\nLL |         let _y: &Fn() -> () = &f;\n   |                  ^^^^^^^^^^ help: use `dyn`: `dyn Fn() -> ()`\n   |\n   = note: `-D bare-trait-objects` implied by `-D warnings`\n\n"}
[01:34:52] {"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":630,"byte_end":643,"line_start":18,"line_end":19,"column_start":59,"column_end":11,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->","highlight_start":59,"highlight_end":61},{"text":"        ()","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":413,"byte_end":432,"line_start":12,"line_end":12,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![deny(clippy::unused_unit)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":630,"byte_end":643,"line_start":18,"line_end":19,"column_start":59,"column_end":11,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->","highlight_start":59,"highlight_end":61},{"text":"        ()","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:18:59\n   |\nLL |       pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) ->\n   |  ___________________________________________________________^\nLL | |         ()\n   | |__________^ help: remove the `-> ()`\n   |\nnote: lint level defined here\n  --> tests/ui/unused_unit.rs:12:9\n   |\nLL | #![deny(clippy::unused_unit)]\n   |         ^^^^^^^^^^^^^^^^^^^\n\n"}
[01:34:52] {"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":853,"byte_end":858,"line_start":28,"line_end":28,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":853,"byte_end":858,"line_start":28,"line_end":28,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:28:19\n   |\nLL |     fn into(self) -> () {\n   |                   ^^^^^ help: remove the `-> ()`\n\n"}
[01:34:52] {"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":869,"byte_end":871,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":869,"byte_end":871,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:29:9\n   |\nLL |         ()\n   |         ^^ help: remove the final `()`\n\n"}
[01:34:52] {"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":898,"byte_end":903,"line_start":33,"line_end":33,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":18,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":898,"byte_end":903,"line_start":33,"line_end":33,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":18,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:33:18\n   |\nLL | fn return_unit() -> () { () }\n   |                  ^^^^^ help: remove the `-> ()`\n\n"}
[01:34:52] {"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":906,"byte_end":908,"line_start":33,"line_end":33,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":906,"byte_end":908,"line_start":33,"line_end":33,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:33:26\n   |\nLL | fn return_unit() -> () { () }\n   |                          ^^ help: remove the final `()`\n\n"}
[01:34:52] {"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1109,"byte_end":1111,"line_start":42,"line_end":42,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1109,"byte_end":1111,"line_start":42,"line_end":42,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:42:14\n   |\nLL |         break();\n   |              ^^ help: remove the `()`\n\n"}
[01:34:52] {"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1129,"byte_end":1131,"line_start":44,"line_end":44,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1129,"byte_end":1131,"line_start":44,"line_end":44,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:44:11\n   |\nLL |     return();\n   |           ^^ help: remove the `()`\n\n"}
[01:34:52] 
[01:34:52] ------------------------------------------
[01:34:52] 
[01:34:52] thread '[ui] ui/unused_unit.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
[01:50:12]    Compiling rls v1.36.0 (/checkout/src/tools/rls)
[01:50:13] warning: trait objects without an explicit `dyn` are deprecated
[01:50:13]   --> src/tools/rls/tests/support/client/mod.rs:43:37
[01:50:13]    |
[01:50:13] 43 | type Channels = Rc<RefCell<Vec<(Box<Fn(&Value) -> bool>, oneshot::Sender<Value>)>>>;
[01:50:13]    |                                     ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Fn(&Value) -> bool`
[01:50:13]    = note: #[warn(bare_trait_objects)] on by default
[01:50:13] 
[01:50:55] [RUSTC-TIMING] tooltip test:true 42.854
[01:52:05] [RUSTC-TIMING] client test:true 113.453
---
[01:56:50] Verifying status of rustfmt...
[01:56:50] Verifying status of clippy-driver...
[01:56:50] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:56:50] 
[01:56:50] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:56:50] 
[01:56:50] If you do intend to update 'clippy-driver', please check the error messages above and
[01:56:50] commit another update.
[01:56:50] 
[01:56:50] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:56:50] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:56:50] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:12fd3db2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 08:32:07 UTC 2019
---
travis_time:end:065baa70:start=1559205129310967696,finish=1559205129318458354,duration=7490658
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05a709c5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b4ac214
travis_time:start:0b4ac214
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01a77090
$ dmesg | grep -i kill
