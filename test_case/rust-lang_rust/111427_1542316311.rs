plain
##[endgroup]
Check compiletest suite=rustdoc-json mode=rustdoc-json (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 106 tests
.FFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFF.FF.FFF.F.FFFFFF.FFFF.F.F.FFFFFFF.FF.FFFFFFFFFFFF  88/106
F.FFFFFF..FFFF.FFF
failures:

---- [rustdoc-json] tests/rustdoc-json/enums/field_hidden.rs stdout ----


error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/field_hidden" "--template" "/checkout/tests/rustdoc-json/enums/field_hidden.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='UnexpectedEndTag')].inner.kind.tuple matched to [], but expected Array [Null] on line 8
Failed check: $.index[*][?(@.name=='UnexpectedEndTag')].inner.discriminant matched to [], but expected Null on line 9
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/field_hidden.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/field_hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/field_hidden" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/field_hidden.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/tuple_fields_hidden.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/tuple_fields_hidden" "--template" "/checkout/tests/rustdoc-json/enums/tuple_fields_hidden.rs"
stdout: none
--- stderr -------------------------------
Failed check: `$.index[*][?(@.name=='One')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 1 on line 20
Failed check: $.index[*][?(@.name=='One')].inner.kind.tuple[0] matched to [], but expected String("0:6:1596") on line 21
Failed check: `$.index[*][?(@.name=='OneHidden')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 1 on line 23
Failed check: $.index[*][?(@.name=='OneHidden')].inner.kind.tuple[0] matched to [], but expected Null on line 24
Failed check: `$.index[*][?(@.name=='Two')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 2 on line 27
Failed check: $.index[*][?(@.name=='Two')].inner.kind.tuple[0] matched to [], but expected String("0:12:1596") on line 28
Failed check: $.index[*][?(@.name=='Two')].inner.kind.tuple[1] matched to [], but expected String("0:13:1597") on line 29
Failed check: `$.index[*][?(@.name=='TwoLeftHidden')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 2 on line 31
Failed check: $.index[*][?(@.name=='TwoLeftHidden')].inner.kind.tuple[0] matched to [], but expected Null on line 32
Failed check: $.index[*][?(@.name=='TwoLeftHidden')].inner.kind.tuple[1] matched to [], but expected String("0:17:1597") on line 33
Failed check: `$.index[*][?(@.name=='TwoRightHidden')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 2 on line 35
Failed check: $.index[*][?(@.name=='TwoRightHidden')].inner.kind.tuple[0] matched to [], but expected String("0:20:1596") on line 36
Failed check: $.index[*][?(@.name=='TwoRightHidden')].inner.kind.tuple[1] matched to [], but expected Null on line 37
Failed check: `$.index[*][?(@.name=='TwoBothHidden')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 2 on line 39
Failed check: $.index[*][?(@.name=='TwoBothHidden')].inner.kind.tuple[0] matched to [], but expected Null on line 40
Failed check: $.index[*][?(@.name=='TwoBothHidden')].inner.kind.tuple[1] matched to [], but expected Null on line 41
Failed check: `$.index[*][?(@.name=='Three1')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 3 on line 44
Failed check: $.index[*][?(@.name=='Three1')].inner.kind.tuple[0] matched to [], but expected Null on line 45
Failed check: $.index[*][?(@.name=='Three1')].inner.kind.tuple[1] matched to [], but expected String("0:29:1597") on line 46
Failed check: $.index[*][?(@.name=='Three1')].inner.kind.tuple[2] matched to [], but expected String("0:30:1598") on line 47
Failed check: `$.index[*][?(@.name=='Three2')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 3 on line 49
Failed check: $.index[*][?(@.name=='Three2')].inner.kind.tuple[0] matched to [], but expected String("0:33:1596") on line 50
Failed check: $.index[*][?(@.name=='Three2')].inner.kind.tuple[1] matched to [], but expected Null on line 51
Failed check: $.index[*][?(@.name=='Three2')].inner.kind.tuple[2] matched to [], but expected String("0:35:1598") on line 52
Failed check: `$.index[*][?(@.name=='Three3')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 3 on line 54
Failed check: $.index[*][?(@.name=='Three3')].inner.kind.tuple[0] matched to [], but expected String("0:38:1596") on line 55
Failed check: $.index[*][?(@.name=='Three3')].inner.kind.tuple[1] matched to [], but expected String("0:39:1597") on line 56
Failed check: $.index[*][?(@.name=='Three3')].inner.kind.tuple[2] matched to [], but expected Null on line 57
Failed check: $.index[*][?(@.docs=='1.1.0')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 73
Failed check: $.index[*][?(@.docs=='2.1.0')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 74
Failed check: $.index[*][?(@.docs=='2.1.1')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 75
Failed check: $.index[*][?(@.docs=='2.2.1')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 76
Failed check: $.index[*][?(@.docs=='2.3.0')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 77
Failed check: $.index[*][?(@.docs=='3.1.1')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 78
Failed check: $.index[*][?(@.docs=='3.1.2')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 79
Failed check: $.index[*][?(@.docs=='3.2.0')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 80
Failed check: $.index[*][?(@.docs=='3.2.2')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 81
Failed check: $.index[*][?(@.docs=='3.3.0')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 82
Failed check: $.index[*][?(@.docs=='3.3.1')].inner matched to [Object {"struct_field": Object {"primitive": String("bool")}}], but expected Object {"kind": String("primitive"), "inner": String("bool")} on line 83
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/tuple_fields_hidden.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/tuple_fields_hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/tuple_fields_hidden" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/tuple_fields_hidden.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/kind.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/kind" "--template" "/checkout/tests/rustdoc-json/enums/kind.rs"
stdout: none
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `0`,
  left: `0`,
 right: `1`: Expected 1 match for `$.index[*][?(@.name=='x' && @.kind=='struct_field')].id` (because of @set): matched to []', src/tools/jsondocck/src/main.rs:314:13
------------------------------------------

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/kind/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/kind" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/kind.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/use_glob.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_glob" "--template" "/checkout/tests/rustdoc-json/enums/use_glob.rs"
stdout: none
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `0`,
  left: `0`,
 right: `1`: Expected 1 match for `$.index[*][?(@.kind == 'import')].id` (because of @set): matched to []', src/tools/jsondocck/src/main.rs:314:13
------------------------------------------

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_glob/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_glob" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/use_glob.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/use_variant.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_variant" "--template" "/checkout/tests/rustdoc-json/enums/use_variant.rs"
stdout: none
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `0`,
  left: `0`,
 right: `1`: Expected 1 match for `$.index[*][?(@.kind == 'import')].id` (because of @set): matched to []', src/tools/jsondocck/src/main.rs:314:13
------------------------------------------

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_variant/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/use_variant" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/use_variant.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/blanket_impls.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/blanket_impls" "--template" "/checkout/tests/rustdoc-json/blanket_impls.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Error')].kind matched to [] but didn't have String("assoc_type") on line 5
Failed check: $.index[*][?(@.name=='Error')].inner.default.kind matched to [] but didn't have String("resolved_path") on line 6
Failed check: $.index[*][?(@.name=='Error')].inner.default.inner.name matched to [] but didn't have String("Infallible") on line 7
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/blanket_impls.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/blanket_impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/blanket_impls" "--deny" "warnings" "/checkout/tests/rustdoc-json/blanket_impls.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/assoc_items.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/assoc_items" "--template" "/checkout/tests/rustdoc-json/assoc_items.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='CONSTANT')].kind matched to [], but expected String("assoc_const") on line 6
Failed check: $.index[*][?(@.docs=='ToDeclare trait')].kind matched to [], but expected String("assoc_type") on line 11
Failed check: $.index[*][?(@.docs=='ToDeclare trait')].inner.default matched to [], but expected Null on line 12
Failed check: $.index[*][?(@.docs=='ToDeclare trait')].inner.bounds matched to [], but expected Array [] on line 13
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE trait')].kind matched to [], but expected String("assoc_const") on line 16
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE trait')].inner.default matched to [], but expected Null on line 17
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE trait')].inner.type.kind matched to [], but expected String("primitive") on line 18
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE trait')].inner.type.inner matched to [], but expected String("usize") on line 19
Failed check: $.index[*][?(@.docs=='ToDeclare impl')].kind matched to [], but expected String("assoc_type") on line 25
Failed check: $.index[*][?(@.docs=='ToDeclare impl')].inner.default.kind matched to [], but expected String("primitive") on line 26
Failed check: $.index[*][?(@.docs=='ToDeclare impl')].inner.default.inner matched to [], but expected String("usize") on line 27
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE impl')].kind matched to [], but expected String("assoc_const") on line 31
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE impl')].inner.type.kind matched to [], but expected String("primitive") on line 32
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE impl')].inner.type.inner matched to [], but expected String("usize") on line 33
Failed check: $.index[*][?(@.docs=='AN_ATTRIBUTE impl')].inner.default matched to [], but expected String("12") on line 34
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/assoc_items.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/assoc_items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/assoc_items" "--deny" "warnings" "/checkout/tests/rustdoc-json/assoc_items.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/fn_pointer/generics.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/generics" "--template" "/checkout/tests/rustdoc-json/fn_pointer/generics.rs"
stdout: none
--- stderr -------------------------------
Failed check: `$.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.decl.inputs[*]` matched to `[]` with length 0, but expected length 1 on line 6
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.decl.inputs[0][0] matched to [], but expected String("val") on line 7
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.decl.inputs[0][1].kind matched to [], but expected String("borrowed_ref") on line 8
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.decl.inputs[0][1].inner.lifetime matched to [], but expected String("'c") on line 9
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.decl.output matched to [], but expected Object {"kind": String("primitive"), "inner": String("i32")} on line 10
Failed check: `$.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.generic_params[*]` matched to `[]` with length 0, but expected length 1 on line 11
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.generic_params[0].name matched to [], but expected String("'c") on line 12
Failed check: $.index[*][?(@.name=='WithHigherRankTraitBounds')].inner.type.inner.generic_params[0].kind matched to [], but expected Object {"lifetime": Object {"outlives": Array []}} on line 13
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/fn_pointer/generics.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/generics/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/generics" "--deny" "warnings" "/checkout/tests/rustdoc-json/fn_pointer/generics.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/only_some_have_discriminant.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/only_some_have_discriminant" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/only_some_have_discriminant.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Has')].inner.discriminant matched to [], but expected Object {"expr": String("0"), "value": String("0")} on line 2
Failed check: $.index[*][?(@.name=='Doesnt')].inner.discriminant matched to [], but expected Null on line 4
Failed check: $.index[*][?(@.name=='AlsoDoesnt')].inner.discriminant matched to [], but expected Null on line 6
Failed check: $.index[*][?(@.name=='AlsoHas')].inner.discriminant matched to [], but expected Object {"expr": String("44"), "value": String("44")} on line 8
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/only_some_have_discriminant.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/only_some_have_discriminant/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/only_some_have_discriminant" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/only_some_have_discriminant.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/tuple.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/tuple" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/tuple.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Tuple')].inner.discriminant matched to [], but expected Null on line 6
Failed check: $.index[*][?(@.name=='TupleWithDiscr')].inner.discriminant matched to [], but expected Object {"expr": String("1"), "value": String("1")} on line 9
Failed check: `$.index[*][?(@.name=='TupleWithDiscr')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 1 on line 10
Failed check: $.index[*][?(@.name=='TupleWithBinDiscr')].inner.discriminant matched to [], but expected Object {"expr": String("0b10"), "value": String("2")} on line 12
Failed check: `$.index[*][?(@.name=='TupleWithBinDiscr')].inner.kind.tuple[*]` matched to `[]` with length 0, but expected length 2 on line 13
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/tuple.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/tuple/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/tuple" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/tuple.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/struct.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/struct" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/struct.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Struct')].inner.discriminant matched to [], but expected Null on line 6
Failed check: $.index[*][?(@.name=='StructWithDiscr')].inner.discriminant matched to [], but expected Object {"expr": String("42"), "value": String("42")} on line 9
Failed check: `$.index[*][?(@.name=='StructWithDiscr')].inner.kind.struct.fields[*]` matched to `[]` with length 0, but expected length 1 on line 10
Failed check: $.index[*][?(@.name=='StructWithHexDiscr')].inner.discriminant matched to [], but expected Object {"expr": String("0x42"), "value": String("66")} on line 12
Failed check: `$.index[*][?(@.name=='StructWithHexDiscr')].inner.kind.struct.fields[*]` matched to `[]` with length 0, but expected length 2 on line 13
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/struct.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/struct/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/struct" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/struct.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/num_underscore_and_suffix.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/num_underscore_and_suffix" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/num_underscore_and_suffix.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Basic')].inner.discriminant.value matched to [], but expected String("0") on line 3
Failed check: $.index[*][?(@.name=='Basic')].inner.discriminant.expr matched to [], but expected String("0") on line 4
Failed check: $.index[*][?(@.name=='Suffix')].inner.discriminant.value matched to [], but expected String("10") on line 6
Failed check: $.index[*][?(@.name=='Suffix')].inner.discriminant.expr matched to [], but expected String("10u32") on line 7
Failed check: $.index[*][?(@.name=='Underscore')].inner.discriminant.value matched to [], but expected String("100") on line 9
Failed check: $.index[*][?(@.name=='Underscore')].inner.discriminant.expr matched to [], but expected String("1_0_0") on line 10
Failed check: $.index[*][?(@.name=='SuffixUnderscore')].inner.discriminant.value matched to [], but expected String("1000") on line 12
Failed check: $.index[*][?(@.name=='SuffixUnderscore')].inner.discriminant.expr matched to [], but expected String("1_0_0_0u32") on line 13
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/num_underscore_and_suffix.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/num_underscore_and_suffix/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/num_underscore_and_suffix" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/num_underscore_and_suffix.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/struct_field_hidden.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/struct_field_hidden" "--template" "/checkout/tests/rustdoc-json/enums/struct_field_hidden.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Variant')].inner.kind.struct.fields_stripped matched to [], but expected Bool(true) on line 12
Failed check: $.index[*][?(@.name=='Variant')].inner.kind.struct.fields[0] matched to [], but expected String("0:6:1619") on line 13
Failed check: $.index[*][?(@.name=='Variant')].inner.kind.struct.fields[1] matched to [], but expected String("0:8:1621") on line 14
Failed check: `$.index[*][?(@.name=='Variant')].inner.kind.struct.fields[*]` matched to `[]` with length 0, but expected length 2 on line 15
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/struct_field_hidden.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/struct_field_hidden/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/struct_field_hidden" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/struct_field_hidden.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/basic.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/basic" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/basic.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Less')].inner.discriminant.expr matched to [], but expected String("-1") on line 3
Failed check: $.index[*][?(@.name=='Less')].inner.discriminant.value matched to [], but expected String("-1") on line 4
Failed check: $.index[*][?(@.name=='Equal')].inner.discriminant.expr matched to [], but expected String("0") on line 6
Failed check: $.index[*][?(@.name=='Equal')].inner.discriminant.value matched to [], but expected String("0") on line 7
Failed check: $.index[*][?(@.name=='Greater')].inner.discriminant.expr matched to [], but expected String("1") on line 9
Failed check: $.index[*][?(@.name=='Greater')].inner.discriminant.value matched to [], but expected String("1") on line 10
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/basic.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/basic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/basic" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/basic.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/fns/generic_returns.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_returns" "--template" "/checkout/tests/rustdoc-json/fns/generic_returns.rs"
stdout: none
--- stderr -------------------------------
Failed check: `$.index[*][?(@.name=='generic_returns')].inner.items[*]` matched to `[]` with length 0, but expected length 2 on line 6
Failed check: $.index[*][?(@.name=='get_foo')].inner.decl.inputs matched to [], but expected Array [] on line 11
Failed check: $.index[*][?(@.name=='get_foo')].inner.decl.output.kind matched to [], but expected String("impl_trait") on line 12
Failed check: `$.index[*][?(@.name=='get_foo')].inner.decl.output.inner[*]` matched to `[]` with length 0, but expected length 1 on line 13
Failed check: $.index[*][?(@.name=='get_foo')].inner.decl.output.inner[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 14
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/fns/generic_returns.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_returns/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_returns" "--deny" "warnings" "/checkout/tests/rustdoc-json/fns/generic_returns.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/fns/generic_args.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args" "--template" "/checkout/tests/rustdoc-json/fns/generic_args.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='generics')].inner.generics.where_predicates matched to [], but expected Array [] on line 12
Failed check: `$.index[*][?(@.name=='generics')].inner.generics.params[*]` matched to `[]` with length 0, but expected length 1 on line 13
Failed check: $.index[*][?(@.name=='generics')].inner.generics.params[0].name matched to [], but expected String("F") on line 14
Failed check: $.index[*][?(@.name=='generics')].inner.generics.params[0].kind.type.default matched to [], but expected Null on line 15
Failed check: `$.index[*][?(@.name=='generics')].inner.generics.params[0].kind.type.bounds[*]` matched to `[]` with length 0, but expected length 1 on line 16
Failed check: $.index[*][?(@.name=='generics')].inner.generics.params[0].kind.type.bounds[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 17
Failed check: `$.index[*][?(@.name=='generics')].inner.decl.inputs[*]` matched to `[]` with length 0, but expected length 1 on line 18
Failed check: $.index[*][?(@.name=='generics')].inner.decl.inputs[0][0] matched to [], but expected String("f") on line 19
Failed check: $.index[*][?(@.name=='generics')].inner.decl.inputs[0][1].kind matched to [], but expected String("generic") on line 20
Failed check: $.index[*][?(@.name=='generics')].inner.decl.inputs[0][1].inner matched to [], but expected String("F") on line 21
Failed check: $.index[*][?(@.name=='impl_trait')].inner.generics.where_predicates matched to [], but expected Array [] on line 24
Failed check: `$.index[*][?(@.name=='impl_trait')].inner.generics.params[*]` matched to `[]` with length 0, but expected length 1 on line 25
Failed check: $.index[*][?(@.name=='impl_trait')].inner.generics.params[0].name matched to [], but expected String("impl Foo") on line 26
Failed check: $.index[*][?(@.name=='impl_trait')].inner.generics.params[0].kind.type.bounds[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 27
Failed check: `$.index[*][?(@.name=='impl_trait')].inner.decl.inputs[*]` matched to `[]` with length 0, but expected length 1 on line 28
Failed check: $.index[*][?(@.name=='impl_trait')].inner.decl.inputs[0][0] matched to [], but expected String("f") on line 29
Failed check: $.index[*][?(@.name=='impl_trait')].inner.decl.inputs[0][1].kind matched to [], but expected String("impl_trait") on line 30
Failed check: `$.index[*][?(@.name=='impl_trait')].inner.decl.inputs[0][1].inner[*]` matched to `[]` with length 0, but expected length 1 on line 31
Failed check: $.index[*][?(@.name=='impl_trait')].inner.decl.inputs[0][1].inner[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 32
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.params[*]` matched to `[]` with length 0, but expected length 3 on line 35
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.params[0].name matched to [], but expected String("F") on line 36
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.params[0].kind matched to [], but expected Object {"type": Object {"bounds": Array [], "default": Null, "synthetic": Bool(false)}} on line 37
Failed check: `$.index[*][?(@.name=='where_clase')].inner.decl.inputs[*]` matched to `[]` with length 0, but expected length 3 on line 38
Failed check: $.index[*][?(@.name=='where_clase')].inner.decl.inputs[0][0] matched to [], but expected String("f") on line 39
Failed check: $.index[*][?(@.name=='where_clase')].inner.decl.inputs[0][1].kind matched to [], but expected String("generic") on line 40
Failed check: $.index[*][?(@.name=='where_clase')].inner.decl.inputs[0][1].inner matched to [], but expected String("F") on line 41
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[*]` matched to `[]` with length 0, but expected length 3 on line 42
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[0].bound_predicate.type matched to [], but expected Object {"inner": String("F"), "kind": String("generic")} on line 44
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[0].bound_predicate.bounds[*]` matched to `[]` with length 0, but expected length 1 on line 45
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[0].bound_predicate.bounds[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 46
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.type matched to [], but expected Object {"inner": String("G"), "kind": String("generic")} on line 48
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.bounds[*]` matched to `[]` with length 0, but expected length 1 on line 49
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.bounds[0].trait_bound.trait.id matched to [], but expected String("0:2:1617") on line 50
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.bounds[0].trait_bound.generic_params[*]` matched to `[]` with length 0, but expected length 1 on line 51
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.bounds[0].trait_bound.generic_params[0].name matched to [], but expected String("'a") on line 52
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.bounds[0].trait_bound.generic_params[0].kind matched to [], but expected Object {"lifetime": Object {"outlives": Array []}} on line 53
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[1].bound_predicate.generic_params matched to [], but expected Array [] on line 54
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.type.kind matched to [], but expected String("borrowed_ref") on line 56
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.type.inner.lifetime matched to [], but expected String("'b") on line 57
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.type.inner.type matched to [], but expected Object {"inner": String("H"), "kind": String("generic")} on line 58
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.bounds[*]` matched to `[]` with length 0, but expected length 1 on line 59
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.bounds[0].trait_bound.trait.id matched to [], but expected String("0:1:1616") on line 60
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.bounds[0].trait_bound.generic_params matched to [], but expected Array [] on line 61
Failed check: `$.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.generic_params[*]` matched to `[]` with length 0, but expected length 1 on line 62
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.generic_params[0].name matched to [], but expected String("'b") on line 63
Failed check: $.index[*][?(@.name=='where_clase')].inner.generics.where_predicates[2].bound_predicate.generic_params[0].kind matched to [], but expected Object {"lifetime": Object {"outlives": Array []}} on line 64
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/fns/generic_args.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args" "--deny" "warnings" "/checkout/tests/rustdoc-json/fns/generic_args.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/enums/discriminant/expr.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/expr" "--template" "/checkout/tests/rustdoc-json/enums/discriminant/expr.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name=='Addition')].inner.discriminant.value matched to [], but expected String("0") on line 2
Failed check: $.index[*][?(@.name=='Addition')].inner.discriminant.expr matched to [], but expected String("{ _ }") on line 3
Failed check: $.index[*][?(@.name=='Bin')].inner.discriminant.value matched to [], but expected String("1") on line 5
Failed check: $.index[*][?(@.name=='Bin')].inner.discriminant.expr matched to [], but expected String("0b1") on line 6
Failed check: $.index[*][?(@.name=='Oct')].inner.discriminant.value matched to [], but expected String("2") on line 8
Failed check: $.index[*][?(@.name=='Oct')].inner.discriminant.expr matched to [], but expected String("0o2") on line 9
Failed check: $.index[*][?(@.name=='PubConst')].inner.discriminant.value matched to [], but expected String("3") on line 11
Failed check: $.index[*][?(@.name=='PubConst')].inner.discriminant.expr matched to [], but expected String("THREE") on line 12
Failed check: $.index[*][?(@.name=='Hex')].inner.discriminant.value matched to [], but expected String("4") on line 14
Failed check: $.index[*][?(@.name=='Hex')].inner.discriminant.expr matched to [], but expected String("0x4") on line 15
Failed check: $.index[*][?(@.name=='Cast')].inner.discriminant.value matched to [], but expected String("5") on line 17
Failed check: $.index[*][?(@.name=='Cast')].inner.discriminant.expr matched to [], but expected String("{ _ }") on line 18
Failed check: $.index[*][?(@.name=='PubCall')].inner.discriminant.value matched to [], but expected String("6") on line 20
Failed check: $.index[*][?(@.name=='PubCall')].inner.discriminant.expr matched to [], but expected String("{ _ }") on line 21
Failed check: $.index[*][?(@.name=='PrivCall')].inner.discriminant.value matched to [], but expected String("7") on line 23
Failed check: $.index[*][?(@.name=='PrivCall')].inner.discriminant.expr matched to [], but expected String("{ _ }") on line 24
Failed check: $.index[*][?(@.name=='PrivConst')].inner.discriminant.value matched to [], but expected String("8") on line 26
Failed check: $.index[*][?(@.name=='PrivConst')].inner.discriminant.expr matched to [], but expected String("EIGHT") on line 27
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/enums/discriminant/expr.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/expr/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/enums/discriminant/expr" "--deny" "warnings" "/checkout/tests/rustdoc-json/enums/discriminant/expr.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/fns/extern_c_variadic.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/extern_c_variadic" "--template" "/checkout/tests/rustdoc-json/fns/extern_c_variadic.rs"
stdout: none
--- stderr -------------------------------
Failed check: $.index[*][?(@.name == 'not_variadic')].inner.decl.c_variadic matched to [], but expected Bool(false) on line 5
Failed check: $.index[*][?(@.name == 'variadic')].inner.decl.c_variadic matched to [], but expected Bool(true) on line 7
Error: "Jsondocck failed for /checkout/tests/rustdoc-json/fns/extern_c_variadic.rs"

Rustdoc Output:
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/extern_c_variadic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/extern_c_variadic" "--deny" "warnings" "/checkout/tests/rustdoc-json/fns/extern_c_variadic.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
stderr: none

---- [rustdoc-json] tests/rustdoc-json/fns/generics.rs stdout ----

error: jsondocck failed!
error: jsondocck failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--doc-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generics" "--template" "/checkout/tests/rustdoc-json/fns/generics.rs"
stdout: none
