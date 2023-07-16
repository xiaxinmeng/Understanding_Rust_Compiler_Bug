
[00:03:07] error[E0432]: unresolved import `Substitution`
[00:03:07]   --> /checkout/src/librustc_errors/diagnostic.rs:12:5
[00:03:07]    |
[00:03:07] 12 | use Substitution;
[00:03:07]    |     ^^^^^^^^^^^^ no `Substitution` in the root
[00:03:07] 
[00:03:07] error[E0560]: struct `CodeSuggestion` has no field named `substitution_parts`
[00:03:07]    --> /checkout/src/librustc_errors/diagnostic.rs:209:13
[00:03:07]     |
[00:03:07] 209 |             substitution_parts: vec![Substitution {
[00:03:07]     |             ^^^^^^^^^^^^^^^^^^^ field does not exist - did you mean `substitutes`?
[00:03:07] 
[00:03:07] error[E0560]: struct `CodeSuggestion` has no field named `substitution_parts`
[00:03:07]    --> /checkout/src/librustc_errors/diagnostic.rs:220:13
[00:03:07]     |
[00:03:07] 220 |             substitution_parts: vec![Substitution {
[00:03:07]     |             ^^^^^^^^^^^^^^^^^^^ field does not exist - did you mean `substitutes`?
[00:03:07] 
[00:03:07] error: no field `substitution_parts` on type `&CodeSuggestion`
[00:03:07]   --> /checkout/src/librustc_errors/emitter.rs:41:21
[00:03:07]    |
[00:03:07] 41 |                sugg.substitution_parts.len() == 1 &&
[00:03:07]    |                     ^^^^^^^^^^^^^^^^^^
[00:03:07] 
