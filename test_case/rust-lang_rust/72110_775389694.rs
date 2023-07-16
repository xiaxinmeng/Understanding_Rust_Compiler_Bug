
It turns out that, whilst the MacOS linker accepts the -s and -S shorthand arguments, they are obsolete and ignored (they generate warnings rather than the fatal errors produced with the longhand arguments).

The MacOS linker no longer provides any ability to strip its output. Instead, Apple provide a strip utility that can be run on a resulting binary—but caveat that it is designed for the output from Apple compilers and may not work with the layouts produced by other compilers. I've not had any issues using it with rust binaries, however.
