
edwardw: to fix #12512, I feel like I must add a new AST node and then add a new visit_* or fold_* method
         to traits that traverse an AST. is that too big a change to propose?
...
eddyb: I guess we only perform renaming in one of 4 namespaces
edwardw: not after my hygiene label patch landed :)
edwardw: which also introduces the name clashing bug
eddyb: bleah, this kinda introduces the need for more resolve-like logic in expansion
       we could intertwine then, but that would cause all sorts of weird things
edwardw: eddyb: it already did https://github.com/mozilla/rust/issues/12512
eddyb: mkay, so what I would do instead, is have more renaming kinds
       one for each namespace
edwardw: exactly, that's why I'm thinking of having a Label AST node
...
eddyb: *record* the renaming
       expansion doesn't rename anything
       it just records renames
       (now that I think of it, more SyntaxContext_ variants will be harder than having another field in the Rename variant, like a RenameKind)
edwardw: that's another way to introduce namespaces for renaming
eddyb: mhmm
       and it's future-proof
edwardw: so you like this idea more than a Label AST node, yes?
eddyb: (I don't like it for other reasons, but it's the least threatening evil IMO)
edwardw: a Label node is kind evil, lol
eddyb: edwardw: well, yeah, I like how rename kinds can work even with namespaces, where just the AST shape won't help
       and/or would require duplicating resolve logic in expansion
       but it warrants a discussion with more than just the two of us
edwardw: sounds reasonably. let me sleep on the ideas and come up with a gist for further discussion
