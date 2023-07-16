
error[E0308]: mismatched types
  --> src/lib.rs:15:9
   |
15 |         self.data.as_mut().map(|v| v.deref_mut()) //Lifetime mismatch
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected enum `Option<&mut dyn SectionData>`
              found enum `Option<&mut (dyn SectionData + 'static)>`
note: the anonymous lifetime defined here...
  --> src/lib.rs:13:17
   |
13 |     pub fn open(&mut self) -> Option<&mut dyn SectionData>
   |                 ^^^^^^^^^
   = note: ...does not necessarily outlive the static lifetime
