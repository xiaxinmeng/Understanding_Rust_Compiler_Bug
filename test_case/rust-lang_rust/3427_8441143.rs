
struct vec3 { data:[float * 3] }

// constructor
pure fn vec3<T>(x:T, y:T, z:T) -> vec3<T> {
    vec3<T> { data: [ x, y, z ] }
}

// 3-dimensional functions
trait Vector3<T> {

    ...
}

// N-dimensional functions
impl vec3: Vector<T> {

    // returns the number of components in the vector (conflicts with constructor)
    #[inline(always)] static pure fn dim() -> uint { 3 }

    ...
}
