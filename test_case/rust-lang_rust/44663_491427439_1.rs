 txt
cargo doc --no-deps --open
 Documenting barc v1.2.0 (/home/david/src/body-image/barc)
error: Unrecognized option: 'extern-private'

error: Could not document `barc`.

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-name barc barc/src/lib.rs --color always -o /home/david/src/body-image/target/doc --cfg 'feature="body-image"' --cfg 'feature="brotli"' --cfg 'feature="default"' --cfg 'feature="memmap"' --cfg 'feature="mmap"' --cfg 'feature="olio"' -L dependency=/home/david/src/body-image/target/debug/deps --extern body_image=/home/david/src/body-image/target/debug/deps/libbody_image-f6307513eafde4da.rmeta --extern-private brotli=/home/david/src/body-image/target/debug/deps/libbrotli-914be93b87d00a74.rmeta --extern-private bytes=/home/david/src/body-image/target/debug/deps/libbytes-60c7a2c9551c05c4.rmeta --extern-private flate2=/home/david/src/body-image/target/debug/deps/libflate2-3410ad175205616a.rmeta --extern http=/home/david/src/body-image/target/debug/deps/libhttp-1bcf97f938c6fdd7.rmeta --extern-private httparse=/home/david/src/body-image/target/debug/deps/libhttparse-b04e539fbadba356.rmeta --extern-private memmap=/home/david/src/body-image/target/debug/deps/libmemmap-b29e83fdba55bd43.rmeta --extern-private mime=/home/david/src/body-image/target/debug/deps/libmime-df20e72307836353.rmeta --extern-private olio=/home/david/src/body-image/target/debug/deps/libolio-c7a6754af0be8b56.rmeta --extern-private tao_log=/home/david/src/body-image/target/debug/deps/libtao_log-7da4b359386cbcf7.rmeta --extern-private tempfile=/home/david/src/body-image/target/debug/deps/libtempfile-5c5a144b50e3000b.rmeta -Z unstable-options --cfg barc_std_try_from` (exit code: 1)
