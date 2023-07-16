Makefile
remap_cwd_bin:
	rm -rf $(TMPDIR) && mkdir $(TMPDIR)
	$(RUSTC) reproducible-build-aux.rs
	mkdir $(TMPDIR)/test
	cp reproducible-build.rs $(TMPDIR)/test
	$(RUSTC) reproducible-build.rs --crate-type bin -C debuginfo=2 \
	  -Z remap-cwd-prefix=.
	cp $(TMPDIR)/reproducible-build $(TMPDIR)/first
	(cd $(TMPDIR)/test && \
	 $(RUSTC) reproducible-build.rs --crate-type bin -C debuginfo=2 \
	   -Z remap-cwd-prefix=.)
	cmp "$(TMPDIR)/first" "$(TMPDIR)/reproducible-build" || exit 1
