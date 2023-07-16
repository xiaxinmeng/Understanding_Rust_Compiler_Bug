plain
Receiving objects: 100% (102/102), 11.41 KiB | 11.41 MiB/s, done.
---
Resolving deltas: 100% (97/97), completed with 89 local objects.
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:34:20] 227 |                         hir::LifetimeName::Name(Symbol::intern(&p.name))
[00:34:20]     |                                                                ^^^^^^^ expected str, found struct `syntax::symbol::InternedString`
