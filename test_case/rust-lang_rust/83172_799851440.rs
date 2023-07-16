
> configure: rust.channel         := nightly
> configure: rust.debug-assertions := True
> configure: llvm.assertions      := True
> configure: dist.missing-tools   := True
> configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
> configure: writing `config.toml` in current directory
> configure: 
> configure: run `python /checkout/x.py --help`
> configure: 
> ---
> skip untracked path cpu-usage.csv during rustfmt invocations
> skip untracked path src/doc/book/ during rustfmt invocations
> skip untracked path src/doc/rust-by-example/ during rustfmt invocations
> skip untracked path src/llvm-project/ during rustfmt invocations
> Diff in /checkout/src/bootstrap/test.rs at line 792:
>          if builder.config.channel == "dev" || builder.config.channel == "nightly" {
>              builder.info("fmt check");
>              if builder.config.initial_rustfmt.is_none() {
> -                let inferred_rustfmt_dir =
> -                    builder.config.initial_rustc.parent().unwrap();
> -                eprintln!("\
> +                let inferred_rustfmt_dir = builder.config.initial_rustc.parent().unwrap();
> +                eprintln!(
> +                    "\
>  error: no `rustfmt` binary found in {PATH}
>  info: `rust.channel` is currently set to \"{CHAN}\"
>  help: if you are testing a beta branch, set `rust.channel` to \"beta\" in the `config.toml` file
> Diff in /checkout/src/bootstrap/test.rs at line 801:
>  help: to skip test's attempt to check tidiness, pass `--exclude src/tools/tidy` to `x.py test`",
> -                          PATH=inferred_rustfmt_dir.display(),
> -                          CHAN=builder.config.channel,
> +                    PATH = inferred_rustfmt_dir.display(),
> +                    CHAN = builder.config.channel,
>                  );
>                  std::process::exit(1);
>              }
> Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs"` failed.
> If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
> Build completed unsuccessfully in 0:00:16
> 