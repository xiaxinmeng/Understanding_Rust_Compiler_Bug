rust
// Dummy
struct Vertex;


// Mesh kind marker types. Can be replaced with enum once const generic 
// lands. This trait is "sealed", i.e. it is only implemented by these two
// types.
trait FaceKind {}
enum TriFace {}
enum PolyFace {}
impl FaceKind for TriFace {}
impl FaceKind for PolyFace {}


trait Mesh {
    type FaceKind: FaceKind;
    
    // All meshes can add triangles, but only some meshes can add 
    // arbitrary faces.
    fn add_triangle(&mut self, vertices: &[Vertex; 3]);
    fn add_face(&mut self, vertices: &[Vertex])
    where
        Self: Mesh<FaceKind = PolyFace>;
}


struct MyTriMesh {}
impl Mesh for MyTriMesh {
    type FaceKind = TriFace;
    
    fn add_triangle(&mut self, vertices: &[Vertex; 3]) {
        // stuff
    }
    
    fn add_face(&mut self, vertices: &[Vertex]) {
        unreachable!() // <-- this is annoying and not robust
    }
}
