
all: one two
        $(MAKE) last

one:
        @echo one1
        @echo one2
        @echo one3
        @echo one4
        @echo one5

two:
        @echo two1
        @echo two2
        @echo two3
        @echo two4
        @echo two5

last:
        @echo foo1
        @echo foo2
        @echo foo3
        @echo foo4
        @echo foo5
