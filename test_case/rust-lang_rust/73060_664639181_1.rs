sh
$compiler $src_file -o $o_file -march=rv32imac -mabi=ilp32 -mno-relax -Wa,-mno-relax -c -g
