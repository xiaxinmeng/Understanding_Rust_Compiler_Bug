
$ build/mips64el-unknown-linux-gnuabi64/stage2/bin/rustc -O -C target-feature=+msa --crate-type rlib --emit asm msa.rs
$ cat msa.s
[...]
_ZN3msa3foo17ha775519a7ac61b9eE:
[...]
        ld.b    $w0, 0($5)
        ldi.b   $w1, 0
        add_a.b $w0, $w0, $w1
        st.b    $w0, 0($4)
        jr      $ra
        move    $2, $4
