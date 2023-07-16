plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/write.rs at line 724:
                 return format!("optimize module {}", m.name);
                 #[cfg(not(windows))]
                 return format!("opt {}", m.name);
+            }
+            }
             WorkItem::CopyPostLtoArtifacts(m) => {
                 #[cfg(windows)]
                 return format!("copy LTO artifacts for {}", m.name);
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/write.rs at line 731:
                 #[cfg(not(windows))]
                 return format!("copy {}", m.name);
+            }
+            }
             WorkItem::LTO(m) => {
                 #[cfg(windows)]
                 return format!("LTO module {}", m.name());
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/write.rs at line 1637:
 
 fn spawn_work<B: ExtraBackendMethods>(cgcx: CodegenContext<B>, work: WorkItem<B>) {
     let builder = thread::Builder::new().name(work.short_description());
-    builder.spawn(move || {
-        // Set up a destructor which will fire off a message that we're done as
-        // we exit.
-        struct Bomb<B: ExtraBackendMethods> {
-            coordinator_send: Sender<Box<dyn Any + Send>>,
-            result: Option<Result<WorkItemResult<B>, FatalError>>,
-            worker_id: usize,
-        }
-        impl<B: ExtraBackendMethods> Drop for Bomb<B> {
-            fn drop(&mut self) {
-                let worker_id = self.worker_id;
-                let msg = match self.result.take() {
-                    Some(Ok(WorkItemResult::Compiled(m))) => {
-                        Message::Done::<B> { result: Ok(m), worker_id }
-                    }
-                    Some(Ok(WorkItemResult::NeedsLink(m))) => {
-                        Message::NeedsLink::<B> { module: m, worker_id }
-                    }
-                    Some(Ok(WorkItemResult::NeedsFatLTO(m))) => {
-                        Message::NeedsFatLTO::<B> { result: m, worker_id }
-                    }
-                    Some(Ok(WorkItemResult::NeedsThinLTO(name, thin_buffer))) => {
-                        Message::NeedsThinLTO::<B> { name, thin_buffer, worker_id }
-                    }
-                    Some(Err(FatalError)) => {
-                        Message::Done::<B> { result: Err(Some(WorkerFatalError)), worker_id }
-                    }
-                    None => Message::Done::<B> { result: Err(None), worker_id },
-                };
-                drop(self.coordinator_send.send(Box::new(msg)));
+    builder
+        .spawn(move || {
+            // Set up a destructor which will fire off a message that we're done as
+            // we exit.
+            struct Bomb<B: ExtraBackendMethods> {
+                coordinator_send: Sender<Box<dyn Any + Send>>,
+                result: Option<Result<WorkItemResult<B>, FatalError>>,
+                worker_id: usize,
-        }
-        }
+            impl<B: ExtraBackendMethods> Drop for Bomb<B> {
+                fn drop(&mut self) {
+                    let worker_id = self.worker_id;
+                    let msg = match self.result.take() {
+                        Some(Ok(WorkItemResult::Compiled(m))) => {
+                            Message::Done::<B> { result: Ok(m), worker_id }
+                        }
+                        Some(Ok(WorkItemResult::NeedsLink(m))) => {
+                            Message::NeedsLink::<B> { module: m, worker_id }
+                        }
+                        Some(Ok(WorkItemResult::NeedsFatLTO(m))) => {
+                            Message::NeedsFatLTO::<B> { result: m, worker_id }
+                        }
+                        Some(Ok(WorkItemResult::NeedsThinLTO(name, thin_buffer))) => {
+                            Message::NeedsThinLTO::<B> { name, thin_buffer, worker_id }
+                        }
+                        Some(Err(FatalError)) => {
+                            Message::Done::<B> { result: Err(Some(WorkerFatalError)), worker_id }
+                        }
+                        None => Message::Done::<B> { result: Err(None), worker_id },
+                    };
+                    drop(self.coordinator_send.send(Box::new(msg)));
+            }
 
 
-        let mut bomb = Bomb::<B> {
-            coordinator_send: cgcx.coordinator_send.clone(),
-            result: None,
-            worker_id: cgcx.worker,
-        };
+            let mut bomb = Bomb::<B> {
+                coordinator_send: cgcx.coordinator_send.clone(),
+                result: None,
+                worker_id: cgcx.worker,
 
 
-        // Execute the work itself, and if it finishes successfully then flag
-        // ourselves as a success as well.
-        //
-        // Note that we ignore any `FatalError` coming out of `execute_work_item`,
-        // as a diagnostic was already sent off to the main thread - just
-        // surface that there was an error in this worker.
-        bomb.result = {
-            let _prof_timer = work.start_profiling(&cgcx);
-            Some(execute_work_item(&cgcx, work))
-        };
-    }).expect("failed to spawn thread");
+            // Execute the work itself, and if it finishes successfully then flag
+            // ourselves as a success as well.
+            //
+            // Note that we ignore any `FatalError` coming out of `execute_work_item`,
+            // as a diagnostic was already sent off to the main thread - just
+            // surface that there was an error in this worker.
+            bomb.result = {
+                let _prof_timer = work.start_profiling(&cgcx);
+                Some(execute_work_item(&cgcx, work))
+        })
+        })
+        .expect("failed to spawn thread");
 
 
 enum SharedEmitterMessage {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/lib.rs" "/checkout/compiler/rustc_mir/src/borrow_check/universal_regions.rs" "/checkout/compiler/rustc_mir/src/borrow_check/facts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
