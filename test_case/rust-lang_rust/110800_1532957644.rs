plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-avyp1iys/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built eb5d8310f23f
Successfully tagged rust-ci:latest
Built container sha256:eb5d8310f23f3f3b60b1d767e670b0124ec04627ed3c22577e054d90850d29dd
Uploading finished image to https://ci-caches.rust-lang.org/docker/4e94059389e21b7da9337056716e9949fcb3f515571262870670b891bfc0802b3ffef74b9bc0b89b7496b9b3116bbdf8e32c6367639ab87759ad6227244406f7
upload failed: - to s3://rust-lang-ci-sccache2/docker/4e94059389e21b7da9337056716e9949fcb3f515571262870670b891bfc0802b3ffef74b9bc0b89b7496b9b3116bbdf8e32c6367639ab87759ad6227244406f7 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
   Compiling basic-toml v0.1.2
   Compiling askama_derive v0.12.1
    Checking askama v0.12.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the trait bound `SubdiagnosticMessage: From<&std::string::String>` is not satisfied
   --> src/librustdoc/passes/check_custom_code_classes.rs:70:13
67  |           .note(
    |            ---- required by a bound introduced by this call
...
70  | /             &format!(
70  | /             &format!(
71  | |                 "found these custom classes: class={}",
72  | |                 tests.custom_classes_found.join(",class=")
73  | |             ),
    | |_____________^ the trait `From<&std::string::String>` is not implemented for `SubdiagnosticMessage`
    |
    = note: required for `&std::string::String` to implement `Into<SubdiagnosticMessage>`
note: required by a bound in `DiagnosticBuilder::<'a, G>::note`
    |
    |
648 |     forward!(pub fn note(&mut self, msg: impl Into<SubdiagnosticMessage>) -> &mut Self);
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DiagnosticBuilder::<'a, G>::note`
   --> /checkout/library/alloc/src/macros.rs:119:23
    |
119 |     ($($arg:tt)*) => {*{
    |                       +
