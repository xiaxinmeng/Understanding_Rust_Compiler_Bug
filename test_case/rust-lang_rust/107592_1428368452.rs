Makefile
include include.mk

all: enum.elf

elf-info: enum.elf
        msp430-elf-objdump -D enum.elf > enum.lst
        msp430-elf-readelf -a --wide enum.elf > enum.sym
        msp430-elf-size -A enum.elf

enum.S: enum.c
        $(CC) -c $(CFLAGS) -S -o enum.S -I$(MSPINC) enum.c

enum.o: enum.S
        $(CC) -c $(CFLAGS) -o enum.o -I$(MSPINC) enum.S

enum.elf: enum.o
        $(CC) $(LDFLAGS) -o enum.elf -L$(MSPINC) enum.o

prog: enum.elf
        mspdebug $(MSPDRV) "prog enum.elf"

clean:
        rm -rf enum.elf enum.o enum.lst enum.sym enum.S
