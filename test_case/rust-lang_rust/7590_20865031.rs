
// CONFLICTING TRAIT TEST
// compiles ok until adding (iii) .. desired behaviour would be for this to compile

trait A { }// VertexGrid interface (heightfield, bezier patch..
trait B { }// trimesh interface

struct P<T> { // heightfield
  i:int}

struct Q<T> { // concrete trimesh of ... (may be more ways of storing)
  i:int}

// (i) heightfield has VertexGrid behaviours
impl<T> A for P<T> { }
// (ii) VertexGrid can be viewed as a mesh
impl<X:A> B for X { }
// (iii)concrete trimesh has trimesh interface
impl<T> B for Q<T> { } 
//^^^^ error: 25:1 conflicting implementations for a trait
// (iii) conflicting here (ii)
// ideally i'd hope the error/ambiguity would come *if* i implemeneted B for Q

fn main() { }
