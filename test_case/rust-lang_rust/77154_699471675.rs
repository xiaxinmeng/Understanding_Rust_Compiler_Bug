
failures:

---- [ui] ui/stdout-during-shutdown.rs stdout ----
diff of run.stdout:

-	hello, world!


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stdout-during-shutdown/stdout-during-shutdown.run.stdout
normalized run.stderr:
stdio streams had content in them that was not flushed. you should set EXIT_RUNTIME to 1 (see the FAQ), or make sure to emit a newline when you printf etc.
