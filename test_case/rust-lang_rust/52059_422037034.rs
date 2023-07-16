
15 |         self.url 
   |         ^^^^^^^^ borrow extends past lifetime of value 
16 |     }
   |     - `self` dropped here while `self.url` still borrowed
   |
   = note: `self` is of type `S<'_>`, which implements `Drop`; thus 
           dropping `self` requires exclusive access to `self.url`.
