plain
2020-03-23T23:06:14.2290631Z ========================== Starting Command Output ===========================
2020-03-23T23:06:14.2293250Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fa677d74-30bd-4093-8851-0d0ceb88ef56.sh
2020-03-23T23:06:14.2293524Z 
2020-03-23T23:06:14.2297457Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T23:06:14.2316123Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-23T23:06:14.2319459Z Task         : Get sources
2020-03-23T23:06:14.2319792Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T23:06:14.2320105Z Version      : 1.0.0
2020-03-23T23:06:14.2320313Z Author       : Microsoft
---
2020-03-23T23:06:15.2292042Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T23:06:15.2298193Z ##[command]git config gc.auto 0
2020-03-23T23:06:15.2302330Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T23:06:15.2306407Z ##[command]git config --get-all http.proxy
2020-03-23T23:06:15.2313982Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69916/merge:refs/remotes/pull/69916/merge
---
2020-03-23T23:14:21.9408230Z configure: build.locked-deps    := True
2020-03-23T23:14:21.9408498Z configure: llvm.ccache          := sccache
2020-03-23T23:14:21.9410062Z configure: build.cargo-native-static := True
2020-03-23T23:14:21.9410524Z configure: dist.missing-tools   := True
2020-03-23T23:14:21.9411287Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T23:14:21.9411807Z configure: writing `config.toml` in current directory
2020-03-23T23:14:21.9412030Z configure: 
2020-03-23T23:14:21.9412392Z configure: run `python /checkout/x.py --help`
2020-03-23T23:14:21.9412613Z configure: 
---
2020-03-23T23:21:11.6347166Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 632:
2020-03-23T23:21:11.6347465Z      Ok(())
2020-03-23T23:21:11.6347612Z  }
2020-03-23T23:21:11.6347750Z  
2020-03-23T23:21:11.6348762Z -fn write_allocation_endline(
2020-03-23T23:21:11.6349177Z -    w: &mut dyn Write,
2020-03-23T23:21:11.6349642Z -    ascii: &str,
2020-03-23T23:21:11.6349967Z -) -> io::Result<()> {
2020-03-23T23:21:11.6350473Z +fn write_allocation_endline(w: &mut dyn Write, ascii: &str) -> io::Result<()> {
2020-03-23T23:21:11.6351171Z      for _ in 0..(BYTES_PER_LINE.bytes() as usize - ascii.chars().count()) {
2020-03-23T23:21:11.6351449Z          write!(w, "   ")?;
2020-03-23T23:21:11.6351826Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 727:
2020-03-23T23:21:11.6351826Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 727:
2020-03-23T23:21:11.6352189Z                  if overflow_width > remainder_width && overflow_width >= target.len() {
2020-03-23T23:21:11.6352580Z                      // The case where the relocation fits into the part in the next line
2020-03-23T23:21:11.6353161Z                      write!(w, "╾{0:─^1$}", "", remainder_width)?;
2020-03-23T23:21:11.6353661Z -                    line_start = write_allocation_newline(
2020-03-23T23:21:11.6354379Z -                        w,
2020-03-23T23:21:11.6355212Z -                        &ascii,
2020-03-23T23:21:11.6355569Z -                        pos_width,
2020-03-23T23:21:11.6355885Z -                        prefix,
2020-03-23T23:21:11.6356410Z -                    )?;
2020-03-23T23:21:11.6356410Z -                    )?;
2020-03-23T23:21:11.6356612Z +                    line_start =
2020-03-23T23:21:11.6356919Z +                        write_allocation_newline(w, line_start, &ascii, pos_width, prefix)?;
2020-03-23T23:21:11.6357243Z                      ascii.clear();
2020-03-23T23:21:11.6357720Z                      write!(w, "{0:─^1$}╼", target, overflow_width)?;
2020-03-23T23:21:11.6358256Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 740:
2020-03-23T23:21:11.6358565Z                      oversized_ptr(&mut target, remainder_width);
2020-03-23T23:21:11.6358565Z                      oversized_ptr(&mut target, remainder_width);
2020-03-23T23:21:11.6359083Z                      write!(w, "╾{0:─^1$}", target, remainder_width)?;
2020-03-23T23:21:11.6359606Z -                    line_start = write_allocation_newline(
2020-03-23T23:21:11.6359990Z -                        w,
2020-03-23T23:21:11.6360721Z -                        &ascii,
2020-03-23T23:21:11.6361091Z -                        pos_width,
2020-03-23T23:21:11.6361449Z -                        prefix,
2020-03-23T23:21:11.6361796Z -                    )?;
2020-03-23T23:21:11.6361796Z -                    )?;
2020-03-23T23:21:11.6361995Z +                    line_start =
2020-03-23T23:21:11.6362300Z +                        write_allocation_newline(w, line_start, &ascii, pos_width, prefix)?;
2020-03-23T23:21:11.6362866Z                      write!(w, "{0:─^1$}╼", "", overflow_width)?;
2020-03-23T23:21:11.6363150Z                      ascii.clear();
2020-03-23T23:21:11.6363593Z Diff in /checkout/src/librustc_mir/util/pretty.rs at line 788:
2020-03-23T23:21:11.6363811Z          }
2020-03-23T23:21:11.6364074Z          // Print a new line header if the next line still has some bytes to print.
2020-03-23T23:21:11.6364074Z          // Print a new line header if the next line still has some bytes to print.
2020-03-23T23:21:11.6364431Z          if i == line_start + BYTES_PER_LINE && i != alloc.size {
2020-03-23T23:21:11.6364821Z -            line_start =
2020-03-23T23:21:11.6365293Z -                write_allocation_newline(w, line_start, &ascii, pos_width, prefix)?;
2020-03-23T23:21:11.6365703Z +            line_start = write_allocation_newline(w, line_start, &ascii, pos_width, prefix)?;
2020-03-23T23:21:11.6366004Z              ascii.clear();
2020-03-23T23:21:11.6366315Z      }
2020-03-23T23:21:11.6366315Z      }
2020-03-23T23:21:11.6383168Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/util/pretty.rs"` failed.
2020-03-23T23:21:11.6384181Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T23:21:11.6393118Z Build completed unsuccessfully in 0:00:37
2020-03-23T23:21:11.6440487Z == clock drift check ==
2020-03-23T23:21:11.6452321Z   local time: Mon Mar 23 23:21:11 UTC 2020
2020-03-23T23:21:11.9347308Z   network time: Mon, 23 Mar 2020 23:21:11 GMT
2020-03-23T23:21:11.9347308Z   network time: Mon, 23 Mar 2020 23:21:11 GMT
2020-03-23T23:21:11.9349981Z == end clock drift check ==
2020-03-23T23:21:13.4961820Z 
2020-03-23T23:21:13.5025562Z ##[error]Bash exited with code '1'.
2020-03-23T23:21:13.5035409Z ##[section]Finishing: Run build
2020-03-23T23:21:13.5079452Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-23T23:21:13.5083779Z Task         : Get sources
2020-03-23T23:21:13.5084131Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T23:21:13.5084463Z Version      : 1.0.0
2020-03-23T23:21:13.5084690Z Author       : Microsoft
2020-03-23T23:21:13.5084690Z Author       : Microsoft
2020-03-23T23:21:13.5085040Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T23:21:13.5085465Z ==============================================================================
2020-03-23T23:21:13.8148980Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T23:21:13.8199113Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-23T23:21:13.8282592Z Cleaning up task key
2020-03-23T23:21:13.8283762Z Start cleaning up orphan processes.
2020-03-23T23:21:13.8448531Z Terminate orphan process: pid (3438) (python)
2020-03-23T23:21:13.8635310Z ##[section]Finishing: Finalize Job
