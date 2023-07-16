plain
Updating files:  99% (32702/33032)
Updating files:  99% (32884/33032)
Updating files: 100% (33032/33032)
Updating files: 100% (33032/33032), done.
Switched to a new branch 'try'
branch 'try' set up to track 'origin/try'.
[command]/usr/local/bin/git log -1 --format='%H'
'32aa173804582e19f5f6a0bcebda41e3e2676755'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMccEynRZYc0
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
[RUSTC-TIMING] build_script_build test:false 0.562
[RUSTC-TIMING] build_script_build test:false 0.562
   Compiling version_check v0.9.3
   Compiling cfg-if v1.0.0
error: File `artifacts/reproducible-artifacts-nightly-x86_64-unknown-linux-gnu/reproducible-artifacts/rustc-pgo.profdata` passed to `-C profile-use` does not exist.
[RUSTC-TIMING] cfg_if test:false 0.038
error: could not compile `cfg-if` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] build_script_build test:false 0.590
[RUSTC-TIMING] build_script_build test:false 0.590
[RUSTC-TIMING] version_check test:false 0.481
Build completed unsuccessfully in 0:43:53
Traceback (most recent call last):
  File "/Users/runner/work/rust/rust/src/ci/pgo-dist.py", line 32, in <module>
Running python x.py dist --rust-profile-use artifacts/reproducible-artifacts-nightly-x86_64-unknown-linux-gnu/reproducible-artifacts/rustc-pgo.profdata --host=x86_64-apple-darwin --target=x86_64-apple-darwin
    subprocess.run(args, check=True, env=env)
  File "/usr/local/Cellar/python@3.9/3.9.12/Frameworks/Python.framework/Versions/3.9/lib/python3.9/subprocess.py", line 528, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['python', 'x.py', 'dist', '--rust-profile-use', 'artifacts/reproducible-artifacts-nightly-x86_64-unknown-linux-gnu/reproducible-artifacts/rustc-pgo.profdata', '--host=x86_64-apple-darwin', '--target=x86_64-apple-darwin']' returned non-zero exit status 1.
