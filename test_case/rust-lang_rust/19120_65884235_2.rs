
28336:9891  <... clone resumed> child_stack=0x7f71ebdfdfb0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID, parent_tidptr=0x7f71ebdfe9d0, tls=0x7f71ebdfe700, child_tidptr=0x7f71ebdfe9d0) = 10792
28350:10792 set_robust_list(0x7f71ebdfe9e0, 24 <unfinished ...>
28358:10792 <... set_robust_list resumed> )   = 0
28359:10792 mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f722c6cc000
28360:10792 sigaltstack({ss_sp=0x7f722c6cc000, ss_flags=0, ss_size=8192}, NULL) = 0
28361:10792 sched_getaffinity(10792, 32, {ff, 0, 0, 0}) = 32
28362:10792 clone(child_stack=0x7f71eafeafb0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID, parent_tidptr=0x7f71eafeb9d0, tls=0x7f71eafeb700, child_tidptr=0x7f71eafeb9d0) = 10793
28363:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
28366:10793 set_robust_list(0x7f71eafeb9e0, 24 <unfinished ...>
28538:10793 <... set_robust_list resumed> )   = 0
28543:10793 mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0 <unfinished ...>
28676:10793 <... mmap resumed> )              = 0x7f722c6be000
28679:10793 sigaltstack({ss_sp=0x7f722c6be000, ss_flags=0, ss_size=8192} <unfinished ...>
28683:10793 <... sigaltstack resumed> , NULL) = 0
28686:10793 sched_getaffinity(10793, 32,  <unfinished ...>
28690:10793 <... sched_getaffinity resumed> {ff, 0, 0, 0}) = 32
28696:10793 getcwd( <unfinished ...>
28699:10793 <... getcwd resumed> "/home/nodakai/src/rust-HEAD", 2048) = 28
28703:10793 getcwd( <unfinished ...>
28706:10793 <... getcwd resumed> "/home/nodakai/src/rust-HEAD", 2048) = 28
28710:10793 socket(PF_INET, SOCK_DGRAM, IPPROTO_IP <unfinished ...>
28714:10793 <... socket resumed> )            = 12
28717:10793 bind(12, {sa_family=AF_INET, sin_port=htons(9640), sin_addr=inet_addr("127.0.0.1")}, 16 <unfinished ...>
28720:10793 <... bind resumed> )              = 0
28724:10793 clone( <unfinished ...>
28729:10793 <... clone resumed> child_stack=0x7f71ebbfcfb0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID, parent_tidptr=0x7f71ebbfd9d0, tls=0x7f71ebbfd700, child_tidptr=0x7f71ebbfd9d0) = 10800
28731:10793 select(13, [12], NULL, NULL, {0, 20000} <unfinished ...>
29248:10800 set_robust_list(0x7f71ebbfd9e0, 24 <unfinished ...>
29463:10800 <... set_robust_list resumed> )   = 0
30101:10800 mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0 <unfinished ...>
30744:10800 <... mmap resumed> )              = 0x7f722c6d2000
31039:10793 <... select resumed> )            = 0 (Timeout)
31041:10793 select(13, [12], NULL, NULL, {0, 0} <unfinished ...>
31043:10793 <... select resumed> )            = 0 (Timeout)
31045:10793 sendto(12, "\0", 1, 0, {sa_family=AF_INET, sin_port=htons(9641), sin_addr=inet_addr("127.0.0.1")}, 16 <unfinished ...>
31047:10793 <... sendto resumed> )            = 1
31049:10793 recvfrom(12,  <unfinished ...>
31291:10800 sigaltstack({ss_sp=0x7f722c6d2000, ss_flags=0, ss_size=8192} <unfinished ...>
31747:10800 <... sigaltstack resumed> , NULL) = 0
32023:10800 sched_getaffinity(10800, 32,  <unfinished ...>
32039:10800 <... sched_getaffinity resumed> {ff, 0, 0, 0}) = 32
32056:10800 socket(PF_INET, SOCK_DGRAM, IPPROTO_IP <unfinished ...>
32062:10800 <... socket resumed> )            = 8
32072:10800 bind(8, {sa_family=AF_INET, sin_port=htons(9641), sin_addr=inet_addr("127.0.0.1")}, 16 <unfinished ...>
32078:10800 <... bind resumed> )              = 0
32082:10800 recvfrom(8,  <unfinished ...>
52140:10792 <... futex resumed> )             = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
52143:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
63288:10792 <... futex resumed> )             = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
63290:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
75761:10792 <... futex resumed> )             = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
75765:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
75802:10792 <... futex resumed> )             = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
75804:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
88756:10793 <... recvfrom resumed> 0x7f71eafea27f, 1, 0, 0x7f71eafea0b8, 0x7f71eafea0ac) = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
88758:10792 <... futex resumed> )             = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
88761:10792 futex(0x7f722ac2da8c, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
89145:10793 recvfrom(12,  <unfinished ...>
172341:10800 +++ killed by SIGINT +++
172342:10793 +++ killed by SIGINT +++
172343:10792 +++ killed by SIGINT +++
