
dcsommer@workP620:~$ nm ~/libtest_c_abi.a | grep -A 3 ^sync_fetch_.*4
sync_fetch_and_add_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_add_4

--
sync_fetch_and_and_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_and_4

--
sync_fetch_and_max_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_max_4

--
sync_fetch_and_min_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_min_4

--
sync_fetch_and_nand_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_nand_4

--
sync_fetch_and_or_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_or_4

--
sync_fetch_and_sub_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_sub_4

--
sync_fetch_and_umax_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_umax_4

--
sync_fetch_and_umin_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_umin_4

--
sync_fetch_and_xor_4.o:
00000000 t $a.0
00000001 T __sync_fetch_and_xor_4
