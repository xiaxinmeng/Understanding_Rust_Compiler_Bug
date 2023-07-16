
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> src/main.rs:4:5
  |
3 | fn convert<T: Trait + ?Sized>(b: Box<T>) -> Box<dyn Trait> {
  |            - this type parameter needs to be `std::marker::Sized`
4 |     b
  |     ^ doesn't have a size known at compile-time
  |
  = note: only pointee types that do not have metadata can be cast to the object type `dyn Trait`
          as casting to a pointer to `dyn Trait` requires adding metadata in the form of a vtable
  = note: currently, `Sized` types are exactly the pointee types that do not have metadata;
          types which are not `Sized` have their own metadata such as slice lengths or vtables
