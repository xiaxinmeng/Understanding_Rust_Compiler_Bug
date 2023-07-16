plain
[00:02:22] Successfully tagged rust-ci:latest
[00:02:22] Built container sha256:c845b7d551e749c0bb22e20fc11fe7b6e4dd7916eea0a0b290d852a6ed8e4bce
[00:02:22] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:22] 
[00:02:22] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:29] xargs: docker: terminated by signal 13

[00:02:30] travis_time:end:031e7680:start=1532649255770690380,finish=1532649394073173811,duration=138302483431
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:30] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[01:06:12]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2618:22
[01:06:15]      |
[01:06:15] 2618 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2619:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2619 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2620:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2620 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2640:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2640 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2641:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2641 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2661:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2661 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2662:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2662 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2663:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2663 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2710:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2710 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2711:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2711 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2712:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2712 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2747:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2747 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2748:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2748 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2773:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2773 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2774:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2774 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2775:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2775 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2776:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2776 |         let mut v4 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2836:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2836 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2837:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2837 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2838:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2838 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2839:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2839 |         let mut v4 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2886:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2886 |         let mut v1 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2887:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2887 |         let mut v2 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2888:22
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2888 |         let mut v3 = Options::default();
[01:06:15]      |                      ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2921:25
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2921 |         let reference = Options::default();
[01:06:15]      |                         ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:2922:24
[01:06:15]      |
[01:06:15]      |
[01:06:15] 2922 |         let mut opts = Options::default();
[01:06:15]      |                        ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:3059:25
[01:06:15]      |
[01:06:15]      |
[01:06:15] 3059 |         let reference = Options::default();
[01:06:15]      |                         ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:3060:24
[01:06:15]      |
[01:06:15]      |
[01:06:15] 3060 |         let mut opts = Options::default();
[01:06:15]      |                        ^^^^^^^ Use of undeclared type or module `Options`
[01:06:15] error[E0433]: failed to resolve. Use of undeclared type or module `Options`
[01:06:15]     --> librustc/session/config.rs:3193:23
[01:06:15]      |
[01:06:15]      |
[01:06:15] 3193 |         let options = Options::default();
[01:06:15]      |                       ^^^^^^^ Use of undeclared type or module `Options`
