rust
extern { type E; }
struct X {
   a: u8,
   b: E,    
   //^ we can't know the alignment of E,
   //  so we can't know offset_of!(X,b) even at runtime
}
