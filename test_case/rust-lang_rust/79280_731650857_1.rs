makefile
ifdef VERBOSE
Q :=
BOOTSTRAP_ARGS := -v
else
Q := @
BOOTSTRAP_ARGS :=
endif

BOOTSTRAP := /opt/homebrew/opt/python@3.9/bin/python3.9 /private/tmp/rust-20201121-94992-iuj811/rustc-1.48.0-src/src/bootstrap/bootstrap.py

all:
	$(Q)$(BOOTSTRAP) build --stage 2 $(BOOTSTRAP_ARGS)
	$(Q)$(BOOTSTRAP) doc --stage 2 $(BOOTSTRAP_ARGS)

help:
	$(Q)echo 'Welcome to the rustbuild build system!'
	$(Q)echo
	$(Q)echo This makefile is a thin veneer over the ./x.py script located
	$(Q)echo in this directory. To get the full power of the build system
	$(Q)echo you can run x.py directly.
	$(Q)echo
	$(Q)echo To learn more run \`./x.py --help\`

clean:
	$(Q)$(BOOTSTRAP) clean $(BOOTSTRAP_ARGS)

rustc-stage1:
	$(Q)$(BOOTSTRAP) build --stage 1 library/test $(BOOTSTRAP_ARGS)
rustc-stage2:
	$(Q)$(BOOTSTRAP) build --stage 2 library/test $(BOOTSTRAP_ARGS)

docs: doc
doc:
	$(Q)$(BOOTSTRAP) doc --stage 2 $(BOOTSTRAP_ARGS)
nomicon:
	$(Q)$(BOOTSTRAP) doc --stage 2 src/doc/nomicon $(BOOTSTRAP_ARGS)
book:
	$(Q)$(BOOTSTRAP) doc --stage 2 src/doc/book $(BOOTSTRAP_ARGS)
standalone-docs:
	$(Q)$(BOOTSTRAP) doc --stage 2 src/doc $(BOOTSTRAP_ARGS)
check:
	$(Q)$(BOOTSTRAP) test --stage 2 $(BOOTSTRAP_ARGS)
check-aux:
	$(Q)$(BOOTSTRAP) test --stage 2 \
		src/tools/cargo \
		src/tools/cargotest \
		$(BOOTSTRAP_ARGS)
check-bootstrap:
	$(Q)/opt/homebrew/opt/python@3.9/bin/python3.9 /private/tmp/rust-20201121-94992-iuj811/rustc-1.48.0-src/src/bootstrap/bootstrap_test.py
dist:
	$(Q)$(BOOTSTRAP) dist $(BOOTSTRAP_ARGS)
distcheck:
	$(Q)$(BOOTSTRAP) dist $(BOOTSTRAP_ARGS)
	$(Q)$(BOOTSTRAP) test --stage 2 distcheck $(BOOTSTRAP_ARGS)
install:
	$(Q)$(BOOTSTRAP) install $(BOOTSTRAP_ARGS)
tidy:
	$(Q)$(BOOTSTRAP) test --stage 2 src/tools/tidy $(BOOTSTRAP_ARGS)
prepare:
	$(Q)$(BOOTSTRAP) build --stage 2 nonexistent/path/to/trigger/cargo/metadata

check-stage2-T-arm-linux-androideabi-H-x86_64-unknown-linux-gnu:
	$(Q)$(BOOTSTRAP) test --stage 2 --target arm-linux-androideabi
check-stage2-T-x86_64-unknown-linux-musl-H-x86_64-unknown-linux-gnu:
	$(Q)$(BOOTSTRAP) test --stage 2 --target x86_64-unknown-linux-musl

TESTS_IN_2 := \
	src/test/ui \
	src/test/compile-fail \
	src/tools/linkchecker

ci-subset-1:
	$(Q)$(BOOTSTRAP) test --stage 2 $(TESTS_IN_2:%=--exclude %)
ci-subset-2:
	$(Q)$(BOOTSTRAP) test --stage 2 $(TESTS_IN_2)

TESTS_IN_MINGW_2 := \
	src/test/ui \
	src/test/compile-fail

ci-mingw-subset-1:
	$(Q)$(BOOTSTRAP) test --stage 2 $(TESTS_IN_MINGW_2:%=--exclude %)
ci-mingw-subset-2:
	$(Q)$(BOOTSTRAP) test --stage 2 $(TESTS_IN_MINGW_2)


.PHONY: dist
