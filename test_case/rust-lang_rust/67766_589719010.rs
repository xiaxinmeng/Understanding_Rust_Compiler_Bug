plain
2020-02-21T14:50:22.7700684Z ========================== Starting Command Output ===========================
2020-02-21T14:50:22.7703612Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7eac49d0-9e55-4d19-88a5-77e641d51f9c.sh
2020-02-21T14:50:22.7703901Z 
2020-02-21T14:50:22.7707038Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T14:50:22.7724988Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-02-21T14:50:22.7728123Z Task         : Get sources
2020-02-21T14:50:22.7728410Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T14:50:22.7728707Z Version      : 1.0.0
2020-02-21T14:50:22.7728894Z Author       : Microsoft
---
2020-02-21T14:50:23.7712307Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T14:50:23.7717510Z ##[command]git config gc.auto 0
2020-02-21T14:50:23.7721208Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T14:50:23.7724406Z ##[command]git config --get-all http.proxy
2020-02-21T14:50:23.7730852Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67766/merge:refs/remotes/pull/67766/merge
---
2020-02-21T15:55:38.6477621Z .................................................................................................... 1700/9695
2020-02-21T15:55:43.3926376Z .................................................................................................... 1800/9695
2020-02-21T15:55:55.0874690Z ..........................................i......................................................... 1900/9695
2020-02-21T15:56:02.8502968Z .................................................................................................... 2000/9695
2020-02-21T15:56:16.8952176Z ................................iiiii............................................................... 2100/9695
2020-02-21T15:56:26.7720025Z .................................................................................................... 2300/9695
2020-02-21T15:56:29.1917123Z .................................................................................................... 2400/9695
2020-02-21T15:56:33.6020856Z .................................................................................................... 2500/9695
2020-02-21T15:56:54.6148027Z .................................................................................................... 2600/9695
---
2020-02-21T15:59:38.3445072Z .......i............................................................................................ 5000/9695
2020-02-21T15:59:47.3097736Z .................................................................................................... 5100/9695
2020-02-21T15:59:51.9328159Z ..................................i................................................................. 5200/9695
2020-02-21T16:00:02.4153100Z .................................................................................................... 5300/9695
2020-02-21T16:00:08.4478618Z ..........ii.ii........i...i........................................................................ 5400/9695
2020-02-21T16:00:12.3022072Z .................................................i..........................F.F..F.................. 5500/9695
2020-02-21T16:00:28.3352733Z ..F................................................................................................. 5700/9695
2020-02-21T16:00:36.0816730Z ..i................................................................................................. 5800/9695
2020-02-21T16:00:41.6404560Z .................................................................................................... 5900/9695
2020-02-21T16:00:41.6404560Z .................................................................................................... 5900/9695
2020-02-21T16:00:51.8188747Z .............................................................................................ii...i. 6000/9695
2020-02-21T16:01:03.9838680Z .ii...........i..................................................................................... 6100/9695
2020-02-21T16:01:20.5337394Z .................................................................................................... 6300/9695
2020-02-21T16:01:27.1080601Z .................................................................................................... 6400/9695
2020-02-21T16:01:27.1080601Z .................................................................................................... 6400/9695
2020-02-21T16:01:49.6640743Z ........................i..ii....................................................................... 6500/9695
2020-02-21T16:02:09.3348670Z .................................................................................................... 6700/9695
2020-02-21T16:02:11.5292345Z ................i................................................................................... 6800/9695
2020-02-21T16:02:13.7054942Z .................................................................................................... 6900/9695
2020-02-21T16:02:16.1555908Z ......................................i............................................................. 7000/9695
---
2020-02-21T16:03:57.3061008Z .................................................................................................... 7700/9695
2020-02-21T16:04:02.3011187Z .................................................................................................... 7800/9695
2020-02-21T16:04:08.9016973Z ..................................................................................i................. 7900/9695
2020-02-21T16:04:18.3943628Z .................................................................................................... 8000/9695
2020-02-21T16:04:25.4437991Z ...............................iiiiiii.i............................................................ 8100/9695
2020-02-21T16:04:40.2124031Z .................................................................................................... 8300/9695
2020-02-21T16:04:49.0485728Z .................................................................................................... 8400/9695
2020-02-21T16:05:02.6977107Z .................................................................................................... 8500/9695
2020-02-21T16:05:09.8402311Z .................................................................................................... 8600/9695
---
2020-02-21T16:07:05.3824976Z 25 
2020-02-21T16:07:05.3825288Z 26 warning: unused variable: `unused_var`
2020-02-21T16:07:05.3825965Z 27   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:37:19
2020-02-21T16:07:05.3826360Z 
2020-02-21T16:07:05.3826812Z 36    |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
2020-02-21T16:07:05.3827260Z 37 
2020-02-21T16:07:05.3827760Z 38 warning: variable `hours_are_suns` is assigned to, but never used
2020-02-21T16:07:05.3851310Z +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:46:26
2020-02-21T16:07:05.3851884Z 40    |
2020-02-21T16:07:05.3852236Z 41 LL |                          mut hours_are_suns,
2020-02-21T16:07:05.3852937Z -    |                              ^^^^^^^^^^^^^^
2020-02-21T16:07:05.3852937Z -    |                              ^^^^^^^^^^^^^^
2020-02-21T16:07:05.3853396Z +    |                          ^^^^^^^^^^^^^^^^^^
2020-02-21T16:07:05.3853722Z 43    |
2020-02-21T16:07:05.3854188Z 44    = note: consider using `_hours_are_suns` instead
2020-02-21T16:07:05.3854714Z 
2020-02-21T16:07:05.3854931Z 
2020-02-21T16:07:05.3855246Z The actual stderr differed from the expected stderr.
2020-02-21T16:07:05.3856307Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern/issue-47390-unused-variable-in-struct-pattern.stderr
2020-02-21T16:07:05.3856307Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern/issue-47390-unused-variable-in-struct-pattern.stderr
2020-02-21T16:07:05.3857213Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T16:07:05.3858273Z To only update this specific test, also pass `--test-args lint/issue-47390-unused-variable-in-struct-pattern.rs`
2020-02-21T16:07:05.3859052Z error: 1 errors occurred comparing output.
2020-02-21T16:07:05.3859398Z status: exit code: 0
2020-02-21T16:07:05.3859398Z status: exit code: 0
2020-02-21T16:07:05.3861871Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern/auxiliary"
2020-02-21T16:07:05.3865204Z ------------------------------------------
2020-02-21T16:07:05.3865554Z 
2020-02-21T16:07:05.3866150Z ------------------------------------------
2020-02-21T16:07:05.3869466Z stderr:
2020-02-21T16:07:05.3869466Z stderr:
2020-02-21T16:07:05.3869967Z ------------------------------------------
2020-02-21T16:07:05.3870260Z warning: unused variable: `i_think_continually`
2020-02-21T16:07:05.3871188Z    |
2020-02-21T16:07:05.3871188Z    |
2020-02-21T16:07:05.3871490Z LL |     let i_think_continually = 2; //~ WARNING unused variable: `i_think_continually`
2020-02-21T16:07:05.3872000Z    |         ^^^^^^^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_i_think_continually`
2020-02-21T16:07:05.3872636Z note: the lint level is defined here
2020-02-21T16:07:05.3873207Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-02-21T16:07:05.3873492Z    |
2020-02-21T16:07:05.3873492Z    |
2020-02-21T16:07:05.3874153Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-02-21T16:07:05.3874754Z    = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`
2020-02-21T16:07:05.3874960Z 
2020-02-21T16:07:05.3875146Z warning: unused variable: `mut_unused_var`
2020-02-21T16:07:05.3875762Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:33:9
---
2020-02-21T16:07:05.3880055Z    |
2020-02-21T16:07:05.3880255Z LL |     let (mut var, unused_var) = (1, 2);
2020-02-21T16:07:05.3880643Z    |                   ^^^^^^^^^^ help: consider prefixing with an underscore: `_unused_var`
2020-02-21T16:07:05.3880952Z 
2020-02-21T16:07:05.3881158Z warning: unused variable: `corridors_of_light`
2020-02-21T16:07:05.3882064Z    |
2020-02-21T16:07:05.3882064Z    |
2020-02-21T16:07:05.3882384Z LL |     if let SoulHistory { corridors_of_light, //~ WARNING unused variable: `corridors_of_light`
2020-02-21T16:07:05.3882913Z    |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
2020-02-21T16:07:05.3883232Z 
2020-02-21T16:07:05.3883484Z warning: variable `hours_are_suns` is assigned to, but never used
2020-02-21T16:07:05.3884416Z    |
2020-02-21T16:07:05.3884416Z    |
2020-02-21T16:07:05.3884723Z LL |                          mut hours_are_suns, //~ WARNING `hours_are_suns` is assigned to, but
2020-02-21T16:07:05.3885291Z    |
2020-02-21T16:07:05.3885291Z    |
2020-02-21T16:07:05.3885635Z    = note: consider using `_hours_are_suns` instead
2020-02-21T16:07:05.3885943Z 
2020-02-21T16:07:05.3889538Z warning: value assigned to `hours_are_suns` is never read
2020-02-21T16:07:05.3895494Z    |
2020-02-21T16:07:05.3895494Z    |
2020-02-21T16:07:05.3895722Z LL |         hours_are_suns = false; //~ WARNING unused_assignments
2020-02-21T16:07:05.3896150Z    |
2020-02-21T16:07:05.3896334Z note: the lint level is defined here
2020-02-21T16:07:05.3897033Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-02-21T16:07:05.3897322Z    |
2020-02-21T16:07:05.3897322Z    |
2020-02-21T16:07:05.3897723Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-02-21T16:07:05.3898235Z    = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
2020-02-21T16:07:05.3898550Z    = help: maybe it is overwritten before being read?
2020-02-21T16:07:05.3898735Z 
2020-02-21T16:07:05.3898920Z warning: unused variable: `fire`
2020-02-21T16:07:05.3898920Z warning: unused variable: `fire`
2020-02-21T16:07:05.3899462Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:52:32
2020-02-21T16:07:05.3899742Z    |
2020-02-21T16:07:05.3900036Z LL |     let LovelyAmbition { lips, fire } = the_spirit; //~ WARNING unused variable: `fire`
2020-02-21T16:07:05.3900517Z    |                                ^^^^ help: try ignoring the field: `fire: _`
2020-02-21T16:07:05.3901427Z warning: unused variable: `case`
2020-02-21T16:07:05.3903660Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:61:23
2020-02-21T16:07:05.3904075Z    |
2020-02-21T16:07:05.3904075Z    |
2020-02-21T16:07:05.3904330Z LL |         Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3904738Z    |                       ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3905143Z warning: unused variable: `case`
2020-02-21T16:07:05.3905711Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:66:24
2020-02-21T16:07:05.3905991Z    |
2020-02-21T16:07:05.3905991Z    |
2020-02-21T16:07:05.3906581Z LL |         &Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3907124Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3907560Z warning: unused variable: `case`
2020-02-21T16:07:05.3908206Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:71:27
2020-02-21T16:07:05.3908524Z    |
2020-02-21T16:07:05.3908524Z    |
2020-02-21T16:07:05.3908812Z LL |         box Large::Suit { case } => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3919635Z    |                           ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3920133Z warning: unused variable: `case`
2020-02-21T16:07:05.3921222Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:76:24
2020-02-21T16:07:05.3921523Z    |
2020-02-21T16:07:05.3921523Z    |
2020-02-21T16:07:05.3921799Z LL |         (Large::Suit { case },) => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3922993Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3924258Z warning: unused variable: `case`
2020-02-21T16:07:05.3925042Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:81:24
2020-02-21T16:07:05.3925349Z    |
2020-02-21T16:07:05.3925349Z    |
2020-02-21T16:07:05.3925668Z LL |         [Large::Suit { case }] => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3926097Z    |                        ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3926549Z warning: unused variable: `case`
2020-02-21T16:07:05.3927148Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:86:29
2020-02-21T16:07:05.3927449Z    |
2020-02-21T16:07:05.3927449Z    |
2020-02-21T16:07:05.3927761Z LL |         Tuple(Large::Suit { case }, ()) => {} //~ WARNING unused variable: `case`
2020-02-21T16:07:05.3928215Z    |                             ^^^^ help: try ignoring the field: `case: _`
2020-02-21T16:07:05.3929245Z warning: variable does not need to be mutable
2020-02-21T16:07:05.3930796Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:33:9
2020-02-21T16:07:05.3931101Z    |
2020-02-21T16:07:05.3931280Z LL |     let mut mut_unused_var = 1;
2020-02-21T16:07:05.3931280Z LL |     let mut mut_unused_var = 1;
2020-02-21T16:07:05.3931717Z    |         ----^^^^^^^^^^^^^^
2020-02-21T16:07:05.3931912Z    |         |
2020-02-21T16:07:05.3932125Z    |         help: remove this `mut`
2020-02-21T16:07:05.3932339Z    |
2020-02-21T16:07:05.3932526Z note: the lint level is defined here
2020-02-21T16:07:05.3933312Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:5:9
2020-02-21T16:07:05.3933638Z    |
2020-02-21T16:07:05.3934108Z LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
2020-02-21T16:07:05.3934649Z    = note: `#[warn(unused_mut)]` implied by `#[warn(unused)]`
2020-02-21T16:07:05.3934861Z 
2020-02-21T16:07:05.3935058Z warning: variable does not need to be mutable
2020-02-21T16:07:05.3935646Z   --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:37:10
---
2020-02-21T16:07:05.3939374Z 1 error: unused variable: `field`
2020-02-21T16:07:05.3939847Z -   --> $DIR/issue-54180-unused-ref-field.rs:20:26
2020-02-21T16:07:05.3940369Z +   --> $DIR/issue-54180-unused-ref-field.rs:20:22
2020-02-21T16:07:05.3940603Z 3    |
2020-02-21T16:07:05.3940820Z 4 LL |         E::Variant { ref field } => (),
2020-02-21T16:07:05.3941655Z -    |                      |
2020-02-21T16:07:05.3941655Z -    |                      |
2020-02-21T16:07:05.3942162Z -    |                      help: try ignoring the field: `field: _`
2020-02-21T16:07:05.3942624Z +    |                      ^^^^^^^^^ help: try ignoring the field: `field: _`
2020-02-21T16:07:05.3943119Z 9 note: the lint level is defined here
2020-02-21T16:07:05.3943601Z 10   --> $DIR/issue-54180-unused-ref-field.rs:3:9
2020-02-21T16:07:05.3943805Z 
2020-02-21T16:07:05.3943805Z 
2020-02-21T16:07:05.3944119Z 20    |                                             ^ help: try ignoring the field: `x: _`
2020-02-21T16:07:05.3944641Z 22 error: unused variable: `f1`
2020-02-21T16:07:05.3945106Z -   --> $DIR/issue-54180-unused-ref-field.rs:26:17
2020-02-21T16:07:05.3945607Z +   --> $DIR/issue-54180-unused-ref-field.rs:26:13
2020-02-21T16:07:05.3945961Z 24    |
2020-02-21T16:07:05.3945961Z 24    |
2020-02-21T16:07:05.3946140Z 25 LL |     let S { ref f1 } = s;
2020-02-21T16:07:05.3946836Z -    |             |
2020-02-21T16:07:05.3946836Z -    |             |
2020-02-21T16:07:05.3947277Z -    |             help: try ignoring the field: `f1: _`
2020-02-21T16:07:05.3947650Z +    |             ^^^^^^ help: try ignoring the field: `f1: _`
2020-02-21T16:07:05.3948227Z 30 error: unused variable: `x`
2020-02-21T16:07:05.3948698Z -   --> $DIR/issue-54180-unused-ref-field.rs:32:28
2020-02-21T16:07:05.3949216Z +   --> $DIR/issue-54180-unused-ref-field.rs:32:20
2020-02-21T16:07:05.3949629Z 32    |
2020-02-21T16:07:05.3949629Z 32    |
2020-02-21T16:07:05.3949840Z 33 LL |         Point { y, ref mut x } => y,
2020-02-21T16:07:05.3950694Z -    |                    |
2020-02-21T16:07:05.3950694Z -    |                    |
2020-02-21T16:07:05.3951179Z -    |                    help: try ignoring the field: `x: _`
2020-02-21T16:07:05.3951605Z +    |                    ^^^^^^^^^ help: try ignoring the field: `x: _`
2020-02-21T16:07:05.3952085Z 38 error: aborting due to 4 previous errors
2020-02-21T16:07:05.3952282Z 39 
2020-02-21T16:07:05.3952401Z 
2020-02-21T16:07:05.3952497Z 
2020-02-21T16:07:05.3952497Z 
2020-02-21T16:07:05.3952704Z The actual stderr differed from the expected stderr.
2020-02-21T16:07:05.3953442Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-54180-unused-ref-field/issue-54180-unused-ref-field.stderr
2020-02-21T16:07:05.3954554Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T16:07:05.3955186Z To only update this specific test, also pass `--test-args lint/issue-54180-unused-ref-field.rs`
2020-02-21T16:07:05.3955661Z error: 1 errors occurred comparing output.
2020-02-21T16:07:05.3955896Z status: exit code: 1
2020-02-21T16:07:05.3955896Z status: exit code: 1
2020-02-21T16:07:05.3958238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-54180-unused-ref-field" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-54180-unused-ref-field/auxiliary"
2020-02-21T16:07:05.3959997Z ------------------------------------------
2020-02-21T16:07:05.3960167Z 
2020-02-21T16:07:05.3960510Z ------------------------------------------
2020-02-21T16:07:05.3960720Z stderr:
2020-02-21T16:07:05.3960720Z stderr:
2020-02-21T16:07:05.3961198Z ------------------------------------------
2020-02-21T16:07:05.3961442Z error: unused variable: `field`
2020-02-21T16:07:05.3961946Z   --> /checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs:20:22
2020-02-21T16:07:05.3962229Z    |
2020-02-21T16:07:05.3962486Z LL |         E::Variant { ref field } => (), //~ ERROR unused variable
2020-02-21T16:07:05.3963009Z    |                      ^^^^^^^^^ help: try ignoring the field: `field: _`
2020-02-21T16:07:05.3963466Z note: the lint level is defined here
2020-02-21T16:07:05.3963970Z   --> /checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs:3:9
2020-02-21T16:07:05.3964359Z    |
2020-02-21T16:07:05.3964523Z LL | #![deny(unused)]
2020-02-21T16:07:05.3964523Z LL | #![deny(unused)]
2020-02-21T16:07:05.3964710Z    |         ^^^^^^
2020-02-21T16:07:05.3977549Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-02-21T16:07:05.3977778Z 
2020-02-21T16:07:05.3977943Z error: unused variable: `x`
2020-02-21T16:07:05.3978622Z   --> /checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs:29:45
2020-02-21T16:07:05.3979521Z    |
2020-02-21T16:07:05.3979806Z LL |     let _: i32 = points.iter().map(|Point { x, y }| y).sum(); //~ ERROR unused variable
2020-02-21T16:07:05.3983191Z    |                                             ^ help: try ignoring the field: `x: _`
2020-02-21T16:07:05.3983680Z error: unused variable: `f1`
2020-02-21T16:07:05.3984375Z   --> /checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs:26:13
2020-02-21T16:07:05.3984665Z    |
2020-02-21T16:07:05.3984665Z    |
2020-02-21T16:07:05.3984901Z LL |     let S { ref f1 } = s; //~ ERROR unused variable
2020-02-21T16:07:05.3985254Z    |             ^^^^^^ help: try ignoring the field: `f1: _`
2020-02-21T16:07:05.3985884Z error: unused variable: `x`
2020-02-21T16:07:05.3986471Z   --> /checkout/src/test/ui/lint/issue-54180-unused-ref-field.rs:32:20
2020-02-21T16:07:05.3986753Z    |
2020-02-21T16:07:05.3986753Z    |
2020-02-21T16:07:05.3986995Z LL |         Point { y, ref mut x } => y, //~ ERROR unused variable
2020-02-21T16:07:05.3987382Z    |                    ^^^^^^^^^ help: try ignoring the field: `x: _`
2020-02-21T16:07:05.3987835Z error: aborting due to 4 previous errors
2020-02-21T16:07:05.3988006Z 
2020-02-21T16:07:05.3988103Z 
2020-02-21T16:07:05.3988606Z ------------------------------------------
---
2020-02-21T16:07:05.3989895Z 
2020-02-21T16:07:05.3990060Z 17     use MyEnum::*;
2020-02-21T16:07:05.3990223Z 18 
2020-02-21T16:07:05.3990369Z 19     match x {
2020-02-21T16:07:05.3990987Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-02-21T16:07:05.3991358Z +         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-02-21T16:07:05.3991850Z 22         }
2020-02-21T16:07:05.3991998Z 23     }
2020-02-21T16:07:05.3992109Z 
2020-02-21T16:07:05.3992284Z 27     use MyEnum::*;
2020-02-21T16:07:05.3992284Z 27     use MyEnum::*;
2020-02-21T16:07:05.3992449Z 28 
2020-02-21T16:07:05.3992596Z 29     match x {
2020-02-21T16:07:05.3993079Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-02-21T16:07:05.3993479Z +         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-02-21T16:07:05.3994101Z 32         }
2020-02-21T16:07:05.3994386Z 33     }
2020-02-21T16:07:05.3994645Z 
2020-02-21T16:07:05.3994812Z 37     use MyEnum::*;
2020-02-21T16:07:05.3994812Z 37     use MyEnum::*;
2020-02-21T16:07:05.3995140Z 38 
2020-02-21T16:07:05.3995303Z 39     match x {
2020-02-21T16:07:05.3995954Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-02-21T16:07:05.3996354Z +         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-02-21T16:07:05.3996860Z 42         }
2020-02-21T16:07:05.3997000Z 43 
2020-02-21T16:07:05.3997118Z 
2020-02-21T16:07:05.3997279Z 49     use MyEnum::*;
2020-02-21T16:07:05.3997279Z 49     use MyEnum::*;
2020-02-21T16:07:05.3997445Z 50 
2020-02-21T16:07:05.3997606Z 51     match x {
2020-02-21T16:07:05.3998275Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-02-21T16:07:05.3998676Z +         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-02-21T16:07:05.3999185Z 54         }
2020-02-21T16:07:05.3999325Z 55 
2020-02-21T16:07:05.3999427Z 
2020-02-21T16:07:05.3999559Z 59 
2020-02-21T16:07:05.3999559Z 59 
2020-02-21T16:07:05.3999767Z 60 pub fn mixed_no_ref(x: MixedEnum) {
2020-02-21T16:07:05.3999989Z 61     match x {
2020-02-21T16:07:05.4000498Z -         MixedEnum::A { i: _ } | MixedEnum::B(_i) => {
2020-02-21T16:07:05.4000846Z +         MixedEnum::A { i } | MixedEnum::B(i) => {
2020-02-21T16:07:05.4001382Z 63             println!("match");
2020-02-21T16:07:05.4001735Z 65     }
2020-02-21T16:07:05.4001842Z 
2020-02-21T16:07:05.4001956Z 67 
2020-02-21T16:07:05.4001956Z 67 
2020-02-21T16:07:05.4002168Z 68 pub fn mixed_with_ref(x: MixedEnum) {
2020-02-21T16:07:05.4002384Z 69     match x {
2020-02-21T16:07:05.4002850Z -         MixedEnum::A { i: _ } | MixedEnum::B(_i) => {
2020-02-21T16:07:05.4003202Z +         MixedEnum::A { ref i } | MixedEnum::B(ref i) => {
2020-02-21T16:07:05.4003506Z 71             println!("match");
2020-02-21T16:07:05.4003956Z 73     }
2020-02-21T16:07:05.4004079Z 
2020-02-21T16:07:05.4004199Z 79 
2020-02-21T16:07:05.4004199Z 79 
2020-02-21T16:07:05.4004440Z 80     inner_no_ref(Some(MyEnum::A { i: 1, j: 2 }));
2020-02-21T16:07:05.4004801Z 81     inner_with_ref(Some(MyEnum::A { i: 1, j: 2 }));
2020-02-21T16:07:05.4005041Z + 
2020-02-21T16:07:05.4005241Z 82     mixed_no_ref(MixedEnum::B(5));
2020-02-21T16:07:05.4005519Z 83     mixed_with_ref(MixedEnum::B(5));
2020-02-21T16:07:05.4005855Z 
2020-02-21T16:07:05.4005953Z 
2020-02-21T16:07:05.4006156Z The actual fixed differed from the expected fixed.
2020-02-21T16:07:05.4006156Z The actual fixed differed from the expected fixed.
2020-02-21T16:07:05.4006949Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/issue-67691-unused-field-in-or-pattern.fixed
2020-02-21T16:07:05.4007789Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T16:07:05.4008508Z To only update this specific test, also pass `--test-args lint/issue-67691-unused-field-in-or-pattern.rs`
2020-02-21T16:07:05.4009126Z error: 1 errors occurred comparing output.
2020-02-21T16:07:05.4009352Z status: exit code: 1
2020-02-21T16:07:05.4009352Z status: exit code: 1
2020-02-21T16:07:05.4011825Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/auxiliary"
2020-02-21T16:07:05.4013771Z ------------------------------------------
2020-02-21T16:07:05.4014047Z 
2020-02-21T16:07:05.4014401Z ------------------------------------------
2020-02-21T16:07:05.4014724Z stderr:
2020-02-21T16:07:05.4014724Z stderr:
2020-02-21T16:07:05.4015088Z ------------------------------------------
2020-02-21T16:07:05.4015472Z error: unused variable: `j`
2020-02-21T16:07:05.4016069Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:20:16
2020-02-21T16:07:05.4016447Z    |
2020-02-21T16:07:05.4016709Z LL |         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-02-21T16:07:05.4017182Z    |
2020-02-21T16:07:05.4017368Z note: the lint level is defined here
2020-02-21T16:07:05.4017960Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:4:9
2020-02-21T16:07:05.4018353Z    |
2020-02-21T16:07:05.4018353Z    |
2020-02-21T16:07:05.4018509Z LL | #![deny(unused)]
2020-02-21T16:07:05.4018702Z    |         ^^^^^^
2020-02-21T16:07:05.4019087Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-02-21T16:07:05.4019355Z help: try ignoring the field
2020-02-21T16:07:05.4019530Z    |
2020-02-21T16:07:05.4019782Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-02-21T16:07:05.4020265Z 
2020-02-21T16:07:05.4020430Z error: unused variable: `j`
2020-02-21T16:07:05.4021066Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:30:16
2020-02-21T16:07:05.4021342Z    |
2020-02-21T16:07:05.4021342Z    |
2020-02-21T16:07:05.4021604Z LL |         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-02-21T16:07:05.4022099Z    |
2020-02-21T16:07:05.4022280Z help: try ignoring the field
2020-02-21T16:07:05.4022444Z    |
2020-02-21T16:07:05.4022444Z    |
2020-02-21T16:07:05.4022704Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-02-21T16:07:05.4023196Z 
2020-02-21T16:07:05.4023479Z error: unused variable: `j`
2020-02-21T16:07:05.4024042Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:40:21
2020-02-21T16:07:05.4024328Z    |
2020-02-21T16:07:05.4024328Z    |
2020-02-21T16:07:05.4024584Z LL |         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-02-21T16:07:05.4025100Z    |
2020-02-21T16:07:05.4025271Z help: try ignoring the field
2020-02-21T16:07:05.4025442Z    |
2020-02-21T16:07:05.4025442Z    |
2020-02-21T16:07:05.4025739Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-02-21T16:07:05.4026268Z 
2020-02-21T16:07:05.4026454Z error: unused variable: `j`
2020-02-21T16:07:05.4027002Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:52:21
2020-02-21T16:07:05.4027288Z    |
2020-02-21T16:07:05.4027288Z    |
2020-02-21T16:07:05.4027569Z LL |         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-02-21T16:07:05.4028111Z    |
2020-02-21T16:07:05.4028285Z help: try ignoring the field
2020-02-21T16:07:05.4028470Z    |
2020-02-21T16:07:05.4028470Z    |
2020-02-21T16:07:05.4028757Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-02-21T16:07:05.4029304Z 
2020-02-21T16:07:05.4029669Z error: unused variable: `i`
2020-02-21T16:07:05.4030267Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:62:24
2020-02-21T16:07:05.4030570Z    |
2020-02-21T16:07:05.4030570Z    |
2020-02-21T16:07:05.4030810Z LL |         MixedEnum::A { i } | MixedEnum::B(i) => {
2020-02-21T16:07:05.4031341Z    |
2020-02-21T16:07:05.4031512Z help: try ignoring the field
2020-02-21T16:07:05.4031685Z    |
2020-02-21T16:07:05.4031685Z    |
2020-02-21T16:07:05.4031933Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_i) => {
2020-02-21T16:07:05.4032465Z 
2020-02-21T16:07:05.4032636Z error: unused variable: `i`
2020-02-21T16:07:05.4033253Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:70:24
2020-02-21T16:07:05.4033641Z    |
2020-02-21T16:07:05.4033641Z    |
2020-02-21T16:07:05.4033908Z LL |         MixedEnum::A { ref i } | MixedEnum::B(ref i) => {
2020-02-21T16:07:05.4034604Z    |
2020-02-21T16:07:05.4034775Z help: try ignoring the field
2020-02-21T16:07:05.4034947Z    |
2020-02-21T16:07:05.4034947Z    |
2020-02-21T16:07:05.4035211Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_i) => {
2020-02-21T16:07:05.4035729Z 
2020-02-21T16:07:05.4035930Z error: aborting due to 6 previous errors
2020-02-21T16:07:05.4036102Z 
2020-02-21T16:07:05.4036198Z 
---
2020-02-21T16:07:05.4043711Z 29 
2020-02-21T16:07:05.4043826Z 
2020-02-21T16:07:05.4043923Z 
2020-02-21T16:07:05.4044128Z The actual stderr differed from the expected stderr.
2020-02-21T16:07:05.4045904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/liveness-dead.stderr
2020-02-21T16:07:05.4046565Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T16:07:05.4047156Z To only update this specific test, also pass `--test-args liveness/liveness-dead.rs`
2020-02-21T16:07:05.4047613Z error: 1 errors occurred comparing output.
2020-02-21T16:07:05.4047848Z status: exit code: 1
2020-02-21T16:07:05.4047848Z status: exit code: 1
2020-02-21T16:07:05.4050294Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-dead.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-dead/auxiliary"
2020-02-21T16:07:05.4052004Z ------------------------------------------
2020-02-21T16:07:05.4052176Z 
2020-02-21T16:07:05.4052524Z ------------------------------------------
2020-02-21T16:07:05.4052720Z stderr:
2020-02-21T16:07:05.4052720Z stderr:
2020-02-21T16:07:05.4053088Z ------------------------------------------
2020-02-21T16:07:05.4053346Z error: value assigned to `x` is never read
2020-02-21T16:07:05.4053816Z   --> /checkout/src/test/ui/liveness/liveness-dead.rs:9:9
2020-02-21T16:07:05.4054184Z    |
2020-02-21T16:07:05.4054589Z LL |     let mut x: isize = 3; //~ ERROR: value assigned to `x` is never read
2020-02-21T16:07:05.4055133Z    |
2020-02-21T16:07:05.4055307Z note: the lint level is defined here
2020-02-21T16:07:05.4055793Z   --> /checkout/src/test/ui/liveness/liveness-dead.rs:2:9
2020-02-21T16:07:05.4056027Z    |
---
2020-02-21T16:07:05.4058746Z 
2020-02-21T16:07:05.4058944Z error: value passed to `x` is never read
2020-02-21T16:07:05.4059406Z   --> /checkout/src/test/ui/liveness/liveness-dead.rs:20:7
2020-02-21T16:07:05.4059625Z    |
2020-02-21T16:07:05.4059897Z LL | fn f4(mut x: i32) { //~ ERROR: value passed to `x` is never read
2020-02-21T16:07:05.4060304Z    |
2020-02-21T16:07:05.4060509Z    = help: maybe it is overwritten before being read?
2020-02-21T16:07:05.4060706Z 
2020-02-21T16:07:05.4061008Z error: value assigned to `x` is never read
---
2020-02-21T16:07:05.4068228Z 53 
2020-02-21T16:07:05.4068336Z 
2020-02-21T16:07:05.4068557Z 65    = help: maybe it is overwritten before being read?
2020-02-21T16:07:05.4069259Z 66 
2020-02-21T16:07:05.4069463Z 67 error: variable `z` is assigned to, but never used
2020-02-21T16:07:05.4070495Z +   --> $DIR/liveness-unused.rs:37:9
2020-02-21T16:07:05.4096174Z 69    |
2020-02-21T16:07:05.4096174Z 69    |
2020-02-21T16:07:05.4096352Z 70 LL |     let mut z = 3;
2020-02-21T16:07:05.4097101Z +    |         ^^^^^
2020-02-21T16:07:05.4097263Z 72    |
2020-02-21T16:07:05.4097469Z 73    = note: consider using `_z` instead
2020-02-21T16:07:05.4097683Z 74 
2020-02-21T16:07:05.4097683Z 74 
2020-02-21T16:07:05.4097789Z 
2020-02-21T16:07:05.4097886Z 
2020-02-21T16:07:05.4098092Z The actual stderr differed from the expected stderr.
2020-02-21T16:07:05.4098803Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/liveness-unused.stderr
2020-02-21T16:07:05.4099927Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T16:07:05.4100620Z To only update this specific test, also pass `--test-args liveness/liveness-unused.rs`
2020-02-21T16:07:05.4101200Z error: 1 errors occurred comparing output.
2020-02-21T16:07:05.4101434Z status: exit code: 1
2020-02-21T16:07:05.4101434Z status: exit code: 1
2020-02-21T16:07:05.4103464Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-unused/auxiliary"
2020-02-21T16:07:05.4105089Z ------------------------------------------
2020-02-21T16:07:05.4105270Z 
2020-02-21T16:07:05.4105631Z ------------------------------------------
2020-02-21T16:07:05.4105848Z stderr:
2020-02-21T16:07:05.4105848Z stderr:
2020-02-21T16:07:05.4106317Z ------------------------------------------
2020-02-21T16:07:05.4106673Z warning: unreachable statement
2020-02-21T16:07:05.4107134Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:92:9
2020-02-21T16:07:05.4107457Z    |
2020-02-21T16:07:05.4107599Z LL |         continue;
2020-02-21T16:07:05.4108027Z    |         -------- any code following this expression is unreachable
2020-02-21T16:07:05.4108334Z LL |         drop(*x as i32); //~ WARNING unreachable statement
2020-02-21T16:07:05.4108607Z    |         ^^^^^^^^^^^^^^^^ unreachable statement
2020-02-21T16:07:05.4108966Z note: the lint level is defined here
2020-02-21T16:07:05.4109390Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:1:9
2020-02-21T16:07:05.4109742Z    |
2020-02-21T16:07:05.4109897Z LL | #![warn(unused)]
2020-02-21T16:07:05.4109897Z LL | #![warn(unused)]
2020-02-21T16:07:05.4110070Z    |         ^^^^^^
2020-02-21T16:07:05.4110329Z    = note: `#[warn(unreachable_code)]` implied by `#[warn(unused)]`
2020-02-21T16:07:05.4110541Z 
2020-02-21T16:07:05.4110702Z error: unused variable: `x`
2020-02-21T16:07:05.4111404Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:8:7
2020-02-21T16:07:05.4111645Z    |
2020-02-21T16:07:05.4111826Z LL | fn f1(x: isize) {
2020-02-21T16:07:05.4112124Z    |       ^ help: consider prefixing with an underscore: `_x`
2020-02-21T16:07:05.4112579Z note: the lint level is defined here
2020-02-21T16:07:05.4113063Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:2:9
2020-02-21T16:07:05.4113299Z    |
2020-02-21T16:07:05.4113475Z LL | #![deny(unused_variables)]
2020-02-21T16:07:05.4113475Z LL | #![deny(unused_variables)]
2020-02-21T16:07:05.4113710Z    |         ^^^^^^^^^^^^^^^^
2020-02-21T16:07:05.4113857Z 
2020-02-21T16:07:05.4114030Z error: unused variable: `x`
2020-02-21T16:07:05.4114529Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:12:8
2020-02-21T16:07:05.4114770Z    |
2020-02-21T16:07:05.4114952Z LL | fn f1b(x: &mut isize) {
2020-02-21T16:07:05.4115278Z    |        ^ help: consider prefixing with an underscore: `_x`
2020-02-21T16:07:05.4115798Z error: unused variable: `x`
2020-02-21T16:07:05.4116260Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:20:9
2020-02-21T16:07:05.4116623Z    |
2020-02-21T16:07:05.4116879Z LL |     let x: isize;
---
2020-02-21T16:07:05.4123380Z LL | #![deny(unused_assignments)]
2020-02-21T16:07:05.4123606Z    |         ^^^^^^^^^^^^^^^^^^
2020-02-21T16:07:05.4123881Z    = help: maybe it is overwritten before being read?
2020-02-21T16:07:05.4124095Z 
2020-02-21T16:07:05.4124300Z error: variable `z` is assigned to, but never used
2020-02-21T16:07:05.4125064Z    |
2020-02-21T16:07:05.4125064Z    |
2020-02-21T16:07:05.4125226Z LL |     let mut z = 3;
2020-02-21T16:07:05.4125670Z    |
2020-02-21T16:07:05.4125870Z    = note: consider using `_z` instead
2020-02-21T16:07:05.4126152Z 
2020-02-21T16:07:05.4126313Z error: unused variable: `i`
---
2020-02-21T16:07:05.4131312Z 
2020-02-21T16:07:05.4131498Z error: unused variable: `x`
2020-02-21T16:07:05.4132040Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:89:13
2020-02-21T16:07:05.4132280Z    |
2020-02-21T16:07:05.4132494Z LL |     for (_, x) in [1, 2, 3].iter().enumerate() {
2020-02-21T16:07:05.4132868Z    |             ^ help: consider prefixing with an underscore: `_x`
2020-02-21T16:07:05.4133327Z error: variable `x` is assigned to, but never used
2020-02-21T16:07:05.4133866Z   --> /checkout/src/test/ui/liveness/liveness-unused.rs:112:9
2020-02-21T16:07:05.4134107Z    |
2020-02-21T16:07:05.4134254Z LL |     let x;
---
2020-02-21T16:07:05.4142508Z test result: FAILED. 9636 passed; 5 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-21T16:07:05.4142783Z 
2020-02-21T16:07:05.4142882Z 
2020-02-21T16:07:05.4142977Z 
2020-02-21T16:07:05.4146777Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-21T16:07:05.4149104Z 
2020-02-21T16:07:05.4149203Z 
2020-02-21T16:07:05.4149407Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-21T16:07:05.4149691Z Build completed unsuccessfully in 1:09:12
2020-02-21T16:07:05.4149691Z Build completed unsuccessfully in 1:09:12
2020-02-21T16:07:05.4150222Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T16:07:05.4150720Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-21T16:07:05.4151290Z == clock drift check ==
2020-02-21T16:07:05.4151552Z   local time: Fri Feb 21 16:07:05 UTC 2020
2020-02-21T16:07:05.5624300Z   network time: Fri, 21 Feb 2020 16:07:05 GMT
2020-02-21T16:07:05.5624725Z == end clock drift check ==
2020-02-21T16:07:05.9939608Z 
2020-02-21T16:07:06.0016604Z ##[error]Bash exited with code '1'.
2020-02-21T16:07:06.0032290Z ##[section]Finishing: Run build
2020-02-21T16:07:06.0084700Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-02-21T16:07:06.0090124Z Task         : Get sources
2020-02-21T16:07:06.0090476Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T16:07:06.0090790Z Version      : 1.0.0
2020-02-21T16:07:06.0091022Z Author       : Microsoft
2020-02-21T16:07:06.0091022Z Author       : Microsoft
2020-02-21T16:07:06.0091373Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T16:07:06.0091777Z ==============================================================================
2020-02-21T16:07:06.3571932Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T16:07:06.3623824Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-02-21T16:07:06.3724082Z Cleaning up task key
2020-02-21T16:07:06.3725375Z Start cleaning up orphan processes.
2020-02-21T16:07:06.3934862Z Terminate orphan process: pid (3648) (python)
2020-02-21T16:07:06.4186333Z ##[section]Finishing: Finalize Job
