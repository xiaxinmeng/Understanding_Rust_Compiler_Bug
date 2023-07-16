plain
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
    found:    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    expected: 9a13a8c8e16028d0e07608da8fbd0ea5b51d9d425230712adcbac679ef5fef9d
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 950, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 914, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 436, in download_toolchain
    self._download_component_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 473, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 51, in get
    raise RuntimeError("failed verification")
make: *** [prepare] Error 1
Command failed. Attempt 2/5:
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
    found:    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    expected: 9a13a8c8e16028d0e07608da8fbd0ea5b51d9d425230712adcbac679ef5fef9d
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 950, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 914, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 436, in download_toolchain
    self._download_component_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 473, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 51, in get
    raise RuntimeError("failed verification")
make: *** [prepare] Error 1
Command failed. Attempt 3/5:
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
    found:    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    expected: 9a13a8c8e16028d0e07608da8fbd0ea5b51d9d425230712adcbac679ef5fef9d
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 950, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 914, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 436, in download_toolchain
    self._download_component_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 473, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 51, in get
    raise RuntimeError("failed verification")
make: *** [prepare] Error 1
Command failed. Attempt 4/5:
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
    found:    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    expected: 9a13a8c8e16028d0e07608da8fbd0ea5b51d9d425230712adcbac679ef5fef9d
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 950, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 914, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 436, in download_toolchain
    self._download_component_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 473, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 51, in get
    raise RuntimeError("failed verification")
make: *** [prepare] Error 1
Command failed. Attempt 5/5:
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2023-03-07/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
curl: option --retry-connrefused: is unknown
curl: try 'curl --help' or 'curl --manual' for more information
    found:    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    expected: 9a13a8c8e16028d0e07608da8fbd0ea5b51d9d425230712adcbac679ef5fef9d
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
  File "/checkout/src/bootstrap/bootstrap.py", line 965, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 950, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 914, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 436, in download_toolchain
    self._download_component_helper(filename, pattern, tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 473, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 51, in get
    raise RuntimeError("failed verification")
make: *** [prepare] Error 1
The command has failed after 5 attempts.
