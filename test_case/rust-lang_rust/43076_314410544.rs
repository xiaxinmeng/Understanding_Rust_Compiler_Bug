
fn test() -> impl GeneratorNoReturn<u8> {
   for i in 1 .. 6 {
       yield i
   }
}
