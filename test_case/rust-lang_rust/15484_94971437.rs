 rust
extern crate gl;

use std::mem;

use gl::types::GLsizeiptr;
use gl::types::GLuint;

#[allow(unreachable_code)]
fn main() {

    return;

    let elements: [GLuint; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    unsafe {
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (elements.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       mem::transmute(&elements[0]),
                       gl::STATIC_DRAW);
    }
}
