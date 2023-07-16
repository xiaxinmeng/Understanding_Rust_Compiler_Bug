rust
     enum LiteralKind {
         Byte,
         Char,
         Integer(Prefix, Suffix),
         Float(Prefix, Suffix),
         CookedStr,
         RawStr(usize),
         ByteStr,
         RawByteStr(usize),
         #[doc(hidden)]
         __NonExhausitive
     }

     struct Prefix(&'static str);

     impl Prefix { fn len(&self) -> usize }

     struct Suffix(&'static str);

     impl Suffix { fn len(&self) -> usize }
     