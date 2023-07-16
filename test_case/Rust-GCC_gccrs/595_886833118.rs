
> $ bin/gccrs test.rs 
> Preparing to parse files. 
> Attempting to parse file: test.rs
> beginning null denotation identifier handling
> current peek token when starting path pratt parse: 'as'
> current token (just about to return path to null denotation): 'as'
> finished null denotation identifier path parsing - next is branching 
> beginning null denotation identifier handling
> current peek token when starting path pratt parse: 'as'
> current token (just about to return path to null denotation): 'as'
> finished null denotation identifier path parsing - next is branching 
> beginning null denotation identifier handling
> current peek token when starting path pratt parse: '('
> current token (just about to return path to null denotation): '('
> finished null denotation identifier path parsing - next is branching 
> beginning null denotation identifier handling
> current peek token when starting path pratt parse: ')'
> current token (just about to return path to null denotation): ')'
> finished null denotation identifier path parsing - next is branching 
> SUCCESSFULLY PARSED CRATE 
> ran register_plugins (with no body)
> SUCCESSFULLY REGISTERED PLUGINS 
> started injection
> finished injection
> SUCCESSFULLY FINISHED INJECTION 
> started expansion
> finished expansion
> SUCCESSFULLY FINISHED EXPANSION 
> test.rs:6:5: fatal error: Failed to lower expr: [UnsafeBlockExpr:
>   outer attributes: noneBlockExpr:
> 
>   outer attributes: none
>   inner attributes: none
>  statements: 
>  
>   outer attributes: none
>  let a = foo
>  
>   outer attributes: none
>  let b = a as *const str
>  
>   outer attributes: none
>  let c = b as *const i8
>  ExprStmtWithoutBlock:
>   CallExpr: 
>    outer attributes: none
>  Function expr: puts
>  Call params:
>   c
>  final expression: none
> 
> ]
>     6 |     unsafe {
>       |     ^
> compilation terminated
> 