
objdump -t test_ld | grep init_array
0000000000048250 l    d  .init_array	0000000000000000              .init_array
0000000000048258 l     O .init_array	0000000000000000              __frame_dummy_init_array_entry
0000000000048250 l     O .init_array	0000000000000008              _ZN3std3sys4unix4args3imp15ARGV_INIT_ARRAY17hb13aa0f306b867eeE
0000000000048260 l       .init_array	0000000000000000              __init_array_end
0000000000048250 l       .init_array	0000000000000000              __init_array_start

objdump -t test_gold | grep init_array
000000000004a218 l     O .init_array	0000000000000000              __frame_dummy_init_array_entry
000000000004c728 l     O .init_array	0000000000000008              _ZN3std3sys4unix4args3imp15ARGV_INIT_ARRAY17hb13aa0f306b867eeE
000000000004a218 l       .init_array	0000000000000000              .hidden __init_array_start
000000000004a220 l       .init_array	0000000000000000              .hidden __init_array_end
