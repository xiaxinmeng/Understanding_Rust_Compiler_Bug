plain
travis_time:end:0e1bcaa7:start=1556383647209041615,finish=1556383721654428037,duration=74445386422
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:34:12] tests/run-pass/update-references.sh '/tmp/compiletesty784bP' 'async-fn.rs'
[01:34:12] 
[01:34:12] error: 1 errors occurred comparing output.
[01:34:12] status: exit code: 1
[01:34:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletesty784bP" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesty784bP/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletesty784bP/async-fn.stage-id.aux" "-A" "unused"
[01:34:12] ------------------------------------------
[01:34:12] 
[01:34:12] ------------------------------------------
[01:34:12] stderr:
---
[01:34:16] tests/run-pass/update-references.sh '/tmp/compiletesty784bP' 'hashmap.rs'
[01:34:16] 
[01:34:16] error: 1 errors occurred comparing output.
[01:34:16] status: exit code: 1
[01:34:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletesty784bP" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesty784bP/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletesty784bP/hashmap.stage-id.aux" "-A" "unused"
[01:34:16] ------------------------------------------
[01:34:16] 
[01:34:16] ------------------------------------------
[01:34:16] stderr:
[01:34:16] stderr:
[01:34:16] ------------------------------------------
[01:34:16] {"message":"Miri evaluation error: tried to call a function with ABI RustIntrinsic using caller ABI PlatformIntrinsic","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n