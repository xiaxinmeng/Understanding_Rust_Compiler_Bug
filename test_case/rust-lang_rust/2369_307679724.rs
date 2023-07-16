
$ ghc --make Main.hs
Module imports form a cycle:
         module ‘Foo’ (./Foo.hs)
        imports ‘Bar’ (./Bar.hs)
  which imports ‘Foo’ (./Foo.hs)
