
union U { f: f32, i: i32 }
let u = U { f: 1.0 }; // Initialize one field, another becomes initialized too
let i = unsafe { u.i }; // OK, `i` became initialized together with `f`, so we don't get an error here
