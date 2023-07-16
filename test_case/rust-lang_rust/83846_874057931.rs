
$(RUSTC) --crate-type=lib --temps-dir=$(TMPDIR)/temp1 --out-dir=$(TMPDIR) $(TMPDIR)/lib.rs \
		& $(RUSTC) --crate-type=cdylib --temps-dir=$(TMPDIR)/temp2 --out-dir=$(TMPDIR) $(TMPDIR)/lib.rs
