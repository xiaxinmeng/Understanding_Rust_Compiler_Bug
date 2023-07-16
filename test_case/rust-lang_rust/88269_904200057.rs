
$ cat Makefile
include ../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/doctests
	$(RUSTC) --crate-type rlib t.rs
	$(RUSTDOC) -Zunstable-options --test --persist-doctests $(TMPDIR)/doctests --extern t=$(TMPDIR)/libt.rlib t.rs --no-run
	$(TMPDIR)/doctests/t_rs_2_0/rust_out
	$(TMPDIR)/doctests/t_rs_8_0/rust_out

$ cat t.rs 
/// Fungle the foople.
/// 