
$ rustc -Z time-passes a.rs
  time: 0.001	parsing
  time: 0.000	recursion limit
  time: 0.000	crate injection
  time: 0.000	plugin loading
  time: 0.000	plugin registration
  time: 0.000	pre ast expansion lint checks
    time: 0.037	expand crate
    time: 0.000	check unused macros
  time: 0.037	expansion
  time: 0.000	maybe building test harness
  time: 0.000	maybe creating a macro crate
  time: 0.000	creating allocators
  time: 0.000	AST validation
  time: 0.000	name resolution
  time: 0.000	complete gated feature checking
  time: 0.000	lowering ast -> hir
  time: 0.000	early lint checks
  time: 0.000	indexing hir
  time: 0.000	load query result cache
  time: 0.000	looking for entry point
  time: 0.000	looking for plugin registrar
  time: 0.000	loop checking
  time: 0.000	attribute checking
  time: 0.000	stability checking
