plain
  'TOOLSTATE_PUBLISH': '1',
  'TOOLSTATE_REPO': 'https://github.com/rust-lang-nursery/rust-toolstate',
  'TOOLSTATE_REPO_ACCESS_TOKEN': '***',
  '_': '/usr/bin/python3'}
stage-build INFO: Executing `python3 x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest bootstrap`
python3: can't open file 'x.py': [Errno 2] No such file or directory
  File "../src/ci/stage-build.py", line 898, in <module>
    "build-manifest", "bootstrap"])
    "build-manifest", "bootstrap"])
  File "../src/ci/stage-build.py", line 510, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['python3', 'x.py', 'dist', '--host', 'x86_64-unknown-linux-gnu', '--target', 'x86_64-unknown-linux-gnu', '--include-default-paths', 'build-manifest', 'bootstrap']' returned non-zero exit status 2.
