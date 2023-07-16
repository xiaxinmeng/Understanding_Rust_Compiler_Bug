\n\nIf the item is not defined in the current module, it must be imported usi^^^^^^^^^^^^^^^^^ `Outer` redefined here
[01:10:40] +    |
[01:10:40] +    = note: `Outer` must be defined only once in the type namespace of this module
[01:10:40] + 
[01:10:40] + error[E0428]: the name `inner` is defined multiple times
[01:10:40] +   --> $DIR/generate-mod.rs:22:1
[01:10:40] +    |
[01:10:40] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:10:40] +    | ----------------------- previous definition of the module `inner` here
[01:10:40] + ...
[01:10:40] + LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:10:40] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `inner` redefined here
[01:10:40] +    |
[01:10:40] +    = note: `inner` must be defined only once in the type namespace of this module
[01:10:40] + 
[01:10:40] + error[E0428]: the name `Alias` is defined multiple times
[01:10:40] +   --> $DIR/generate-mod.rs:26:10
[01:10:40] +    |
[01:10:40] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:10:40] +    | ----------------------- previous definition of the type `Alias` here
[01:10:40] + ...
[01:10:40] + LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:10:40] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ `Alias` redefined here
[01:10:40] +    |
[01:10:40] +    = note: `Alias` must be defined only once in the type namespace of this module
[01:10:40] + 
[01:10:40] + error[E0428]: the name `Outer` is defined multiple times
[01:10:40] +   --> $DIR/generate-mod.rs:26:10
[01:10:40] +    |
[01:10:40] + LL | generate_mod::check!(); suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs","byte_start":644,"byte_end":667,"line_start":19,"line_end":19,"column_start":1,"column_end":24,"is_primary":false,"text":[{"text":"generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"generate_mod::check!","def_site_span":null}}],"children":[{"message":"`Alias` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0428]: the name `Alias` is defined multiple times\n  --> /checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs:22:1\n   |\nLL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope\n   | ----------------------- previous definition of the type `Alias` here\n...\nLL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Alias` redefined here\n   |\n   = note: `Alias` must be defined only once in the type namespace of this module\n\n"}
[01:10:40] {"message":"the name `Outer` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n