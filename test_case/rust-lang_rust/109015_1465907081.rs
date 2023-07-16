plain
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/measureme`
    Updating git repository `https://github.com/rust-lang/team`
    Updating git repository `https://github.com/rust-lang/measureme`
warning: spurious network error (2 tries remaining): failed to get successful HTTP response from `https://index.crates.io/3/r/rgb`, got 503


warning: spurious network error (1 tries remaining): failed to get successful HTTP response from `https://index.crates.io/3/r/rgb`, got 503


error: failed to get `rgb` as a dependency of package `inferno v0.10.6`
    ... which satisfies dependency `inferno = "^0.10"` (locked to 0.10.6) of package `site v0.1.0 (/tmp/tmp-multistage/opt-artifacts/rustc-perf/site)`
Caused by:
  failed to query replaced source registry `crates-io`

Caused by:
Caused by:
  download of 3/r/rgb failed
Caused by:
Caused by:
  failed to get successful HTTP response from `https://index.crates.io/3/r/rgb`, got 503
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
-----------------------------------------
-----------------------------------------
                                         
Total duration:                        0s
-----------------------------------------
root INFO: Free disk space: 601.22 GiB out of total 666.61 GiB (9.81% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 752, in execute_build_pipeline
    pipeline.build_rustc_perf()
  File "../src/ci/stage-build.py", line 145, in build_rustc_perf
    RUSTC_BOOTSTRAP="1"
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo', 'build', '-p', 'collector']' returned non-zero exit status 101.
