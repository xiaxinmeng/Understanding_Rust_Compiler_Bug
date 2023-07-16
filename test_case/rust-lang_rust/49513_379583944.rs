plain
Resolving deltas: 100% (611130/611130), completed with 4848 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:42:57] ............................................................................i.......................
[00:43:03] ...................i................................................................................
---
[00:43:39] .................................................................................................i..
[00:43:46] .......................................................................i............................
[00:43:52] ....................................................................................................
[00:43:59] ....................................................................................................
[00:44:07] ...............................................................................................FF...
---
[00:44:08] - error[E0080]: enums without inhabited variants do not have discriminants
[00:44:08] + error[E0391]: cyclic dependency detected
[00:44:08] 2   --> $DIR/uninhabited_enum_discriminant1.rs:14:9
[00:44:08] 3    |
[00:44:08] 4 LL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants
[00:44:08]
[00:44:08] +    |         ^^^^^^^^^^^^^ cyclic reference
[00:44:08] +    |
[00:44:08] + note: the cycle begins when const-evaluating `X::A::{{initializer}}`...
[00:44:08] +   --> $DIR/uninhabited_enum_discriminant1.rs:14:9
[00:44:08] +    |
[00:44:08] + LL |     A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants
[00:44:08] 5    |         ^^^^^^^^^^^^^
[00:44:08] +    = note: ...which then again requires const-evaluating `X::A::{{initializer}}`, completing the cycle.
---
[00:44:08] Actual stderr saved to /checkout/oait SecondTrait : FirstTrait {\n\n}\n