
fn main() {
    { 2 } // <- trailing expression if only "output(empty!())  ;" is re-parsed after the macro expansion
           // (assuming empty statements are ignored when determining trailing-ness)
           // <- not a trailing expression if some larger context is re-parsed after the macro expansion

    ; // An "empty statement" if only "output(empty!())  ;" is re-parsed after the macro expansion
      // A part of "{ 2 };" statement if some larger context is re-parsed after the macro expansion
}
