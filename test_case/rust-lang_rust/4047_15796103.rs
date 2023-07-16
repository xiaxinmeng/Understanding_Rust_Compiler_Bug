
<@pcwalton> I know of no other static language with declarative imports
            that allows (a) reexports; (b) glob import; (c) mutual
            recursion everywhere; (d) nested modules; (e) two namespaces
<@pcwalton> java doesn't have reexports, Go doesn't have nested modules
            or reexports, C# doesn't have reexports I don't think, C++
            doesn't have reexports or mutual recursion everywhere, maybe
            D...
<@pcwalton> ML and Haskell don't have mutual recursion everywhere and
            don't have reexports
<@pcwalton> mutually recursive modules too
<@pcwalton> most languages do not have mutually recursive modules
< bstrie> pcwalton: so what you're saying is that rust has once again
          failed to be unoriginal :)
<@pcwalton> well, as you said, Python
<@pcwalton> but Rust is trying to achieve Python's module system's
            expressive power in a static system
  jld tries to remember how the various Scheme module systems work
<@pcwalton> turns out that's pretty hard :)
<@pcwalton> jld: no mutual recursion
<@pcwalton> btw, ES6 hit the exact same problems we did
<@pcwalton> and had to pare down their module system as a result
<@pcwalton> anyway none of this is an excuse, we should make it easier to
            use
<@pcwalton> but I'm just saying it's not that easy...
