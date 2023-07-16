
gcc/rust/ChangeLog:

	* resolve/rust-ast-resolve-path.cc (ResolvePath::resolve_path):
	* resolve/rust-ast-resolve-type.cc (ResolveTypeToCanonicalPath::visit):
	* resolve/rust-ast-resolve-type.h:
	* typecheck/rust-hir-type-check-path.cc (TypeCheckExpr::visit):

gcc/testsuite/ChangeLog:

	* rust/execute/torture/issue-1249.rs: New test.
