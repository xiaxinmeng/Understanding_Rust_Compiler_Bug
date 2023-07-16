\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32963.rs","byte_start":581,"byte_end":606,"line_start":18,"line_end":18,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    size_of_copy::<Misc+Copy>();","highlight_start":5,"highlight_end":30}],"label":"the trait `std::marker::Copy` is not implemented for `dyn Misc`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `size_of_copy`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32963.rs","byte_start":497,"byte_end":539,"line_start":15,"line_end":15,"column_start":1,"column_end":43,"is_primary":true,"text":[{"text":"fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `dyn Misc: std::marker::Copy` is not satisfied\n  --> /checkout/src/test/ui/issues/issue-32963.rs:18:5\n   |\nLL |     size_of_copy::<Misc+Copy>();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn Misc`\n   |\nnote: required by `size_of_copy`\n  --> /checkout/src/test/ui/issues/issue-32963.rs:15:1\n   |\nLL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:51:08] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:08] {"message":"Some errors occurred: E0225, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0225, E0277.\n"}
[00:51:08] {"message":"For more information about an error, try `rustc --explain E0225`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0225`.\n"}
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] thread '[ui] ui/issues/issue-32963.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:08] 
[00:51:08] 
[00:51:08] ---- [ui] ui/traits/trait-alias-object.rs stdout ----
[00:51:08] diff of stderr:
[00:51:08] 
[00:51:08] + error[E0225]: only auto traits can be used as additional traits in a trait object
[00:51:08] +   --> $DIR/trait-alias-object.rs:13:17
[00:51:08] +    |
[00:51:08] + LL | trait EqAlias = Eq;
[00:51:08] +    |                 ^^ non-auto additional trait
[00:51:08] + ...
[00:51:08] + LL |     let _: &dyn EqAlias = &123;
[00:51:08] +    |                 ------- expanded from this alia. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n