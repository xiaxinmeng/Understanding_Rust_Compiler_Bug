\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":1656,"byte_end":1682,"line_start":58,"line_end":58,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider adding an explicit lifetime bound `<T as Anything<'_#6r, '_#7r>>::AssocType: ReEarlyBound(0, 'a)`...","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0309]: the associated type `<T as Anything<'_#6r, '_#7r>>::AssocType` may not live long enough\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:58:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider adding an explicit lifetime bound `<T as Anything<'_#6r, '_#7r>>::AssocType: ReEarlyBound(0, 'a)`...\n\n"}
[00:48:48] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2083,"byte_end":2109,"line_start":71,"line_end":71,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:32 ~ projection_two_region_trait_bound_closure[317d]::projection_outlives[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    '_#2r,\n    '_#3r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#4r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#3r)>>::AssocType: '_#4r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:71:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:32 ~ projection_two_region_trait_bound_closure[317d]::projection_outlives[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               '_#2r,\n               '_#3r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#4r ()>, T))\n           ]\n   = note: number of external vids: 5\n   = note: where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#3r)>>::AssocType: '_#4r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":1804,"byte_end":2113,"line_start":63,"line_end":72,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn projection_outlives<'a, 'b, 'c, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":64},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b, 'c>,","highlight_start":1,"highlight_end":25},{"text":"    T::AssocType: 'a,","highlight_start":1,"highlight_end":22},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    // We are projecting `<T as Anything<'b>>::AssocType`, and we know","highlight_start":1,"highlight_end":71},{"text":"    // that this outlives `'a` because of the where-clause.","highlight_start":1,"highlight_end":60},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:10 ~ projection_two_region_trait_bound_closure[317d]::projection_outlives[0]) with substs [\n    '_#1r,\n    '_#2r,\n    '_#3r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:63:1\n   |\nLL | / fn projection_outlives<'a, 'b, 'c, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b, 'c>,\nLL | |     T::AssocType: 'a,\n...  |\nLL | |     with_signature(cell, t, |cell, t| require(cell, t));\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:10 ~ projection_two_region_trait_bound_closure[317d]::projection_outlives[0]) with substs [\n               '_#1r,\n               '_#2r,\n               '_#3r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2267,"byte_end":2293,"line_start":80,"line_end":80,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:37 ~ projection_two_region_trait_bound_closure[317d]::elements_outlive1[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    '_#2r,\n    '_#3r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#4r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#3r)>>::AssocType: '_#4r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:80:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:37 ~ projection_two_region_trait_bound_closure[317d]::elements_outlive1[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               '_#2r,\n               '_#3r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#4r ()>, T))\n           ]\n   = note: number of external vids: 5\n   = note: where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#3r)>>::AssocType: '_#4r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2132,"byte_end":2297,"line_start":75,"line_enote: number of external vids: 5\n   = note: where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#3r)>>::AssocType: '_#4r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2316,"byte_end":2481,"line_start":84,"line_end":90,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn elements_outlive2<'a, 'b, 'c, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":62},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b, 'c>,","highlight_start":1,"highlight_end":25},{"text":"    'c: 'a,","highlight_start":1,"highlight_end":12},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:12 ~ projection_two_region_trait_bound_closure[317d]::elements_outlive2[0]) with substs [\n    '_#1r,\n    '_#2r,\n    '_#3r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:84:1\n   |\nLL | / fn elements_outlive2<'a, 'b, 'c, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b, 'c>,\nLL | |     'c: 'a,\nLL | | {\nLL | |     with_signature(cell, t, |cell, t| require(celll, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:46 ~ projection_two_region_trait_bound_closure[317d]::two_regions[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n           ]\n   = note: late-bound region is '_#3r\n   = note: number of external vids: 4\n   = note: where <T as Anything<ReClosureBound('_#1r), ReClosureBound('_#1r)>>::AssocType: '_#2r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2500,"byte_end":2691,"line_start":93,"line_end":99,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn two_regions<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":52},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b, 'b>,","highlight_start":1,"highlight_end":25},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"    //~^ ERROR unsatisfied lifetime constraints","highlight_start":1,"highlight_end":48},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:13 ~ projection_two_region_trait_bound_closure[317d]::two_regions[0]) with substs [\n    '_#1r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:93:1\n   |\nLL | / fn two_regions<'a, 'b, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b, 'b>,\nLL | | {\nLL | |     with_signature(cell, t, |cell, t| require(cell, t));\nLL | |     //~^ ERROR unsatisfied lifetime constraints\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:13 ~ projection_two_region_trait_bound_closure[317d]::two_regions[0]) with substs [\n               '_#1r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2519,"byte_end":2521,"line_start":93,"line_end":93,"column_start":20,"column_end":22,"is_primary":false,"text":[{"text":"fn two_regions<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":20,"highlight_end":22}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2515,"byte_end":2517,"line_start":93,"line_end":93,"column_start":16,"column_end":18,"is_primary":false,"text":[{"text":"fn two_regions<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":16,"highlight_end":18}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":nullegions_outlive[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    '_#2r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#3r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#2r)>>::AssocType: '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:107:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:50 ~ projection_two_region_trait_bound_closure[317d]::two_regions_outlive[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               '_#2r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#3r ()>, T))\n           ]\n   = note: number of external vids: 4\n   = note: where <T as Anything<ReClosureBound('_#2r), ReClosureBound('_#2r)>>::AssocType: '_#3r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2710,"byte_end":2873,"line_start":102,"line_end":108,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn two_regions_outlive<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":60},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b, 'b>,","highlight_start":1,"highlight_end":25},{"text":"    'b: 'a,","highlight_start":1,"highlight_end":12},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:14 ~ projection_two_region_trait_bound_closure[317d]::two_regions_outlive[0]) with substs [\n    '_#1r,\n    '_#2r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:102:1\n   |\nLL | / fn two_regions_outlive<'a, 'b, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b, 'b>,\nLL | |     'b: 'a,\nLL | | {\nLL | |     with_signature(cell, t, |cell, t| require(cell, t));\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:14 ~ projection_two_region_trait_bound_closure[317d]::two_regions_outlive[0]) with substs [\n               '_#1r,\n               '_#2r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":3218,"byte_end":3244,"line_start":119,"line_end":119,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:53 ~ projection_two_region_trait_bound_closure[317d]::one_region[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 3","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as Anything<ReClosureBound('_#1r), ReClosureBound('_#1r)>>::AssocType: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:119:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:53 ~ projection_two_region_trait_bound_closure[317d]::one_region[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n           ]\n   = note: number of external vids: 3\n   = note: where <T as Anything<ReClosureBound('_#1r), ReClosureBound('_#1r)>>::AssocType: '_#2r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":2892,"byte_end":3248,"line_start":111,"line_end":120,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn one_region<'a, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":47},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'a, 'a>,","highlight_start":1,"highlight_end":25},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    // Note that in this case the closure still propagates an external","highlight_start":1,"highlight_end":71},{"text":"    // requirement between two variables in its signature, but the","highlight_start":1,"highlight_end":67},{"text":"    // creator maps both those two region variables to `'a` on its","highlight_start":1,"highlight_end":67},{"text":"    // side.","highlight_start":1,"highlight_end":13},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:15 ~ projection_two_region_trait_bound_closure[317d]::one_region[0]) with substs [\n    '_#1r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:111:1\n   |\nLL | / fn one_region<'a, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'a, 'a>,\nLL | | {\n...  |\nLL | |     with_signature(cell, t, |cell, t| require(cell, t));\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:15 ~ projection_two_region_trait_bound_closure[317d]::one_region[0]) with substs [\n               '_#1r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:48] {"message":"For more information about this error, try `rustc --explain E0309`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0309`.\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/type-alias-free-regions.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/type-alias-free-regions.rs:19:9
[00:48:48] 3    |
[00:48:48] 4 LL | impl<'a> FromBox<'a> for c<'a> {
[00:48:48] 
[00:48:48] 8 LL |         c { f: b } //~ ERROR
[00:48:48] 9    |         ^^^^^^^^^^ returning this value requires that `'1` must outlive `'a`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 12   --> $DIR/type-alias-free-regions.rs:29:9
[00:48:48] 13    |
[00:48:48] 13    |
[00:48:48] 14 LL | impl<'a> FromTuple<'a> fort_start":17,"highlight_end":18}],"label":"has type `std::boxed::Box<std::boxed::Box<&'1 isize>>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/type-alias-free-regions.rs","byte_start":305,"byte_end":307,"line_start":17,"line_end":17,"column_start":6,"column_end":8,"is_primary":false,"text":[{"text":"impl<'a> FromBox<'a> for c<'a> {","highlight_start":6,"highlight_end":8}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/type-alias-free-regions.rs","byte_start":378,"byte_end":388,"line_start":19,"line_end":19,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        c { f: b } //~ ERROR","highlight_start":9,"highlight_end":19}],"label":"returning this value requires that `'1` must outlive `'a`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/type-alias-free-regions.rs:19:9\n   |\nLL | impl<'a> FromBox<'a> for c<'a> {\n   |      -- lifetime `'a` defined here\nLL |     fn from_box(b: Box<b>) -> Self {\n   |                 - has type `std::boxed::Box<std::boxed::Box<&'1 isize>>`\nLL |         c { f: b } //~ ERROR\n   |         ^^^^^^^^^^ returning this value requires that `'1` must outlive `'a`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/type-alias-free-regions.rs","byte_start":523,"byte_end"due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/type-alias-free-regions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/closure-substs.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/closure-substs.rs:18:16
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a>() {
[00:48:48] 
[00:48:48] 7 LL |         return x; //~ ERROR unsatisfied lifetime constraints
[00:48:48] 8    |                ^ returning this value requires that `'a` must outlive `'static`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 11   --> $DIR/closure-substs.rs:25:16
[00:48:48] 12    |
[00:48:48] 12    |
[00:48:48] 13 LL |     |x: &i32| -> &'static i32 {
[00:48:48] 
[00:48:48] 15 LL |         return x; //~ ERROR unsatisfied lifetime constraints
[00:48:48] 16    |                ^ returning this value requires that `'1` must outlive `'static`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 19   --> $DIR/closure-substs.rs:32:9
[00:48:48] 20    |
[00:48:48] 20    |
[00:48:48] 21 LL | fn bar<'a>() {
[00:48:48] 
[00:48:48] 
[00:48:48] The actual stderr differed from the ","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/closure-substs.rs","byte_start":663,"byte_end":664,"line_start":18,"line_end":18,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        return x; //~ ERROR unsatisfied lifetime constraints","highlight_start":16,"highlight_end":17}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:18:16\n   |\nLL | fn foo<'a>() {\n   |        -- lifetime `'a` defined here\n...\nLL |         return x; //~ ERROR unsatisfied lifetime constraints\n   |                ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/closure-substs.rs","byte_start":784,"byte_end":785,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"    |x: &i32| -> &'static i32 {","highlight_start":9,"highlight_end":10}],"label":"let's call the lifetime of this reference `'1`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/closure-substs.rs","byte_start":823,"byte_end":824,"line_start":25,"line_end":25,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        return x; //~ ERROR unsatisfied lifetime constraints","highlight_start":16,"highlight_end":17}],"label":"returning this value requires that `'1` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:25:16\n   |\nLL |     |x: &i32| -> &'static i32 {\n   |         - let's call the lifetime of this reference `'1`\nLL |         return x; //~ ERROR unsatisfied lifetime constraints\n   |                ^ returning this value requires that `'1` must outlive `'static`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/closure-substs.rs","byte_start":886,"byte_end":888,"line_start":29,"line_end":29,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn bar<'a>() {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/closure-substs.rs","byte_start":986,"byte_end":990,"line_start":32,"line_end":32,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        b(x); //~ ERROR unsatisfied lifetime constraints","highlight_start":9,"highlight_end":13}],"label":"argument requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-inherent-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-1/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-inherent-1.rs","byte_start":106,"byte_end":108,"line_start":9,"line_end":9,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a>(_: &'a u32) -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-inherent-1.rs","byte_start":144,"byte_end":156,"line_start":10,"line_end":10,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    <Foo<'a>>::C //~ ERROR","highlight_start":5,"highlight_end":17}],"labend pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-trait-item-1.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-1/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs","byte_start":125,"byte_end":127,"line_start":11,"line_end":11,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a>(_: &'a u32) -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs","byte_start":163,"byte_end":181,"line_start":12,"line_end":12,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    <() as Foo<'a>>::C //~ ERROR","highlight_start":5,"highlight_end":23}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-1.rs:12:5\n   |\nLL | fn foo<'a>(_: &'a u32) -> &'static u32 {\n   |        -- lifetime `'a` defined here\nLL |     <() as Foo<'a>>::C //~ ERROR\n   |     ^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/user-annotations/constant-in-expr-trait-item-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/constant-in-expr-normalize.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/constant-in-expr-normalize.rs:20:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a>(_: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] 
[00:48:48] Try":false,"text":[{"text":"fn foo<'a>(_: &'a u32) -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-normalize.rs","byte_start":253,"byte_end":271,"line_start":20,"line_end":20,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    <() as Foo<'a>>::C //~ ERROR","highlight_start":5,"highlight_end":23}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-normalize.rs:20:5\n   |\nLL | fn foo<'a>(_: &'a u32) -> &'static u32 {\n   |        -- lifetime `'a` defined here\nLL |     <() as Foo<'a>>::C //~ ERROR\n   |     ^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/user-annotations/constant-in-expr-normalize.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/constant-in-expr-trait-item-2.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/constant-in-expr-trait-item-2.rs:12:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a, T: Foo<'a>>() -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2/constant-in-expr-trait-item-2.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-trait-item-2.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-2/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs","byte_start":125,"byte_end":127,"line_start":11,"line_end":11,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a, T: Foo<'a>>() -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs","byte_start":165,"byte_end":182,"line_start":12,"line_end":12,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    <T as Foo<'a>>::C //~ ERROR","highlight_start":5,"highlight_end":22}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-2.rs:12:5\n   |\nLL | fn foo<'a, T: Foo<'a>>() -> &'static u32 {\n   |        -- lifetime `'a` defined here\nLL |     <T as Foo<'a>>::C //~ ERROR\n   |     ^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/user-annotations/constant-in-expr-trait-item-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/constant-in-expr-trait-item-3.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/constant-in-expr-trait-item-3.rs:12:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a, T: Foo<'a>>() -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-3/constant-in-expr-trait-item-3.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-trait-item-3.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-trait-item-3/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-3.rs","byte_start":125,"byte_end":127,"line_start":11,"line_end":11,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a, T: Foo<'a>>() -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-3.rs","byte_start":165,"byte_end":169,"line_start":12,"line_end":12,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    T::C //~ ERROR","highlight_start":5,"highlight_end":9}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/constant-in-expr-trait-item-3.rs:12:5\n   |\nLL | fn foo<'a, T: Foo<'a>>() -> &'static u32 {\n   |        -- lifetime `'a` defined here\nLL |     T::C //~ ERROR\n   |     ^^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error",ustc" "/checkout/src/test/ui/nll/user-annotations/issue-54124.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-54124/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-54124.rs","byte_start":57,"byte_end":58,"line_start":4,"line_end":4,"column_start":24,"column_end":25,"is_primary":false,"text":[{"text":"    let _:fn(&()) = |_:&'a ()| {};","highlight_start":24,"highlight_end":25}],"label":"let's call the lifetime of this reference `'1`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-54124.rs","byte_start":26,"byte_end":28,"line_start":3,"line_end":3,"column_start":9,"column_end":11,"is_primary":false,"text":[{"text":"fn test<'a>() {","highlight_start":9,"highlight_end":11}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annot:null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/issue-54124.rs:4:22\n   |\nLL | fn test<'a>() {\n   |         -- lifetime `'a` defined here\nLL |     let _:fn(&()) = |_:&'a ()| {};\n   |                      ^ requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/user-annotations/issue-54124.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/wf-self-type.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/wf-self-type.rs:22:5
[00:48:48] 3    |
[00:48:48] 4 LL | pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type/wf-self-type.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type/wf-self-type.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/user-annotations/wf-self-type.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48]gestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/wf-self-type.rs","byte_start":663,"byte_end":676,"line_start":22,"line_end":22,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    Foo::xmute(u) //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":18}],"label":"returning this value requires that `'b` must outlive `'a`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/wf-self-type.rs:22:5\n   |\nLL | pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {\n   |            --  -- lifetime `'b` defined here\n   |            |\n   |            lifetime `'a` defined here\nLL |     Foo::xmute(u) //~ ERROR unsatisfied lifetime constraints\n   |     ^^^^^^^^^^^^^ returning this value requires that `'b` must outlive `'a`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/user-annotations/wf-self-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/user-annotations/patterns.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 148 LL | }
[00:48:48] 149    | - `x` dropped here while still borrowed
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 152   --> $DIR/patterns.rs:113:5
[00:48:48] 153    |
[00:48:48] 153    |
[00:48:48] 154 LL | fn static_to_a_to_static_through_variable<'a>(x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] 157 LL |     y //~ ERROR
[00:48:48] 158    |     ^ returning this value requires that `'a` must outlive `'static`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 161   --> $DIR/patterns.rs:125:5
[00:48:48] 162    |
[00:48:48] 162    |
[00:48:48] 163 LL | fn static_to_a_to_static_through_tuple<'a>(x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] 166 LL |     y //~ ERROR
[00:48:48] 167    |     ^ returning this value requires that `'a` must outlive `'static`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 170   --> $DIR/patterns.rs:130:5
[00:48:48] 171    |
[00:48:48] 171    |
[00:48:48] 172 LL | fn static_to_a_to_static_through_struct<'a>(_x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] 175 LL |     y //~ ERROR
[00:48:48] 176    |     ^ returning this value requires that `'a` must outlive `'static`
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 179   --> $DIR/patterns.rs:134:18
[00:48:48] 180    |
[00:48:48] 180    |
[00:48:48] 181 LL | fn a_to_static_then_static<'a>(x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64c u32;\n   |            ------------ type annotation requires that `x` is borrowed for `'static`\nLL |     y = &x; //~ ERROR\n   |         ^^ borrowed value does not live long enough\nLL | }\n   | - `x` dropped here while still borrowed\n\n"}
[00:48:48] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n