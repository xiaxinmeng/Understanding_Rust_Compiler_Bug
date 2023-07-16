
Decode(AtPos(99))             <-- reserve/cache 99
  Decode(Alloc(99))           
    Decode(AtPos(42))         <-- reserve/cache 42
      Decode(Alloc(42))
        Decode(Alloc(99))
          Decode(AtPos(42))   <-- cache hit 42
          Done(Alloc(99))     <-- Alloc(99) interned
        Done(Alloc(42))       <-- Alloc(42) interned
    Done(AtPos(42))           <-- cache[42] = Alloc(42)
  Done(Alloc(99))             <-- Alloc(99) interned (again)
Done(AtPos(99))               <-- cache[99] = Alloc(99)
