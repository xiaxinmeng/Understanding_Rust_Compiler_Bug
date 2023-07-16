
$ cat > test311.hs
class A a => A a
$ ghc test311.hs
[1 of 1] Compiling Main             ( test311.hs, test311.o )

test311.hs:1:1:
    Cycle in class declaration (via superclasses): A -> A
    In the class declaration for `A'
