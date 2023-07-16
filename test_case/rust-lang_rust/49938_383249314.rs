rust
   /// General-case enums: for each case there is a struct, and they all have
   /// all space reserved for the tag, and their first field starts
   /// at a non-0 offset, after where the tag would go.
   Tagged {
       tag: Scalar,
       variants: Vec<LayoutDetails>,
   },
