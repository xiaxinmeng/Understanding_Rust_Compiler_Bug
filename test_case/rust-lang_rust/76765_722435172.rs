diff
> -	error[E0623]: lifetime mismatch
> -	  --> $DIR/issue-76547.rs:20:13
> +	error: lifetime may not live long enough
> +	  --> $DIR/issue-76547.rs:19:14
> 	   |
> 	LL | async fn fut(bufs: &mut [&mut [u8]]) {
> -	   |                          ---------   -
> -	   |                          |           |
> -	   |                          |           this `async fn` implicitly returns an `impl Future<Output = ()>`
> -	   |                          this parameter and the returned future are declared with different lifetimes...
> -	LL |     ListFut(bufs).await
> -	   |             ^^^^ ...but data from `bufs` is held across an await point here
> +	   |              ^^^^  -     - let's call the lifetime of this reference `'2`
> +	   |              |     |
> +	   |              |     let's call the lifetime of this reference `'1`
> +	   |              assignment requires that `'1` must outlive `'2`
> 	
> -	error[E0623]: lifetime mismatch
> -	  --> $DIR/issue-76547.rs:34:14
> +	error: lifetime may not live long enough
> +	  --> $DIR/issue-76547.rs:33:15
> 	   |
> 	LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
> -	   |                           ---------      ---
> -	   |                           |              |
> -	   |                           |              this `async fn` implicitly returns an `impl Future<Output = i32>`
> -	   |                           this parameter and the returned future are declared with different lifetimes...
> -	LL |     ListFut2(bufs).await
> -	   |              ^^^^ ...but data from `bufs` is held across an await point here
> +	   |               ^^^^  -     - let's call the lifetime of this reference `'2`
> +	   |               |     |
> +	   |               |     let's call the lifetime of this reference `'1`
> +	   |               assignment requires that `'1` must outlive `'2`
> 	
> 	error: aborting due to 2 previous errors
> 	
> 
> -	For more information about this error, try `rustc --explain E0623`.
> 