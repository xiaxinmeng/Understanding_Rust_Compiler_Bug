shell
$ RUSTFLAGS_BOOTSTRAP="-Z sanitizer=address" ./x.py test --stage 0 library/std --test-args read_large_dir
