
$ cd app
$ cargo rustc --release -- -C lto
(..)
  = note: /home/japaric/tmp/app/target/release/deps/app-fb503aa189655f1d.app0-fe76e448b3289da3a00eec0c3c849ae7.rs.rcgu.o: In function `main':
          app0-fe76e448b3289da3a00eec0c3c849ae7.rs:(.text.main+0x2): undefined reference to `FOO'
          collect2: error: ld returned 1 exit status
