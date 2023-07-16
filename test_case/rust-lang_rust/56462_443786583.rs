plain
travis_time:end:0df5dd84:start=1543853811483088899,finish=1543853883734079907,duration=72250991008
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:58]    Compiling rustc-rayon v0.1.1
[00:05:06]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:18]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:19]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:19]    Compiling rustc_query_macro v0.1.0 (/checkout/src/librustc_query_macro)
[00:06:40]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:40]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:41] group [Other] type_of
[00:06:41] QUERY Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , } ,
[00:06:41] DEP NODE [  ] TypeOf ( DefId ) ,
[00:06:41] DEP NODE FORCE DepKind :: TypeOf => {
[00:06:41] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; }
[00:06:41] RETURNING macro_rules ! rustc_query_append {
[00:06:41] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:06:41] $ ( $ macro ) * {
[00:06:41] $ ( $ other ) * Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , }
[00:06:41] , } } } macro_rules ! rustc_dep_node_append {
[00:06:41] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:06:41] $ ( $ macro ) * ( $ ( $ other ) * [  ] TypeOf ( DefId ) , ) ; } } macro_rules
[00:06:41] ! rustc_dep_node_force {
[00:06:41] ( [ $ dep_node : expr , $ tcx : expr ] $ ( $ other : tt ) * ) => {
[00:06:41] match $ dep_node . kind {
[00:06:41] $ ( $ other ) * DepKind :: TypeOf => {
[00:06:41] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; } } } }
[00:12:02]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:25]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:14:55]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:14:56]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
---
[00:20:57]    Compiling rustc-rayon v0.1.1
[00:21:05]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:21:17]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:21:18]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:21:18]    Compiling rustc_query_macro v0.1.0 (/checkout/src/librustc_query_macro)
[00:22:49]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:22:49]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:22:50] group [Other] type_of
[00:22:50] QUERY Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , } ,
[00:22:50] DEP NODE [  ] TypeOf ( DefId ) ,
[00:22:50] DEP NODE FORCE DepKind :: TypeOf => {
[00:22:50] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; }
[00:22:50] RETURNING macro_rules ! rustc_query_append {
[00:22:50] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:22:50] $ ( $ macro ) * {
[00:22:50] $ ( $ other ) * Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , }
[00:22:50] , } } } macro_rules ! rustc_dep_node_append {
[00:22:50] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:22:50] $ ( $ macro ) * ( $ ( $ other ) * [  ] TypeOf ( DefId ) , ) ; } } macro_rules
[00:22:50] ! rustc_dep_node_force {
[00:22:50] ( [ $ dep_node : expr , $ tcx : expr ] $ ( $ other : tt ) * ) => {
[00:22:50] match $ dep_node . kind {
[00:22:50] $ ( $ other ) * DepKind :: TypeOf => {
[00:22:50] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; } } } }
[00:28:22]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:31:14]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:31:14]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:32:31]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
---
[00:43:47] .................................................................................................... 400/5107
[00:43:50] .................................................................................................... 500/5107
[00:43:53] ..............................i..................................................................... 600/5107
[00:43:56] .................................................................................................... 700/5107
[00:44:02] ..................................................................................................i. 800/5107
[00:44:06] ..............i......F...F.......................................................................... 900/5107
[00:44:09] .....................i.iiii......................................................................... 1000/5107
[00:44:13] .................................................................................................... 1200/5107
[00:44:15] .................................................................................................... 1300/5107
[00:44:18] .................................................................................................... 1400/5107
[00:44:20] .................................................................................................... 1500/5107
---
[00:46:12] 55 error: OK
[00:46:12] 56   --> $DIR/dep-graph-struct-signature.rs:70:9
[00:46:12] 
[00:46:12] 57    |
[00:46:12] - LL |         #[rustc_then_this_would_need(TypeOfItem)] //~ ERROR OK
[00:46:12] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:12] + LL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:46:12] 60 
[00:46:12] 61 error: OK
[00:46:12] 62   --> $DIR/dep-graph-struct-signature.rs:72:9
[00:46:12] 
[00:46:12] 
[00:46:12] 63    |
[00:46:12] - LL |         #[rustc_then_this_would_need(TypeOfItem)] //~ ERROR OK
[00:46:12] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:12] + LL |         #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:46:12] 66 
[00:46:12] 66 
[00:46:12] - error: no path from `WillChange` to `TypeOfItem`
[00:46:12] + error: no path from `WillChange` to `TypeOf`
[00:46:12] 69    |
[00:46:12] 69    |
[00:46:12] - LL |     #[rustc_then_this_would_need(TypeOfItem)] //~ ERROR no path
[00:46:12] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:12] + LL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:46:12] 72 
[00:46:12] 72 
[00:46:12] - error: no path from `WillChange` to `TypeOfItem`
[00:46:12] + error: no path from `WillChange` to `TypeOf`
[00:46:12] 75    |
[00:46:12] 75    |
[00:46:12] - LL |     #[rustc_----------------------------------
[00:46:12] ------------------------------------------
[00:46:12] ------------------------------------------
[00:46:12] {"message":"no path from `WillChange` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":878,"byte_end":915,"line_start":37,"line_end":37,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:37:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `AssociatedItems`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":938,"byte_end":984,"line_start":38,"line_end":38,"column_start":5,"column_end":51,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(AssociatedItems)] //~ ERROR no path","highlight_start":5,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `AssociatedItems`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:38:5\n   |\nLL |     #[rustc_then_this_would_need(AssociatedItems)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `TraitDefOfItem`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1007,"byte_end":1052,"line_start":39,"line_end":39,"column_start":5,"column_end":50,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TraitDefOfItem)] //~ ERROR no path","highlight_start":5,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TraitDefOfItem`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:39:5\n   |\nLL |     #[rustc_then_this_would_need(TraitDefOfItem)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1202,"byte_end":1244,"line_start":45,"line_end":45,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:45:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/eckTables)] //~ ERROR OK","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:50:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1567,"byte_end":1604,"line_start":55,"line_end":55,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:55:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1838,"byte_end":1875,"line_start":62,"line_end":62,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2330,"byte_end":2367,"line_start":77,"line_end":77,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:77:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2475,"byte_end":2512,"line_start":84,"line_end":84,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:84:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `FnSignature`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2676,"byte_end":2718,"line_start":90,"line_end":90,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `FnSignature`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:90:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `FnSignature`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2770,"byte_end":2812,"line_start":93,"line_end":93,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path from `WillChange`","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `FnSignature`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:93:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path from `WillChange`\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `TypeckTables`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2853,"byte_end":2896,"line_start":94,"line_end":94,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path from `WillChange`","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `TypeckTables`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:94:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path from `WillChange`\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1095,"byte_end":1137,"line_start":41,"line_end":41,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:41:9\n   |\nLL |         #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `WillChange` to `FnSignature`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":2553,"byte_end":2595,"line_start":86,"line_end":86,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `WillChange` to `FnSignature`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:86:9\n   |\nLL |         #[rustc_then_this_would_need(FnSignature)] //~ ERROR no path\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1648,"byte_end":1690,"line_start":57,"line_end":57,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs:57:9\n   |\nLL |         #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs","byte_start":1712,"byte_end":1755,"line_start":58,"line_end":58,"column_start":9,"column_end":52,"is_primary":true,"text":[{"text":"        #[rustc_then_this_would_need(TypeckTables)] //~ ERROR OK","highlight_start":9,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggustc_then_this_would_need(TypeOf)] //~ ERROR no path
[00:46:12] 36 
[00:46:12] 37 error: OK
[00:46:12] 38   --> $DIR/dep-graph-type-alias.rs:59:1
[00:46:12] 
[00:46:12] 
[00:46:12] 39    |
[00:46:12] - LL | #[rustc_then_this_would_need(TypeOfItem)] //~ ERROR OK
[00:46:12] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:46:12] + LL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK
[00:46:12] 42 
[00:46:12] 43 error: OK
[00:46:12] 44   --> $DIR/dep-graph-type-alias.rs:62:1
[00:46:12] 
[00:46:12] 
[00:46:12] 
[00:46:12] The actual stderr differed from the expected stderr.
[00:46:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/dep-graph-type-alias.stderr
[00:46:12] To update references, rerun the tests and pass the `--bless` flag
[00:46:12] To only update this specific test, also pass `--test-args dep-graph/dep-graph-type-alias.rs`
[00:46:12] error: 1 errors occurred comparing output.
[00:46:12] status: exit code: 1
[00:46:12] status: exit code: 1
[00:46:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[00:46:12] ------------------------------------------
[00:46:12] 
[00:46:12] ------------------------------------------
[00:46:12] stderr:
[00:46:12] stderr:
[00:46:12] ------------------------------------------
[00:46:12] {"message":"no path from `TypeAlias` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":802,"byte_end":839,"line_start":28,"line_end":28,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:28:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":878,"byte_end":915,"line_start":30,"line_end":30,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:30:5\n   |\nLL |     #[rustc_then_this_would_need(TypeOf)] //~ ERROR OK\n   |  s.rs","byte_start":1152,"byte_end":1189,"line_start":44,"line_end":44,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:44:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"no path from `TypeAlias` to `TypeOf`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1339,"byte_end":1376,"line_start":52,"line_end":52,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR no path","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `TypeOf`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:52:1\n   |\nLL | #[rustc_then_this_would_need(TypeOf)] //~ ERROR no path\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1573,"byte_end":1610,"line_start":59,"line_end":59,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(TypeOf)] //~ ERROR OK"es)] //~ ERROR OK\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1226,"byte_end":1268,"line_start":46,"line_end":46,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:46:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1415,"byte_end":1457,"line_start":54,"line_end":54,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs:54:5\n   |\nLL |     #[rustc_then_this_would_need(FnSignature)] //~ ERROR OK\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:12] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs","byte_start":1475,"byte_end":1518,"line_start": "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:12] 
[00:46:12] 
[00:46:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:12] Build completed unsuccessfully in 0:03:39
[00:46:12] Build completed unsuccessfully in 0:03:39
[00:46:12] make: *** [check] Error 1
[00:46:12] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27002e77
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 17:04:24 UTC 2018
---
travis_time:end:0ba09a09:start=1543856665929268237,finish=1543856665933144290,duration=3876053
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:226aeccb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|
