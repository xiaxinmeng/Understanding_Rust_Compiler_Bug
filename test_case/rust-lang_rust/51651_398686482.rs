
[00:43:35] ---- [ui] ui/nll/issue-51268.rs stdout ----
[00:43:35] diff of stderr:
[00:43:35] 
[00:43:35] 1	error[E0502]: cannot borrow `self.thing` as mutable because it is also borrowed as immutable
[00:43:35] -	  --> $DIR/issue-51268.rs:26:9
[00:43:35] +	  --> $DIR/issue-51268.rs:28:9
[00:43:35] 3	   |
[00:43:35] 4	LL |            self.thing.bar(|| {
[00:43:35] 5	   |            ^              -- immutable borrow occurs here
[00:43:35] 
[00:43:35] 
