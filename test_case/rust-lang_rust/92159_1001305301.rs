plain
hw.tbfrequency: 1000000000
hw.use_kernelmanagerd: 1
+ ./x.py dist --stage 2
    Finished dev [unoptimized] target(s) in 0.32s
thread 'main' panicked at 'fs::read_dir(builder.src.join(&relative_path).join("redirects")) failed with No such file or directory (os error 2)', src/bootstrap/doc.rs:235:21
Build completed unsuccessfully in 0:00:00
