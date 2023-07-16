
    bb8: {
        tmp34 = &mut var1;               // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:25: 4168:35
        tmp37 = const "glTexCoord3iv";   // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:36: 4168:51
        tmp36 = &(*tmp37);               // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:36: 4168:51
        tmp40 = promoted1620;            // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:53: 4168:56
        tmp39 = &(*tmp40);               // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:53: 4168:56
        tmp38 = tmp39 as &[&str] (Unsize); // scope 2 at ../target/release/build
/glium-ea08b897ed8f243c/out/gl_bindings.rs:4168:53: 4168:56
        tmp35 = (tmp36, tmp38);          // scope 2 at ../target/release/build/g
lium-ea08b897ed8f243c/out/gl_bindings.rs:4168:25: 4168:57
        tmp33 = <[closure@../target/release/build/glium-ea08b897ed8f243c/out/gl_bindings.rs:4161:38: 4163:18 loadfn:&mut F] as std::ops::FnMut<(&str, &[&str])>>::call_mut(tmp34, tmp35) -> [return: bb9, unwind: bb2]; // scope 2 at ../target/release/build/glium-ea08b897ed8f243c/out/gl_bindings.rs:4168:25: 4168:57
    }


    bb9: {
        tmp32 = FnPtr::new(tmp33) -> [return: bb10, unwind: bb2]; // scope 2 at ../target/release/build/glium-ea08b897ed8f243c/out/gl_bindings.rs:4168:14: 4168:58
    }
