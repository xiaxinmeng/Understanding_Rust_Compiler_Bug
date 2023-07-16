bash
mkdir mwe-asm
cd mwe-asm/
cat <<EOF > b.s
.section .mysection
.align 4
.global X
X:
    .word 42
EOF
cat <<EOF > a.s
.comm X, 4, 4

.global main
main:
    ret
EOF
as -o a.o a.s
as -o b.o b.s
cat <<EOF > link.x
ENTRY(main)

SECTIONS {
    .mysection : ALIGN(8) {
        _START_ = .;
        KEEP(*(.mysection))
        _END_ = .;
    }
}

ASSERT(_END_ != _START_, "Section empty");
EOF
# link directly
ld -o archive-no.elf a.o b.o -Tlink.x --gc-sections
# create archive and link
ar rcs libb.a b.o
ld -o archive-yes.elf a.o b.o -Tlink.x -lb -L. --gc-sections
