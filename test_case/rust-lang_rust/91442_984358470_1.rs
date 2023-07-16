mips
000001cc <do_calc>:
     1cc:       44800000        mtc1    zero,$f0
     1d0:       46006032        c.eq.s  $f12,$f0
     1d4:       45010005        bc1t    1ec <do_calc+0x20>
     1d8:       00000000        nop
     1dc:       3c010009        lui     at,0x9
     1e0:       c42058f0        lwc1    $f0,22768(at)
     1e4:       03e00008        jr      ra
     1e8:       460c0003        div.s   $f0,$f0,$f12
     1ec:       3c010009        lui     at,0x9
     1f0:       c42059ac        lwc1    $f0,22956(at)
     1f4:       03e00008        jr      ra
     1f8:       00000000        nop
     1fc:       04170001        0x4170001
 