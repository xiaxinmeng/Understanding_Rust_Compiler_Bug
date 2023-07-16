 make
    $(Q)CFG_LLVM_LINKAGE_FILE=$$(LLVM_LINKAGE_PATH_$(3)) \
        $$(subst @,,$$(STAGE$(1)_T_$(2)_H_$(3))) \
        $$(RUST_LIB_FLAGS_ST$(1)) \
        -L "$$(RT_OUTPUT_DIR_$(2))" \
        -L "$$(LLVM_LIBDIR_$(2))" \
        -L "$$(dir $$(LLVM_STDCPP_LOCATION_$(2)))" \
        $$(RUSTFLAGS_$(4)) \
        --out-dir $$(@D) \
        -C extra-filename=-$$(CFG_FILENAME_EXTRA) \
        $$<
