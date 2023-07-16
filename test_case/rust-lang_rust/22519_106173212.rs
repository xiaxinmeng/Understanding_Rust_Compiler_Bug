 rust
   pub trait IntoIterator {
       /// The type of the elements being iterated
       #[stable(feature = "rust1", since = "1.0.0")]
       type Item;
   
       /// A container for iterating over elements of type `Item`
       #[stable(feature = "rust1", since = "1.0.0")]
       type IntoIter: Iterator<Item=Self::Item>;
   
       /// Consumes `Self` and returns an iterator over it
       #[stable(feature = "rust1", since = "1.0.0")]
       fn into_iter(self) -> Self::IntoIter;
   }
   