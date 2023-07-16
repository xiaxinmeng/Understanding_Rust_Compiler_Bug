
bash-4.2$ cargo build --verbose
       Fresh khronos_api v0.0.4
       Fresh pkg-config v0.1.5
       Fresh unsafe-any v0.2.1
       Fresh regex v0.1.10
       Fresh bitflags v0.1.0
       Fresh glium_macros v0.0.1 (https://github.com/tomaka/glium.git#fe9e2b5b)
       Fresh rustc-serialize v0.2.8
       Fresh vecmath v0.0.3 (https://github.com/PistonDevelopers/vecmath#3080ca14)
       Fresh phantom v0.0.3
       Fresh gl_common v0.0.3
       Fresh compile_msg v0.1.3 (https://github.com/huonw/compile_msg#b19da50c)
       Fresh khronos_api v0.0.5
       Fresh log v0.1.9
       Fresh xml-rs v0.1.13
       Fresh nalgebra v0.2.0 (https://github.com/sebcrozet/nalgebra#713589ff)
       Fresh num v0.1.9 (https://github.com/rust-lang/num#51b94c5e)
       Fresh cgmath v0.0.1 (https://github.com/bjz/cgmath-rs#9bb77e6a)
       Fresh typemap v0.0.8 (https://github.com/reem/rust-typemap#3594b654)
       Fresh gl_generator v0.0.12
       Fresh image v0.2.0-alpha.6 (https://github.com/PistonDevelopers/image#52fe4324)
       Fresh libz-sys v0.1.0
       Fresh freetype-sys v0.0.2 (https://github.com/PistonDevelopers/freetype-sys#b0c1eaa3)
       Fresh glutin v0.0.4-pre (https://github.com/tomaka/glutin.git#13be9b25)
       Fresh freetype-rs v0.0.3 (https://github.com/PistonDevelopers/freetype-rs#b3db542d)
       Fresh glium v0.0.2 (https://github.com/tomaka/glium.git#fe9e2b5b)
   Compiling lux v0.0.1 (file:///homes/iws/tyoverby/workspace/Personal/rust/lux)
     Running `rustc src/lib.rs --crate-name lux --crate-type lib -g -C metadata=0044c44cb706a208 -C extra-filename=-0044c44cb706a208 --out-dir /homes/iws/tyoverby/workspace/Personal/rust/lux/target --emit=dep-info,link -L dependency=/homes/iws/tyoverby/workspace/Personal/rust/lux/target -L dependency=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps --extern freetype=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libfreetype-47f57fe79f3e1411.rlib --extern glutin=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglutin-abe6f4e5d9a1c8bb.rlib --extern vecmath=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libvecmath-350a77008be8648b.rlib --extern glium_macros=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglium_macros-0d63a9614c44c703.so --extern image=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libimage-49298ff37d3495f7.rlib --extern glium=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglium-af9a1675d224f665.rlib --extern typemap=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libtypemap-3efd7ccbb9d24a2a.rlib -L native=/usr/lib64 -L native=/usr/lib64`
     Running `rustc src/bin.rs --crate-name lux_demo --crate-type bin -g --out-dir /homes/iws/tyoverby/workspace/Personal/rust/lux/target --emit=dep-info,link -L dependency=/homes/iws/tyoverby/workspace/Personal/rust/lux/target -L dependency=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps --extern freetype=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libfreetype-47f57fe79f3e1411.rlib --extern glutin=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglutin-abe6f4e5d9a1c8bb.rlib --extern vecmath=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libvecmath-350a77008be8648b.rlib --extern glium_macros=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglium_macros-0d63a9614c44c703.so --extern image=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libimage-49298ff37d3495f7.rlib --extern glium=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libglium-af9a1675d224f665.rlib --extern typemap=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/deps/libtypemap-3efd7ccbb9d24a2a.rlib --extern lux=/homes/iws/tyoverby/workspace/Personal/rust/lux/target/liblux-0044c44cb706a208.rlib -L native=/usr/lib64 -L native=/usr/lib64`
src/bin.rs:29:5: 29:33 warning: unused result which must be used, #[warn(unused_must_use)] on by default
src/bin.rs:29     face.set_pixel_sizes(0, 48);
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
