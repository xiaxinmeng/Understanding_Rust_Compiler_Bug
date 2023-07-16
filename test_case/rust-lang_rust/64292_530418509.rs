
2019-09-10T19:17:04.0564321Z normalized stderr:
2019-09-10T19:17:04.0564553Z error[E0597]: `x` does not live long enough
2019-09-10T19:17:04.0565934Z   --> $DIR/async-fn.rs:18:13
2019-09-10T19:17:04.0566063Z    |
2019-09-10T19:17:04.0566130Z 18 |     async { x + y }.await
2019-09-10T19:17:04.0566458Z    |     --------^------
2019-09-10T19:17:04.0566539Z    |     |     | |
2019-09-10T19:17:04.0566641Z    |     |     | borrowed value does not live long enough
2019-09-10T19:17:04.0566727Z    |     |     value captured here by generator
2019-09-10T19:17:04.0566873Z    |     a temporary with access to the borrow is created here ...
2019-09-10T19:17:04.0566971Z 19 | }
2019-09-10T19:17:04.0567254Z    | -
2019-09-10T19:17:04.0567323Z    | |
2019-09-10T19:17:04.0567409Z    | `x` dropped here while still borrowed
2019-09-10T19:17:04.0567524Z    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `impl std::future::Future`
2019-09-10T19:17:04.0567653Z    |
2019-09-10T19:17:04.0568341Z    = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.
2019-09-10T19:17:04.0568497Z 
2019-09-10T19:17:04.0568622Z error[E0597]: `y` does not live long enough
2019-09-10T19:17:04.0568922Z   --> $DIR/async-fn.rs:18:17
2019-09-10T19:17:04.0569038Z    |
2019-09-10T19:17:04.0569103Z 18 |     async { x + y }.await
2019-09-10T19:17:04.0569731Z    |     ------------^--
2019-09-10T19:17:04.0569811Z    |     |     |     |
2019-09-10T19:17:04.0569908Z    |     |     |     borrowed value does not live long enough
2019-09-10T19:17:04.0569999Z    |     |     value captured here by generator
2019-09-10T19:17:04.0570108Z    |     a temporary with access to the borrow is created here ...
2019-09-10T19:17:04.0570189Z 19 | }
2019-09-10T19:17:04.0570464Z    | -
2019-09-10T19:17:04.0570534Z    | |
2019-09-10T19:17:04.0570619Z    | `y` dropped here while still borrowed
2019-09-10T19:17:04.0570726Z    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `impl std::future::Future`
2019-09-10T19:17:04.0570849Z    |
2019-09-10T19:17:04.0571489Z    = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.
2019-09-10T19:17:04.0571685Z 
2019-09-10T19:17:04.0571753Z error: aborting due to 2 previous errors
