 make
ifeq ($$(CFG_ENABLE_LLVM_STATIC_STDCPP),1)
LLVM_STDCPP_LOCATION_$(1) = $$(shell $$(CC_$(1)) $$(CFG_GCCISH_CFLAGS_$(1)) \
                    -print-file-name=libstdc++.a)
else
LLVM_STDCPP_LOCATION_$(1) =
endif
