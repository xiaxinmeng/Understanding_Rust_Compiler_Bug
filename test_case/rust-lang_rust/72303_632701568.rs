diff
  Description	`iter`	`future`
  Direct function	`from_fn`	`poll_fn`
- Ends instantly	`empty`	`ready`/`lazy`
+ Ends instantly	`empty`	(none)
- Continues forever	`repeat`/`repeat_with`	`pending`
+ Continues forever	`repeat`/`repeat_with`	(none)
- Continues once	`once`/`once_with`	`yield`/`yield_with` (although these don't exist yet)
+ Continues once	`once`/`once_with`	`ready`/`lazy`
