 make
PKG_FILES := \
    $(S)COPYRIGHT \
    $(S)LICENSE-APACHE \
    $(S)LICENSE-MIT \
    $(S)AUTHORS.txt \
    $(S)CONTRIBUTING.md \
    $(S)README.md \
    $(S)RELEASES.txt \
    $(S)configure $(S)Makefile.in \
    $(S)man \
    $(S)doc \
    $(addprefix $(S)src/, \
      README.md \
      compiletest \
      driver \
      etc \
      $(foreach crate,$(CRATES),lib$(crate)) \
      rt \
      rustllvm \
      snapshots.txt \
      test) \
    $(PKG_GITMODULES) \
    $(filter-out Makefile config.stamp config.mk, \
                 $(MKFILE_DEPS))
