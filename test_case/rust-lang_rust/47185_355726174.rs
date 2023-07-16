
---- [ui] ui/issue-10969.rs stdout ----
	diff of stderr:

5	   |     ^^^
6	   |
7	note: defined here
+	  --> $DIR/issue-10969.rs:11:9
8	   |
9	11 | fn func(i: i32) {
10	   |         ^

12	error[E0618]: expected function, found `i32`
13	  --> $DIR/issue-10969.rs:16:5
14	   |
+	16 |     i(); //~ERROR expected function, found `i32`
-	16 |     i(); //~ERROR expected a function, I found `i32`!!
16	   |     ^^^
17	   |
18	note: defined here

19	  --> $DIR/issue-10969.rs:15:9
-	i is `i32`, not a function just like I mentioned before
21	   |
22	15 |     let i = 0i32;
23	   |         ^


The actual stderr differed from the expected stderr.
