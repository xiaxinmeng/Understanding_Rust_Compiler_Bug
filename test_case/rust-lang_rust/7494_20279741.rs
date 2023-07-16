
ifeq "$(origin CFLAGS)" "command line"
    CFG_GCCISH_CFLAGS += $(CFLAGS)
endif
ifeq "$(origin CXXFLAGS)" "command line"
    CFG_GCCISH_CXXFLAGS += $(CXXFLAGS)
endif
