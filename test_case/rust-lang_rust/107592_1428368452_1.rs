
MSPINC=/path/to/include # https://github.com/pftbest/msp430_svd
MSPMCU=msp430g2211 # Define to desired micro to build on.
MSPDRV=rf2500 # tilib, etc. Input to mspdebug.

# GCC invocation
CC=msp430-elf-gcc
CFLAGS=-Os -mmcu=$(MSPMCU)
LDFLAGS=-mcpu=msp430 -Wl,-T$(MSPMCU).ld # Work around possible driver bug with -mcpu=msp430.
# -mmcu=$(MSPMCU) also works in place of both these options.

# Clang invocation- requires existing GCC.
# GCC_TOOLCHAIN=/path/to/gcc/toolchain # This can't be automated if not installing clang.
# CC=clang --target=msp430 --gcc-toolchain=$(GCC_TOOLCHAIN) # Or /path/to/clang if using clang's build directory.
# CFLAGS=-Os -mmcu=$(MSPMCU)
# LDFLAGS=-Wl,-T$(MSPMCU).ld # No such option -mcpu=msp430.
# -mmcu=$(MSPMCU) also works in place of providing a linker script.
