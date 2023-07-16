
[INFO] [stderr]    Compiling display-as v0.4.6 (/opt/crater/workdir)
[INFO] [stderr] error: custom attribute panicked
[INFO] [stderr]    --> tests/format_as.rs:104:5
[INFO] [stderr]     |
[INFO] [stderr] 104 |     #[with_template("Foo " self.0)]
[INFO] [stderr]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[INFO] [stderr]     |
[INFO] [stderr]     = help: message: with_template must be applied to an impl that ends in '{}', not { }
