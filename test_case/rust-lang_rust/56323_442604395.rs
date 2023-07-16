plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:039dcbd8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4f/dd/d1b374af4e5f374173517e7d63e2f4489d3d2e9a45626df830c885412cc9/awscli-1.16.64-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 9.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    2% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[00:06:33]    Compiling polonius-engine v0.5.0
[00:06:33]    Compiling chalk-engine v0.8.0
[00:06:34]    Compiling env_logger v0.5.12
[00:06:35]    Compiling parking_lot_core v0.3.0
[00:06:36]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:06:39]    Compiling parking_lot v0.6.4
[00:06:40]    Compiling crossbeam-epoch v0.3.1
[00:06:42]    Compiling log_settings v0.1.2
[00:06:42]    Compiling flate2 v1.0.3
---
[00:53:49]    Compiling chalk-engine v0.8.0
[00:53:49]    Compiling env_logger v0.5.12
[00:53:51]    Compiling tempfile v3.0.3
[00:53:52]    Compiling parking_lot_core v0.3.0
[00:53:52]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:53:54]    Compiling parking_lot v0.6.4
[00:53:55]    Compiling crossbeam-epoch v0.3.1
[00:53:57]    Compiling log_settings v0.1.2
[00:53:57]    Compiling flate2 v1.0.3
[00:53:57]    Compiling flate2 v1.0.3
[00:53:58]    Compiling backtrace v0.3.9
[00:54:00]    Compiling crossbeam-deque v0.2.0
[00:54:02]    Compiling rustc-rayon v0.1.1
[00:54:08]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:54:08] error: unused import: `rustc_ezilaires as rustc_serialize`
[00:54:08]    |
[00:54:08]    |
[00:54:08] 40 | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize; // used by deriving
[00:54:08]    |
[00:54:08]    = note: `-D unused-imports` implied by `-D warnings`
[00:54:08] 
[00:54:10] error: aborting due to previous error
