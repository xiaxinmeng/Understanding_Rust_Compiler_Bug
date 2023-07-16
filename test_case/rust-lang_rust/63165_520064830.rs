plain
2019-08-09T18:58:18.5153259Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T18:58:18.5153308Z 
2019-08-09T18:58:18.5153547Z   git checkout -b <new-branch-name>
2019-08-09T18:58:18.5153603Z 
2019-08-09T18:58:18.5153892Z HEAD is now at 0329c99a4 Auto merge of #63165 - xen0n:mips64-musl-targets, r=alexcrichton
2019-08-09T18:58:18.5313819Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T18:58:18.5316707Z ==============================================================================
2019-08-09T18:58:18.5316810Z Task         : Bash
2019-08-09T18:58:18.5316876Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T21:07:40.0677910Z     Finished release [optimized] target(s) in 5m 20s
2019-08-09T21:07:40.0840987Z [TIMING] ToolBuild { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-musl" }, target: "x86_64-unknown-linux-musl", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: ["clippy"] } -- 320.848
2019-08-09T21:07:59.0686344Z 
2019-08-09T21:07:59.0688007Z 
2019-08-09T21:07:59.0689732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-musl/stage0-tools-bin/fabricate" "generate" "--product-name=Rust" "--rel-manifest-dir=rustlib" "--success-message=RLS-ready-to-serve." "--image-dir" "/checkout/obj/build/tmp/dist/rls-image" "--work-dir" "/checkout/obj/build/tmp/dist" "--output-dir" "/checkout/obj/build/dist" "--non-installed-overlay" "/checkout/obj/build/tmp/dist/rls-overlay" "--package-name=rls-nightly-x86_64-unknown-linux-musl" "--legacy-manifest-dirs=rustlib,cargo" "--component-name=rls-preview"
2019-08-09T21:07:59.0690489Z expected success, got: signal: 11
2019-08-09T21:07:59.0690903Z 
2019-08-09T21:07:59.0693928Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
2019-08-09T21:07:59.0694383Z Build completed unsuccessfully in 2:01:25
2019-08-09T21:07:59.0694383Z Build completed unsuccessfully in 2:01:25
2019-08-09T21:08:02.7663572Z ##[error]Bash exited with code '1'.
2019-08-09T21:08:02.7699022Z ##[section]Starting: Upload CPU usage statistics
2019-08-09T21:08:02.7710334Z ==============================================================================
2019-08-09T21:08:02.7710447Z Task         : Bash
2019-08-09T21:08:02.7710518Z Description  : Run a Bash script on macOS, Linux, or Windows
