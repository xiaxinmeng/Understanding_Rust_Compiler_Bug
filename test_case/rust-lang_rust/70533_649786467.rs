
check-aux-and-gui: check-aux
	$(Q)$(BOOTSTRAP) test \
        src/test/rustdoc-gui \
		$(BOOTSTRAP_ARGS)
