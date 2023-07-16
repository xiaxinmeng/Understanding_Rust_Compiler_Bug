
[00:43:56] ---- [ui] ui/impl-trait/impl-generic-mismatch.rs stdout ----
[00:43:56] diff of stderr:
[00:43:56] 
[00:43:56] 9	help: try removing the generic parameter and using `impl Trait` instead
[00:43:56] 10	   |
[00:43:56] 11	LL |     fn foo(&self, _: &impl Debug) { }
[00:43:56] -	   |
[00:43:56] +	   |          --           ^^^^^^^^^^
[00:43:56] 13	
[00:43:56] 14	error[E0643]: method `bar` has incompatible signature for trait
[00:43:56] 15	  --> $DIR/impl-generic-mismatch.rs:27:23
