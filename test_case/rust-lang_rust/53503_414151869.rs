plain
[00:44:35] ....................................................................................................
[00:44:37] ....................................................................................................
[00:44:40] ....................................................................................................
[00:44:44] ........i...........................................................................................
[00:44:48] ..................................................................................F...........i.....
[00:44:51] ..i.................................................................................................
[00:44:56] ....................................................................................................
[00:44:58] ....................................................................................................
[00:45:01] ....................................................................................................
[00:45:03] ....................................................................................................
---
[00:46:12] ....................................................................................................
[00:46:16] .......................................i............................................................
[00:46:19] ....................................................................................................
[00:46:22] ....................................................................................................
":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const-evaluating `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":9648,"byte_end":9674,"line_start":291,"line_end":291,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"    unsafe { intrinsics::size_of::<T>() }","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing layout of `Foo`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"
