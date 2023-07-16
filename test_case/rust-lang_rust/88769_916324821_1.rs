sh
bugpoint --opt-command=$(echo ~/.rustup/toolchains/nightly-2021-09-08-x*64*/lib/rustlib/x*64*/bin/opt) -O3 issue-88769-min.*-cgu.3.rcgu.no-opt.bc
