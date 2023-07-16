make
a:
    foo 2>&1 | tee $(TMPDIR)/out
    grep bar $(TMPDIR)/out
