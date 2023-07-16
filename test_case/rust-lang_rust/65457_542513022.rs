plain
2019-10-16T04:49:50.7787658Z + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
2019-10-16T04:49:51.0900214Z     Finished dev [unoptimized] target(s) in 0.22s
2019-10-16T04:49:51.1788747Z thread 'main' panicked at '
2019-10-16T04:49:51.1789747Z 
2019-10-16T04:49:51.1790316Z couldn't find required command: "riscv32-unknown-elf-gcc"
2019-10-16T04:49:51.1790676Z ', src/bootstrap/sanity.rs:57:13
2019-10-16T04:49:51.1790787Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-16T04:49:51.1834711Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
2019-10-16T04:49:51.1835153Z Build completed unsuccessfully in 0:00:00
2019-10-16T04:49:51.1835153Z Build completed unsuccessfully in 0:00:00
2019-10-16T04:49:51.1890117Z == clock drift check ==
2019-10-16T04:49:51.1905515Z   local time: Wed Oct 16 04:49:51 UTC 2019
2019-10-16T04:49:51.7318637Z   network time: Wed, 16 Oct 2019 04:49:51 GMT
2019-10-16T04:49:51.7322900Z == end clock drift check ==
2019-10-16T04:49:54.4106372Z ##[error]Bash exited with code '1'.
2019-10-16T04:49:54.4157741Z ##[section]Starting: Upload CPU usage statistics
2019-10-16T04:49:54.4163843Z ==============================================================================
2019-10-16T04:49:54.4163945Z Task         : Bash
2019-10-16T04:49:54.4164023Z Description  : Run a Bash script on macOS, Linux, or Windows
