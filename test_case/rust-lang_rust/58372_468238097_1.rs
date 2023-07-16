rust
  struct StringTableEntry {
     components: [EntryComponent]
  }

  enum EntryComponent {
     Contents(String),
     Ref(StringTableId),
     End,
  } 
  