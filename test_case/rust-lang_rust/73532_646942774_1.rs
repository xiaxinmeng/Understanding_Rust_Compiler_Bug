rust
   2195 #[stable(feature = "rust1", since = "1.0.0")]
   2196 impl<T: ?Sized + Debug> Debug for RefCell<T> {
   2197     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
   2198         match self.try_borrow() {
   2199             Ok(borrow) => f.debug_struct("RefCell").field("value", &borrow).f
   2199 inish(),
   2200             Err(_) => {
   2201                 // The RefCell is mutably borrowed so we can't look at its va
   2201 lue
   2202                 // here. Show a placeholder instead.
   2203                 struct BorrowedPlaceholder;
   2204 
   2205                 impl Debug for BorrowedPlaceholder {
   2206                     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
   2207                         f.write_str("<borrowed>")
   2208                     }
   2209                 }
   2210 
   2211                 f.debug_struct("RefCell").field("value", &BorrowedPlaceholder   2211 ).finish()
   2212             }
   2213         }
   2214     }
   2215 }
