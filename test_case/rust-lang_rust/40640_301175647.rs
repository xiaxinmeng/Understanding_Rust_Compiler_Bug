
trait Thing {}

type Alias<T: Thing> = (T);

type Bad = Alias<i32>; // i32 doesn't implement Thing, but no error
