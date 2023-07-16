console
$ cd src/bootstrap
$ tox
GLOB sdist-make: /Development/rust/rust/src/bootstrap/setup.py
py27 create: /Development/rust/rust/src/bootstrap/.tox/py27
py27 inst: /Development/rust/rust/src/bootstrap/.tox/dist/rust-bootstrap-0.0.1.zip
py27 installed: rust-bootstrap==0.0.1
py27 runtests: PYTHONHASHSEED='3410238989'
py27 runtests: commands[0] | /Development/rust/rust/src/bootstrap/.tox/py27/bin/python bootstrap_test.py
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bootstrap_binary ... ok
cargo_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.cargo_stamp ... ok
get_mk (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_mk ... ok
get_string (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_string ... ok
get_toml (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_toml ... ok
rustc_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.rustc_stamp ... ok
format_build_time (bootstrap)
Doctest: bootstrap.format_build_time ... ok
test_stage0_data (__main__.Stage0DataTestCase)
Extract data from stage0.txt ... ok
test_invalid_file (__main__.VerifyTestCase)
Should verify that the file is invalid ... invalid checksum:
    found:    334d016f755cd6dc58c53a86e183882f8ec14f52fb05345887c8a5edd42c87b7
    expected: 64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c
ok
test_valid_file (__main__.VerifyTestCase)
Check if the sha256 sum of the given file is valid ... ok

----------------------------------------------------------------------
Ran 11 tests in 0.013s

OK
py35 create: /Development/rust/rust/src/bootstrap/.tox/py35
py35 inst: /Development/rust/rust/src/bootstrap/.tox/dist/rust-bootstrap-0.0.1.zip
py35 installed: rust-bootstrap==0.0.1
py35 runtests: PYTHONHASHSEED='3410238989'
py35 runtests: commands[0] | /Development/rust/rust/src/bootstrap/.tox/py35/bin/python bootstrap_test.py
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bootstrap_binary ... ok
cargo_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.cargo_stamp ... ok
get_mk (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_mk ... ok
get_string (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_string ... ok
get_toml (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_toml ... ok
rustc_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.rustc_stamp ... ok
format_build_time (bootstrap)
Doctest: bootstrap.format_build_time ... ok
test_stage0_data (__main__.Stage0DataTestCase)
Extract data from stage0.txt ... ok
test_invalid_file (__main__.VerifyTestCase)
Should verify that the file is invalid ... invalid checksum:
    found:    334d016f755cd6dc58c53a86e183882f8ec14f52fb05345887c8a5edd42c87b7
    expected: 64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c
ok
test_valid_file (__main__.VerifyTestCase)
Check if the sha256 sum of the given file is valid ... ok

----------------------------------------------------------------------
Ran 11 tests in 0.021s

OK
py36 create: /Development/rust/rust/src/bootstrap/.tox/py36
py36 inst: /Development/rust/rust/src/bootstrap/.tox/dist/rust-bootstrap-0.0.1.zip
py36 installed: rust-bootstrap==0.0.1
py36 runtests: PYTHONHASHSEED='3410238989'
py36 runtests: commands[0] | /Development/rust/rust/src/bootstrap/.tox/py36/bin/python bootstrap_test.py
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bootstrap_binary ... ok
cargo_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.cargo_stamp ... ok
get_mk (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_mk ... ok
get_string (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_string ... ok
get_toml (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_toml ... ok
rustc_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.rustc_stamp ... ok
format_build_time (bootstrap)
Doctest: bootstrap.format_build_time ... ok
test_stage0_data (__main__.Stage0DataTestCase)
Extract data from stage0.txt ... ok
test_invalid_file (__main__.VerifyTestCase)
Should verify that the file is invalid ... invalid checksum:
    found:    334d016f755cd6dc58c53a86e183882f8ec14f52fb05345887c8a5edd42c87b7
    expected: 64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c
ok
test_valid_file (__main__.VerifyTestCase)
Check if the sha256 sum of the given file is valid ... ok

----------------------------------------------------------------------
Ran 11 tests in 0.019s

OK
___________________________________ summary ____________________________________
  py27: commands succeeded
  py35: commands succeeded
  py36: commands succeeded
  congratulations :)
