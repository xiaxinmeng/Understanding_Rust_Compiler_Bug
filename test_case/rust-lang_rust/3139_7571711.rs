
diff --git a/src/libcore/pipes.rs b/src/libcore/pipes.rs
index b6b54a4..e415086 100644
--- a/src/libcore/pipes.rs
+++ b/src/libcore/pipes.rs
@@ -111,6 +111,22 @@ type buffer<T: send> = {
     data: T,
 };

+#[doc(hidden)]
+fn strengthen_task(task: *rust_task) -> *rust_task unsafe {
+    // Sets the low bit on a task, indicating that it needs a deref.
+
+    let x : libc::uintptr_t = reinterpret_cast(task);
+    reinterpret_cast(x | 1)
+}
+
+fn weaken_task(task: *rust_task) -> (*rust_task, bool) unsafe {
+    let x : libc::uintptr_t = reinterpret_cast(task);
+    let strong = x & 1;
+    // 0 the low bit.
+    let x = (x >> 1) << 1;
+    (reinterpret_cast(x), if strong == 1 { true } else { false })
+}
+
 struct packet_header {
     let mut state: state;
     let mut blocked_task: *rust_task;
@@ -128,6 +144,7 @@ struct packet_header {
     // Returns the old state.
     unsafe fn mark_blocked(this: *rust_task) -> state {
         rustrt::rust_task_ref(this);
+        let this = strengthen_task(this);
         let old_task = swap_task(self.blocked_task, this);
         assert old_task.is_null();
         swap_state_acq(self.state, blocked)
@@ -135,7 +152,11 @@ struct packet_header {

     unsafe fn unblock() {
         let old_task = swap_task(self.blocked_task, ptr::null());
-        if !old_task.is_null() { rustrt::rust_task_deref(old_task) }
+        if !old_task.is_null() {
+            let (old_task, deref) = weaken_task(old_task);
+            assert deref;
+            rustrt::rust_task_deref(old_task)
+        }
         match swap_state_acq(self.state, empty) {
           empty | blocked => (),
           terminated => self.state = terminated,
@@ -358,9 +379,12 @@ fn send<T: send, Tbuffer: send>(-p: send_packet_buffered<T, Tbuffer>,
             debug!{"waking up task for %?", p_};
             let old_task = swap_task(p.header.blocked_task, ptr::null());
             if !old_task.is_null() {
+                let (old_task, deref) = weaken_task(old_task);
                 rustrt::task_signal_event(
                     old_task, ptr::addr_of(p.header) as *libc::c_void);
-                rustrt::rust_task_deref(old_task);
+                if deref {
+                    rustrt::rust_task_deref(old_task);
+                }
             }

             // The receiver will eventually clean this up.
@@ -411,9 +435,7 @@ fn try_recv<T: send, Tbuffer: send>(-p: recv_packet_buffered<T, Tbuffer>)
     // regular path
     let this = rustrt::rust_get_task();
     rustrt::task_clear_event_reject(this);
-    rustrt::rust_task_ref(this);
-    let old_task = swap_task(p.header.blocked_task, this);
-    assert old_task.is_null();
+    p.header.blocked_task = this;
     let mut first = true;
     let mut count = SPIN_COUNT;
     loop {
@@ -443,10 +465,7 @@ fn try_recv<T: send, Tbuffer: send>(-p: recv_packet_buffered<T, Tbuffer>)
           full => {
             let mut payload = none;
             payload <-> p.payload;
-            let old_task = swap_task(p.header.blocked_task, ptr::null());
-            if !old_task.is_null() {
-                rustrt::rust_task_deref(old_task);
-            }
+            p.header.blocked_task = ptr::null();
             p.header.state = empty;
             return some(option::unwrap(payload))
           }
@@ -454,11 +473,7 @@ fn try_recv<T: send, Tbuffer: send>(-p: recv_packet_buffered<T, Tbuffer>)
             // This assert detects when we've accidentally unsafely
             // casted too big of a number to a state.
             assert old_state == terminated;
-
-            let old_task = swap_task(p.header.blocked_task, ptr::null());
-            if !old_task.is_null() {
-                rustrt::rust_task_deref(old_task);
-            }
+            p.header.blocked_task = ptr::null();
             return none;
           }
         }
@@ -494,10 +509,13 @@ fn sender_terminate<T: send>(p: *packet<T>) {
         // wake up the target
         let old_task = swap_task(p.header.blocked_task, ptr::null());
         if !old_task.is_null() {
+            let (old_task, deref) = weaken_task(old_task);
             rustrt::task_signal_event(
                 old_task,
                 ptr::addr_of(p.header) as *libc::c_void);
-            rustrt::rust_task_deref(old_task);
+            if deref {
+                rustrt::rust_task_deref(old_task);
+            }
         }
         // The receiver will eventually clean up.
         //unsafe { forget(p) }
