
> error[E0599]: no variant or associated item named `Type` found for enum `ParamKindOrd` in the current scope
>    --> compiler/rustc_errors/src/diagnostic.rs:173:32
>     |
> 173 |             ast::ParamKindOrd::Type => "type",
>     |                                ^^^^ variant or associated item not found in `ParamKindOrd`
> 
> error[E0599]: no variant or associated item named `Const` found for enum `ParamKindOrd` in the current scope
>    --> compiler/rustc_errors/src/diagnostic.rs:174:32
>     |
> 174 |             ast::ParamKindOrd::Const => "const",
>     |                                ^^^^^ variant or associated item not found in `ParamKindOrd`
> 
> error[E0599]: no variant or associated item named `Infer` found for enum `ParamKindOrd` in the current scope
>    --> compiler/rustc_errors/src/diagnostic.rs:175:32
>     |
> 175 |             ast::ParamKindOrd::Infer => "infer",
>     |                                ^^^^^ variant or associated item not found in `ParamKindOrd`
> 