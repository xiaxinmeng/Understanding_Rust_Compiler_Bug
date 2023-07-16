rust
diff --git a/src/librustc/mir/interpret/error.rs b/src/librustc/mir/interpret/error.rs
index c3fe5d773a..f8c87f3393 100644
--- a/src/librustc/mir/interpret/error.rs
+++ b/src/librustc/mir/interpret/error.rs
@@ -413,7 +413,25 @@ impl<'tcx, O> EvalErrorKind<'tcx, O> {
                 "referenced constant has errors",
             Overflow(mir::BinOp::Add) => "attempt to add with overflow",
             Overflow(mir::BinOp::Sub) => "attempt to subtract with overflow",
-            Overflow(mir::BinOp::Mul) => "attempt to multiply with overflow",
+            Overflow(mir::BinOp::Mul) => {
+                //sys_common::backtrace::print(, PrintFormat::Short);
+                //extern crate sys;//HOW to?!
+                //use sys::backtrace;
+                //extern crate backtrace;
+                //use backtrace;
+                //#[cfg(feature = "backtrace")] //this is never true here!
+                {
+                    //extern crate sys_common;
+                    //use sys_common::rwlock::RWLock;
+                    //use sys_common::thread_info;
+                    //use sys_common::util;
+                    //use sys_common::backtrace;
+                    use backtrace::Backtrace;
+                    eprintln!("{:?}", Backtrace::new()); //thanks to WindowsBunny on irc for the idea!
+                }
+                "wuuuuuuattttttt??!attempt to multiply with overflow"
+            }
+                ,
             Overflow(mir::BinOp::Div) => "attempt to divide with overflow",
             Overflow(mir::BinOp::Rem) => "attempt to calculate the remainder with overflow",
             OverflowNeg => "attempt to negate with overflow",
