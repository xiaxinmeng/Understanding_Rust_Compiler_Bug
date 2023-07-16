plain
travis_time:end:2c3d9600:start=1543604043754178478,finish=1543604101565207354,duration=57811028876
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:58] .................................................................................................... 300/5104
[00:49:00] .................................................................................................... 400/5104
[00:49:04] .................................................................................................... 500/5104
[00:49:07] ..............................i..................................................................... 600/5104
[00:49:10] ................................................F.F....F...............................F............ 700/5104
[00:49:16] .......F.F...............................F.........F.............................................i.. 800/5104
[00:49:24] ....................iiiii........................................................................... 1000/5104
[00:49:26] .................................................................................................... 1100/5104
[00:49:28] .................................................................................................... 1200/5104
[00:49:31] .................................................................................................... 1300/5104
---
[00:51:35] diff of stderr:
[00:51:35] 
[00:51:35] 13    |         ^^^^^^^^^
[00:51:35] 14 
[00:51:35] 15 error: any use of this value will cause an error
[00:51:35] +   --> $DIR/const-err-multi.rs:13:1
[00:51:35] +    |
[00:51:35] + LL | pub const A: i8 = -std::i8::MIN;
[00:51:35] +    |                   |
[00:51:35] +    |                   attempt to negate with overflow
[00:51:35] + 
[00:51:35] + 
[00:51:35] + error: any use of this value will cause an error
[00:51:35] 17    |
[00:51:35] 17    |
[00:51:35] 18 LL | pub const B: i8 = A;
[00:51:35] 36    |                   |
[00:51:35] 37    |                   referenced constant has errors
[00:51:35] 38 
[00:51:35] - error: aborting due to 4 previous errors
[00:51:35] - error: aborting due to 4 previous errors
[00:51:35] + error: aborting due to 5 previous errors
[00:51:35] 40 
[00:51:35] 41 
[00:51:35] 
[00:51:35] 
[00:51:35] The actual stderr differed from the expected stderr.
[00:51:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-multi/const-err-multi.stderr
[00:51:35] To update references, rerun the tests and pass the `--bless` flag
[00:51:35] To only update this specific test, also pass `--test-args consts/const-err-multi.rs`
[00:51:35] error: 1 errors occurred comparing output.
[00:51:35] status: exit code: 1
[00:51:35] status: exit code: 1
[00:51:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "ile_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![deny(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-err-multi.rs:13:1\n   |\nLL | pub const A: i8 = -std::i8::MIN;\n   | ^^^^^^^^^^^^^^^^^^-------------^\n   |                   |\n   |                   attempt to negate with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-err-multi.rs:11:9\n   |\nLL | #![deny(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":506,"byte_end":519,"line_start":13,"line_end":13,"column_start":19,"column_end":32,"is_primary":false,"text":[{"text":"pub const A: i8 = -std::i8::MIN;","highlight_start":19,"highlight_end":32}],"label":"attempt to negate with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":488,"byte_end":520,"line_start":13,"line_end":13,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"pub const A: i8 = -std::i8::MIN;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replaces":[{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":602,"byte_end":609,"line_start":17,"line_end":17,"column_start":19,"column_end":26,"is_primary":false,"text":[{"text":"pub const C: u8 = A as u8;","highlight_start":19,"highlight_end":26}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":584,"byte_end":610,"line_start":17,"line_end":17,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"pub const C: u8 = A as u8;","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-err-multi.rs:17:1\n   |\nLL | pub const C: u8 = A as u8;\n   | ^^^^^^^^^^^^^^^^^^-------^\n   |                   |\n   |                   referenced constant has errors\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":650,"byte_end":656,"line_start":19,"line_end":19,"column_start":19,"column_end":25,"is_primary":false,"text":[{"text":"pub const D: i8 = 50 - A;","highlight_start":19,"highlight_end":25}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-err-multi.rs","byte_start":632,"byte_end":617,"highlight_end":25}],"label":"index out of bounds: the len is 1 but the index is 1","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-err.rs","byte_start":613,"byte_end":638,"line_start":20,"line_end":20,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"const FOO: u8 = [5u8][1];","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err.rs","byte_start":552,"byte_end":561,"line_start":14,"line_end":14,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-err.rs:20:1\n   |\nLL | const FOO: u8 = [5u8][1];\n   | ^^^^^^^^^^^^^^^^--------^\n   |                 |\n   |                 index out of bounds: the len is 1 but the index is 1\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-err.rs:14:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-err.rs","byte_start":629,"byte_end":637,"line_start----------------------------------------
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":542,"byte_end":547,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":false,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":524,"byte_end":574,"line_start":15,"line_end":15,"column_start":1,"column_end":51,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":1,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":475,"byte_end":484,"line_start":11,"line_end":11,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:15:1\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   | ^^^^^^^^^^^^^^^^^^-----^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |                   |\n   |                   attempt to subtract with overflow\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:11:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[00:51:35] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":542,"byte_end":547,"line_start":15,"line_end":15,"column_start":19,"column_end":24,"is_primary":false,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":19,"highlight_end":24}],"label":"attempt to subtract with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs","byte_start":524,"byte_end":574,"line_start":15,"line_end":15,"column_start":1,"column_end":51,"is_primary":true,"text":[{"text":"const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];","highlight_start":1,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs:15:1\n   |\nLL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];\n   | ^^^^^^^^^^^^^^^^^^-----^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |                   |\n   |                   attempt to subtract with overflow\n\n"}
[00:51:35] {"message":"evaluation of constant expression failed","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n