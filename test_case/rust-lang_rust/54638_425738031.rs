plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1ba24530
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:03:43] Looks like docker image is the same as before, not uploading
[00:03:43] travis_fold:end:build_docker

nfigure-args := ['--enable-full-tools', '--enable-sanitizers', ...
[00:03:44]     configure_section(sections[section_key], section_config)
[00:03:44]   File "/checkout/src/bootstrap/configure.py", line 425, in configure_section
[00:03:44]     raise RuntimeError("failed to find config line for {}".format(key))
[00:03:44] RuntimeError: failed to find config line for missing-tools

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:005d7983
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
