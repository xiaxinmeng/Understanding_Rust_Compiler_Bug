plain
travis_time:end:0e06eaa0:start=1559265893032168119,finish=1559265983459214359,duration=90427046240
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
DEPRECATION: Python 2.7 will reach the end of its life on January 1st, 2020. Please upgrade your Python as Python 2.7 won't be maintained after that date. A future version of pip will drop support for Python 2.7.
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/ae/df/edd5e30bd7b7e6ffc541f44b922424d947be06b56fceb0d414b4164268ab/awscli-1.16.169-py2.py3-none-any.whl (1.6MB)
    1% |▍                               | 20kB 2.2MB/s eta 0:00:01
    1% |▋                               | 30kB 3.1MB/s eta 0:00:01
    2% |▉                               | 40kB 2.1MB/s eta 0:00:01
    3% |█                               | 51kB 2.5MB/s eta 0:00:01
---
[00:28:10]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:28:10]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:28:11]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:28:11]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:28:26] error: impl has missing stability attribute
[00:28:26]     |
[00:28:26]     |
[00:28:26] 209 | / impl<'a, 'b, A: Sized, B, const N: usize> PartialEq<&'b [B]> for [A; N] where A: PartialEq<B> {
[00:28:26] 210 | |     #[inline]
[00:28:26] 211 | |     fn eq(&self, other: &&'b [B]) -> bool { self[..] == other[..] }
[00:28:26] 212 | |     #[inline]
[00:28:26] 213 | |     fn ne(&self, other: &&'b [B]) -> bool { self[..] != other[..] }
[00:28:26]     | |_^
[00:28:26] 
[00:28:27] error: aborting due to previous error
[00:28:27] 
