plain
2019-09-25T21:07:21.3680197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T21:07:21.3905429Z ##[command]git config gc.auto 0
2019-09-25T21:07:21.3968747Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T21:07:21.4030294Z ##[command]git config --get-all http.proxy
2019-09-25T21:07:21.4191195Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64749/merge:refs/remotes/pull/64749/merge
---
2019-09-25T21:12:55.2646486Z  ---> 85b05e3bb556
2019-09-25T21:12:55.2689694Z Successfully built 85b05e3bb556
2019-09-25T21:12:55.4783468Z Successfully tagged rust-ci:latest
2019-09-25T21:12:55.5283128Z Built container sha256:85b05e3bb5563f9a4bbf8035c089b5ad123879637385e48e9f5e4b7a47a706cb
2019-09-25T21:12:55.5301803Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/d75f4159e94a655bec830f29c217435ec5e0ac916ddc4020533e7b87fe06a0623bf7377f6d5cd1ca2fd627e09081e1dfacce244379f5b5e14a8d0757d0290bdc
2019-09-25T21:13:55.4626335Z upload failed: - to s3://rust-lang-ci-sccache2/docker/d75f4159e94a655bec830f29c217435ec5e0ac916ddc4020533e7b87fe06a0623bf7377f6d5cd1ca2fd627e09081e1dfacce244379f5b5e14a8d0757d0290bdc An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-09-25T21:13:56.9023660Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-09-25T21:13:56.9060771Z == clock drift check ==
2019-09-25T21:13:56.9071480Z   local time: Wed Sep 25 21:13:56 UTC 2019
2019-09-25T21:13:57.0589597Z   network time: Wed, 25 Sep 2019 21:13:57 GMT
---
2019-09-25T21:17:38.1899928Z    Compiling serde_json v1.0.40
2019-09-25T21:17:39.9179861Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-25T21:17:50.9652158Z     Finished release [optimized] target(s) in 1m 29s
2019-09-25T21:17:50.9742977Z tidy check
2019-09-25T21:17:51.2795158Z tidy error: /checkout/src/test/ui/nll/promoted-liveness.rs: too many trailing newlines (2)
2019-09-25T21:17:52.9259320Z some tidy checks failed
2019-09-25T21:17:52.9261252Z 
2019-09-25T21:17:52.9261252Z 
2019-09-25T21:17:52.9262281Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-25T21:17:52.9262532Z 
2019-09-25T21:17:52.9262551Z 
2019-09-25T21:17:52.9269265Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-25T21:17:52.9318749Z Build completed unsuccessfully in 0:01:32
2019-09-25T21:17:52.9318749Z Build completed unsuccessfully in 0:01:32
2019-09-25T21:17:52.9327183Z == clock drift check ==
2019-09-25T21:17:52.9342814Z   local time: Wed Sep 25 21:17:52 UTC 2019
2019-09-25T21:17:53.0846227Z   network time: Wed, 25 Sep 2019 21:17:53 GMT
2019-09-25T21:17:53.0851013Z == end clock drift check ==
2019-09-25T21:17:54.4245233Z ##[error]Bash exited with code '1'.
2019-09-25T21:17:54.4283041Z ##[section]Starting: Checkout
2019-09-25T21:17:54.4285933Z ==============================================================================
2019-09-25T21:17:54.4285988Z Task         : Get sources
2019-09-25T21:17:54.4286022Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
