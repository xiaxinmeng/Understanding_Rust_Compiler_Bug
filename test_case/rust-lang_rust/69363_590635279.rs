plain
2020-02-25T00:33:08.6744820Z ========================== Starting Command Output ===========================
2020-02-25T00:33:08.6763334Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9ba41f73-1cec-4590-8cd4-2ce9c22e7fb8.sh
2020-02-25T00:33:08.9645550Z 
2020-02-25T00:33:08.9736730Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T00:33:08.9772336Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-25T00:33:08.9789392Z Task         : Get sources
2020-02-25T00:33:08.9790075Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T00:33:08.9790605Z Version      : 1.0.0
2020-02-25T00:33:08.9790979Z Author       : Microsoft
---
2020-02-25T00:33:11.5186092Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T00:33:11.5301626Z ##[command]git config gc.auto 0
2020-02-25T00:33:11.5336841Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T00:33:11.5371013Z ##[command]git config --get-all http.proxy
2020-02-25T00:33:11.5464223Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69363/merge:refs/remotes/pull/69363/merge
---
2020-02-25T01:29:50.8443724Z    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-02-25T01:30:27.4207181Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-25T01:30:27.7732092Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-02-25T01:30:35.9353457Z error: type parameter on variant
2020-02-25T01:30:35.9354240Z     --> src/librustc_codegen_ssa/back/write.rs:1002:66
2020-02-25T01:30:35.9354776Z      |
2020-02-25T01:30:35.9356269Z 1002 |             drop(coordinator_send2.send(Box::new(Message::Token::<B>(token))));
2020-02-25T01:30:35.9357951Z      |
2020-02-25T01:30:35.9357951Z      |
2020-02-25T01:30:35.9358636Z      = note: `-D type-param-on-variant-ctor` implied by `-D warnings`
2020-02-25T01:30:35.9359349Z help: set the type parameter on the enum
2020-02-25T01:30:35.9359838Z      |
2020-02-25T01:30:35.9360625Z 1002 |             drop(coordinator_send2.send(Box::new(Message::<B>::Token(token))));
2020-02-25T01:30:35.9424245Z 
2020-02-25T01:30:35.9960649Z error: type parameter on variant
2020-02-25T01:30:35.9960649Z error: type parameter on variant
2020-02-25T01:30:35.9961523Z     --> src/librustc_codegen_ssa/back/write.rs:1558:40
2020-02-25T01:30:35.9962119Z      |
2020-02-25T01:30:35.9963174Z 1558 |                         Message::Done::<B> { result: Ok(m), worker_id }
2020-02-25T01:30:35.9964613Z      |
2020-02-25T01:30:35.9965137Z help: set the type parameter on the enum
2020-02-25T01:30:35.9965626Z      |
2020-02-25T01:30:35.9965626Z      |
2020-02-25T01:30:35.9966389Z 1558 |                         Message::<B>::Done { result: Ok(m), worker_id }
2020-02-25T01:30:35.9975477Z 
2020-02-25T01:30:36.0071709Z error: type parameter on variant
2020-02-25T01:30:36.0071709Z error: type parameter on variant
2020-02-25T01:30:36.0072410Z     --> src/librustc_codegen_ssa/back/write.rs:1561:47
2020-02-25T01:30:36.0072965Z      |
2020-02-25T01:30:36.0073750Z 1561 |                         Message::NeedsFatLTO::<B> { result: m, worker_id }
2020-02-25T01:30:36.0075184Z      |
2020-02-25T01:30:36.0075707Z help: set the type parameter on the enum
2020-02-25T01:30:36.0076191Z      |
2020-02-25T01:30:36.0076191Z      |
2020-02-25T01:30:36.0076963Z 1561 |                         Message::<B>::NeedsFatLTO { result: m, worker_id }
2020-02-25T01:30:36.0078487Z 
2020-02-25T01:30:36.0117922Z error: type parameter on variant
2020-02-25T01:30:36.0117922Z error: type parameter on variant
2020-02-25T01:30:36.0118830Z     --> src/librustc_codegen_ssa/back/write.rs:1564:48
2020-02-25T01:30:36.0119358Z      |
2020-02-25T01:30:36.0120165Z 1564 |                         Message::NeedsThinLTO::<B> { name, thin_buffer, worker_id }
2020-02-25T01:30:36.0121601Z      |
2020-02-25T01:30:36.0122142Z help: set the type parameter on the enum
2020-02-25T01:30:36.0122633Z      |
2020-02-25T01:30:36.0122633Z      |
2020-02-25T01:30:36.0123408Z 1564 |                         Message::<B>::NeedsThinLTO { name, thin_buffer, worker_id }
2020-02-25T01:30:36.0135302Z 
2020-02-25T01:30:36.0190731Z error: type parameter on variant
2020-02-25T01:30:36.0190731Z error: type parameter on variant
2020-02-25T01:30:36.0191429Z     --> src/librustc_codegen_ssa/back/write.rs:1567:40
2020-02-25T01:30:36.0191955Z      |
2020-02-25T01:30:36.0192781Z 1567 |                         Message::Done::<B> { result: Err(Some(WorkerFatalError)), worker_id }
2020-02-25T01:30:36.0194201Z      |
2020-02-25T01:30:36.0194747Z help: set the type parameter on the enum
2020-02-25T01:30:36.0195240Z      |
2020-02-25T01:30:36.0195240Z      |
2020-02-25T01:30:36.0196037Z 1567 |                         Message::<B>::Done { result: Err(Some(WorkerFatalError)), worker_id }
2020-02-25T01:30:36.0200968Z 
2020-02-25T01:30:36.0230876Z error: type parameter on variant
2020-02-25T01:30:36.0230876Z error: type parameter on variant
2020-02-25T01:30:36.0231600Z     --> src/librustc_codegen_ssa/back/write.rs:1569:44
2020-02-25T01:30:36.0232131Z      |
2020-02-25T01:30:36.0232911Z 1569 |                     None => Message::Done::<B> { result: Err(None), worker_id },
2020-02-25T01:30:36.0234362Z      |
2020-02-25T01:30:36.0234886Z help: set the type parameter on the enum
2020-02-25T01:30:36.0235399Z      |
2020-02-25T01:30:36.0235399Z      |
2020-02-25T01:30:36.0236169Z 1569 |                     None => Message::<B>::Done { result: Err(None), worker_id },
2020-02-25T01:30:36.0237492Z 
2020-02-25T01:30:36.0435311Z error: type parameter on variant
2020-02-25T01:30:36.0435311Z error: type parameter on variant
2020-02-25T01:30:36.0436183Z     --> src/librustc_codegen_ssa/back/write.rs:1805:76
2020-02-25T01:30:36.0436842Z      |
2020-02-25T01:30:36.0437953Z 1805 |         drop(self.coordinator_send.send(Box::new(Message::CodegenComplete::<B>)));
2020-02-25T01:30:36.0440097Z      |
2020-02-25T01:30:36.0440735Z help: set the type parameter on the enum
2020-02-25T01:30:36.0441350Z      |
2020-02-25T01:30:36.0441350Z      |
2020-02-25T01:30:36.0442299Z 1805 |         drop(self.coordinator_send.send(Box::new(Message::<B>::CodegenComplete)));
2020-02-25T01:30:36.0447838Z 
2020-02-25T01:30:36.0502012Z error: type parameter on variant
2020-02-25T01:30:36.0502012Z error: type parameter on variant
2020-02-25T01:30:36.0502721Z     --> src/librustc_codegen_ssa/back/write.rs:1817:75
2020-02-25T01:30:36.0503243Z      |
2020-02-25T01:30:36.0504347Z 1817 |         drop(self.coordinator_send.send(Box::new(Message::CodegenAborted::<B>)));
2020-02-25T01:30:36.0505887Z      |
2020-02-25T01:30:36.0506407Z help: set the type parameter on the enum
2020-02-25T01:30:36.0506883Z      |
2020-02-25T01:30:36.0506883Z      |
2020-02-25T01:30:36.0507630Z 1817 |         drop(self.coordinator_send.send(Box::new(Message::<B>::CodegenAborted)));
2020-02-25T01:30:36.0512675Z 
2020-02-25T01:30:36.0578949Z error: type parameter on variant
2020-02-25T01:30:36.0578949Z error: type parameter on variant
2020-02-25T01:30:36.0579666Z     --> src/librustc_codegen_ssa/back/write.rs:1846:65
2020-02-25T01:30:36.0580195Z      |
2020-02-25T01:30:36.0581600Z 1846 |     drop(tx_to_llvm_workers.send(Box::new(Message::CodegenDone::<B> { llvm_work_item, cost })));
2020-02-25T01:30:36.0583762Z      |
2020-02-25T01:30:36.0584298Z help: set the type parameter on the enum
2020-02-25T01:30:36.0584822Z      |
2020-02-25T01:30:36.0584822Z      |
2020-02-25T01:30:36.0585638Z 1846 |     drop(tx_to_llvm_workers.send(Box::new(Message::<B>::CodegenDone { llvm_work_item, cost })));
2020-02-25T01:30:36.0599959Z 
2020-02-25T01:30:36.0644231Z error: type parameter on variant
2020-02-25T01:30:36.0644231Z error: type parameter on variant
2020-02-25T01:30:36.0645264Z     --> src/librustc_codegen_ssa/back/write.rs:1855:65
2020-02-25T01:30:36.0646737Z      |
2020-02-25T01:30:36.0647649Z 1855 |     drop(tx_to_llvm_workers.send(Box::new(Message::CodegenDone::<B> { llvm_work_item, cost: 0 })));
2020-02-25T01:30:36.0649572Z      |
2020-02-25T01:30:36.0650100Z help: set the type parameter on the enum
2020-02-25T01:30:36.0650761Z      |
2020-02-25T01:30:36.0650761Z      |
2020-02-25T01:30:36.0651606Z 1855 |     drop(tx_to_llvm_workers.send(Box::new(Message::<B>::CodegenDone { llvm_work_item, cost: 0 })));
2020-02-25T01:30:36.0653100Z 
2020-02-25T01:30:36.0728256Z error: type parameter on variant
2020-02-25T01:30:36.0728256Z error: type parameter on variant
2020-02-25T01:30:36.0729293Z     --> src/librustc_codegen_ssa/back/write.rs:1875:73
2020-02-25T01:30:36.0729889Z      |
2020-02-25T01:30:36.0731598Z 1875 |     drop(tx_to_llvm_workers.send(Box::new(Message::AddImportOnlyModule::<B> {
2020-02-25T01:30:36.0734326Z      |
2020-02-25T01:30:36.0734854Z help: set the type parameter on the enum
2020-02-25T01:30:36.0735361Z      |
2020-02-25T01:30:36.0735361Z      |
2020-02-25T01:30:36.0736127Z 1875 |     drop(tx_to_llvm_workers.send(Box::new(Message::<B>::AddImportOnlyModule {
2020-02-25T01:30:36.0737600Z 
2020-02-25T01:30:37.4185840Z error: aborting due to 11 previous errors
2020-02-25T01:30:37.4189999Z 
2020-02-25T01:30:37.4284478Z error: could not compile `rustc_codegen_ssa`.
---
2020-02-25T01:31:18.7913384Z   local time: Tue Feb 25 01:31:18 UTC 2020
2020-02-25T01:31:19.0908473Z   network time: Tue, 25 Feb 2020 01:31:19 GMT
2020-02-25T01:31:19.0911600Z == end clock drift check ==
2020-02-25T01:31:19.6658677Z 
2020-02-25T01:31:19.6733412Z ##[error]Bash exited with code '1'.
2020-02-25T01:31:19.6747250Z ##[section]Finishing: Run build
2020-02-25T01:31:19.6795206Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-25T01:31:19.6799994Z Task         : Get sources
2020-02-25T01:31:19.6800332Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T01:31:19.6800628Z Version      : 1.0.0
2020-02-25T01:31:19.6800836Z Author       : Microsoft
2020-02-25T01:31:19.6800836Z Author       : Microsoft
2020-02-25T01:31:19.6801183Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T01:31:19.6801564Z ==============================================================================
2020-02-25T01:31:20.0218554Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T01:31:20.0268728Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-25T01:31:20.0357471Z Cleaning up task key
2020-02-25T01:31:20.0358709Z Start cleaning up orphan processes.
2020-02-25T01:31:20.0536460Z Terminate orphan process: pid (4250) (python)
2020-02-25T01:31:20.0639290Z ##[section]Finishing: Finalize Job
