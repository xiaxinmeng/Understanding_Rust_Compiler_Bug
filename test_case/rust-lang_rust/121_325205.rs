
$ RUST_LOG=all ./non_aligned_test 2>&1
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x100180
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x100180) -> 0xbfffeca8
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x1001e0
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x1001e0) -> 0xbfffecb8
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x100200
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x100200) -> 0xbfffecc8
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x100220
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x100220) -> 0xbfffecd8
rt: 0xa0563720:0xbfffec84:            new dom 0xbfffec84
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(84) -> 0x100240
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(776) -> 0x801000
rt: 0xa0563720:0xbfffec84:            new stk 0x801000
rt: 0xa0563720:0xbfffec84:            stk limit 0x801308
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x1002a0
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x1002a0) -> 0x100278
rt: 0xa0563720:0xbfffec84:            new task 0x100240
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(16) -> 0x1002c0
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x1002d0
rt: 0xa0563720:0xbfffec84:            startup: 1 args
rt: 0xa0563720:0xbfffec84:            startup: arg[0] = './non_aligned_test'
rt: 0xa0563720:0xbfffec84:            new mem_area [0x41b0,0x4241]
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(32) -> 0x1002f0
rt: 0xa0563720:0xbfffec84:            new ptr_vec(data=0x1002f0) -> 0xbffff5f0
rt: 0xa0563720:0xbfffec84:            read abbrev: 1
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100310
rt: 0xa0563720:0xbfffec84:            read abbrev: 2
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100330
rt: 0xa0563720:0xbfffec84:            read abbrev: 3
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100350
rt: 0xa0563720:0xbfffec84:            read abbrev: 4
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100370
rt: 0xa0563720:0xbfffec84:            read abbrev: 5
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100390
rt: 0xa0563720:0xbfffec84:            read abbrev: 6
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x1003b0
rt: 0xa0563720:0xbfffec84:            read abbrev: 7
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x1003d0
rt: 0xa0563720:0xbfffec84:            read abbrev: 8
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x1003f0
rt: 0xa0563720:0xbfffec84:            read abbrev: 9
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100410
rt: 0xa0563720:0xbfffec84:            rust_dom::realloc(0x1002f0, 64) -> 0x100430
rt: 0xa0563720:0xbfffec84:            read abbrev: 10
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x1002f0
rt: 0xa0563720:0xbfffec84:            read abbrev: 11
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100470
rt: 0xa0563720:0xbfffec84:            read abbrev: 12
rt: 0xa0563720:0xbfffec84:            rust_dom::malloc(24) -> 0x100490
rt: 0xa0563720:0xbfffec84:            new mem_area [0x4000,0x41aa]
rt: 0xa0563720:0xbfffec84:            new root CU at 0x4000
rt: 0xa0563720:0xbfffec84:            CU unit length: 422
rt: 0xa0563720:0xbfffec84:            dwarf version: 2
rt: 0xa0563720:0xbfffec84:            CU abbrev off: 0
rt: 0xa0563720:0xbfffec84:            size of address: 4
rt: 0xa0563720:0xbfffec84:            crate_reader on crate: 0xbffff5cc
rt: 0xa0563720:0xbfffec84:            debug_abbrev: 0x41b0
rt: 0xa0563720:0xbfffec84:            debug_info: 0x4000
rt: 0xa0563720:0xbfffec84:            DIE <0xb> abbrev 0x1
rt: 0xa0563720:0xbfffec84:              tag 0x11, has children: 1
rt: 0xa0563720:0xbfffec84:              attr str: Rustboot prerelease (6c217d3 2010-07-23 19:37:19 -0700)
rt: 0xa0563720:0xbfffec84:              attr num: 0x2
rt: 0xa0563720:0xbfffec84:              attr str: non_aligned_test.rs
rt: 0xa0563720:0xbfffec84:              attr str: /Users/jyasskin/src/rust/fork/src/.
rt: 0xa0563720:0xbfffec84:              attr num: 0x152a
rt: 0xa0563720:0xbfffec84:              attr num: 0x1e7a
rt: 0xa0563720:0xbfffec84:              attr num: 0x1
rt: 0xa0563720:0xbfffec84:                DIE <0x89> abbrev 0x3
rt: 0xa0563720:0xbfffec84:                  tag 0x13, has children: 0
rt: 0xa0563720:0xbfffec84:                  attr num: 0x2
rt: 0xa0563720:0xbfffec84:                  attr num: 0x1
rt: 0xa0563720:0xbfffec84:                DIE <0x8c> abbrev 0x2
rt: 0xa0563720:0xbfffec84:                  tag 0x2e, has children: 1
rt: 0xa0563720:0xbfffec84:                  attr str: main
rt: 0xa0563720:0xbfffec84:                  attr num: 0x89
rt: 0xa0563720:0xbfffec84:                  attr num: 0x1530
rt: 0xa0563720:0xbfffec84:                  attr num: 0x1cd1
rt: 0xa0563720:0xbfffec84:                  attr ??:
rt: 0xa0563720:0xbfffec84:                  attr ??:
rt: 0xa0563720:0xbfffec84:                  attr num: 0x0
rt: 0xa0563720:0xbfffec84:                  attr num: 0x0
rt: 0xa0563720:0xbfffec84:                  attr num: 0x0
rt: 0xa0563720:0xbfffec84:                    DIE <0xa6> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                      tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                      attr num: 0x15e9
rt: 0xa0563720:0xbfffec84:                      attr num: 0x1cca
rt: 0xa0563720:0xbfffec84:                        DIE <0xaf> abbrev 0x6
rt: 0xa0563720:0xbfffec84:                          tag 0x13, has children: 1
rt: 0xa0563720:0xbfffec84:                          attr ??:
rt: 0xa0563720:0xbfffec84:                            DIE <0xb6> abbrev 0x7
rt: 0xa0563720:0xbfffec84:                              tag 0x12, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                            DIE <0xc1> abbrev 0x8
rt: 0xa0563720:0xbfffec84:                              tag 0xd, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr str: _0
rt: 0xa0563720:0xbfffec84:                              attr num: 0xb6
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                            DIE <0xd6> (null)
rt: 0xa0563720:0xbfffec84:                        DIE <0xd7> abbrev 0x9
rt: 0xa0563720:0xbfffec84:                          tag 0xf, has children: 0
rt: 0xa0563720:0xbfffec84:                          attr num: 0x5
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1
rt: 0xa0563720:0xbfffec84:                          attr num: 0xaf
rt: 0xa0563720:0xbfffec84:                        DIE <0xde> abbrev 0xa
rt: 0xa0563720:0xbfffec84:                          tag 0x26, has children: 0
rt: 0xa0563720:0xbfffec84:                          attr num: 0xd7
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1
rt: 0xa0563720:0xbfffec84:                        DIE <0xe4> abbrev 0x5
rt: 0xa0563720:0xbfffec84:                          tag 0x34, has children: 0
rt: 0xa0563720:0xbfffec84:                          attr str: tracker_p
rt: 0xa0563720:0xbfffec84:                          attr ??:
rt: 0xa0563720:0xbfffec84:                          attr num: 0xde
rt: 0xa0563720:0xbfffec84:                        DIE <0xf7> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                          tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1620
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1979
rt: 0xa0563720:0xbfffec84:                            DIE <0x100> abbrev 0x9
rt: 0xa0563720:0xbfffec84:                              tag 0xf, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr num: 0x4
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1
rt: 0xa0563720:0xbfffec84:                              attr num: 0xaf
rt: 0xa0563720:0xbfffec84:                            DIE <0x107> abbrev 0xa
rt: 0xa0563720:0xbfffec84:                              tag 0x26, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr num: 0x100
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1
rt: 0xa0563720:0xbfffec84:                            DIE <0x10d> abbrev 0x5
rt: 0xa0563720:0xbfffec84:                              tag 0x34, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr str: tracker
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                              attr num: 0x107
rt: 0xa0563720:0xbfffec84:                            DIE <0x11e> abbrev 0xa
rt: 0xa0563720:0xbfffec84:                              tag 0x26, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr num: 0xb6
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1
rt: 0xa0563720:0xbfffec84:                            DIE <0x124> abbrev 0x5
rt: 0xa0563720:0xbfffec84:                              tag 0x34, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr str: msg
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                              attr num: 0x11e
rt: 0xa0563720:0xbfffec84:                            DIE <0x131> (null)
rt: 0xa0563720:0xbfffec84:                        DIE <0x132> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                          tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1979
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1ad0
rt: 0xa0563720:0xbfffec84:                            DIE <0x13b> (null)
rt: 0xa0563720:0xbfffec84:                        DIE <0x13c> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                          tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1ad5
rt: 0xa0563720:0xbfffec84:                          attr num: 0x1c52
rt: 0xa0563720:0xbfffec84:                            DIE <0x145> abbrev 0xa
rt: 0xa0563720:0xbfffec84:                              tag 0x26, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr num: 0xaf
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1
rt: 0xa0563720:0xbfffec84:                            DIE <0x14b> abbrev 0x5
rt: 0xa0563720:0xbfffec84:                              tag 0x34, has children: 0
rt: 0xa0563720:0xbfffec84:                              attr str: expected
rt: 0xa0563720:0xbfffec84:                              attr ??:
rt: 0xa0563720:0xbfffec84:                              attr num: 0x145
rt: 0xa0563720:0xbfffec84:                            DIE <0x15d> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                              tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1b5d
rt: 0xa0563720:0xbfffec84:                              attr num: 0x1bf2
rt: 0xa0563720:0xbfffec84:                                DIE <0x166> (null)
rt: 0xa0563720:0xbfffec84:                            DIE <0x167> (null)
rt: 0xa0563720:0xbfffec84:                        DIE <0x168> (null)
rt: 0xa0563720:0xbfffec84:                    DIE <0x169> (null)
rt: 0xa0563720:0xbfffec84:                DIE <0x16a> abbrev 0xb
rt: 0xa0563720:0xbfffec84:                  tag 0x5, has children: 0
rt: 0xa0563720:0xbfffec84:                  attr str: tracker
rt: 0xa0563720:0xbfffec84:                  attr ??:
rt: 0xa0563720:0xbfffec84:                  attr num: 0x100
rt: 0xa0563720:0xbfffec84:                DIE <0x17a> abbrev 0xc
rt: 0xa0563720:0xbfffec84:                  tag 0x24, has children: 0
rt: 0xa0563720:0xbfffec84:                  attr str: int
rt: 0xa0563720:0xbfffec84:                  attr num: 0x5
rt: 0xa0563720:0xbfffec84:                  attr num: 0x4
rt: 0xa0563720:0xbfffec84:                DIE <0x181> abbrev 0xb
rt: 0xa0563720:0xbfffec84:                  tag 0x5, has children: 0
rt: 0xa0563720:0xbfffec84:                  attr str: order
rt: 0xa0563720:0xbfffec84:                  attr ??:
rt: 0xa0563720:0xbfffec84:                  attr num: 0x17a
rt: 0xa0563720:0xbfffec84:                DIE <0x18f> abbrev 0xb
rt: 0xa0563720:0xbfffec84:                  tag 0x5, has children: 0
rt: 0xa0563720:0xbfffec84:                  attr str: message
rt: 0xa0563720:0xbfffec84:                  attr ??:
rt: 0xa0563720:0xbfffec84:                  attr num: 0xb6
rt: 0xa0563720:0xbfffec84:                DIE <0x19f> abbrev 0x4
rt: 0xa0563720:0xbfffec84:                  tag 0xb, has children: 1
rt: 0xa0563720:0xbfffec84:                  attr num: 0x1fc0
rt: 0xa0563720:0xbfffec84:                  attr num: 0x2080
rt: 0xa0563720:0xbfffec84:                    DIE <0x1a8> (null)
rt: 0xa0563720:0xbfffec84:                DIE <0x1a9> (null)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100490)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100470)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x1002f0)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100410)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x1003f0)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x1003d0)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x1003b0)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100390)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100370)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100350)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100330)
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100310)
rt: 0xa0563720:0xbfffec84:            ~ptr_vec 0xbffff5f0, data=0x100430
rt: 0xa0563720:0xbfffec84:            rust_dom::free(0x100430)
rt: 0xa0563720:0xbfffec84:            exit-task glue 0x2da0
rt: 0xa0563720:0xbfffec84:            from spawnee 0x1530
rt: 0xa0563720:0xbfffec84:            adding task 0x100240 in state 'running' to vec 0xbfffeca8
rt: 0xa0563720:0xbfffec84:            creating timer for domain 0xbfffec84
rt: 0xa0563720:0xbfffec84:            running main-loop on domain 0xbfffec84
rt: 0xa0563720:0xbfffec84:            exit-task glue 0x2da0
rt: 0xa0563720:0xbfffec84:            activating task 0x100240, sp=0x8012ac
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_port - task: 0x100240 retpc: x1617
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall_new_port(task=0x100240, unit_sz=4)
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(44) -> 0x100470
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x1002f0
rt: 0xa0563720:0xbfffec84:                new ptr_vec(data=0x1002f0) -> 0x10047c
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x100310
rt: 0xa0563720:0xbfffec84:                new ptr_vec(data=0x100310) -> 0x10048c
rt: 0xa0563720:0xbfffec84:0x00100240:     new rust_port(task=0x100240, unit_sz=4) -> port=0x100470
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_chan - task: 0x100240 retpc: x1650
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall_new_chan(task=0x100240, port=0x100470)
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(52) -> 0x100430
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x100330
rt: 0xa0563720:0xbfffec84:                new circular_buffer(buffer_sz=32, unread=0)-> circular_buffer=0x10043c
rt: 0xa0563720:0xbfffec84:0x00100240:     new rust_chan(task=0x100240, port=0x100470) -> chan=0x100430
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x1692
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x100350
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('Message', 8) = 0x100350
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_malloc - task: 0x100240 retpc: x1dd4
rt: 0xa0563720:0xbfffec84:                upcall malloc(20, 0x0) with gc-chain head = 0x0
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(20) -> 0x100370
rt: 0xa0563720:0xbfffec84:                upcall malloc(20, 0x0) = 0x100370 with gc-chain head = 0x0
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x1711
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x100390
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('Other', 6) = 0x100390
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_malloc - task: 0x100240 retpc: x1dd4
rt: 0xa0563720:0xbfffec84:                upcall malloc(20, 0x0) with gc-chain head = 0x0
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(20) -> 0x1003b0
rt: 0xa0563720:0xbfffec84:                upcall malloc(20, 0x0) = 0x1003b0 with gc-chain head = 0x0
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_send - task: 0x100240 retpc: x2037
rt: 0xa0563720:0xbfffec84:0x00100240:     chan: 0x100430, sptr: 0x8011a4, size: 4
rt: 0xa0563720:0xbfffec84:                circular_buffer enqueue unread: 0, buffer_sz: 32, unit_sz: 4
rt: 0xb0081000:0xbfffec84:                in timer 0xbfffec18
rt: 0xa0563720:0xbfffec84:                circular_buffer pushed data at index: 0
rt: 0xa0563720:0xbfffec84:0x00100240:     === WROTE DATA ===>
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x100390)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x100390)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1842
rt: 0xa0563720:0xbfffec84:                upcall free(0x1003b0)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x1003b0)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_send - task: 0x100240 retpc: x2037
rt: 0xa0563720:0xbfffec84:0x00100240:     chan: 0x100430, sptr: 0x8011a4, size: 4
rt: 0xa0563720:0xbfffec84:                circular_buffer enqueue unread: 4, buffer_sz: 32, unit_sz: 4
rt: 0xa0563720:0xbfffec84:                circular_buffer pushed data at index: 4
rt: 0xa0563720:0xbfffec84:0x00100240:     === WROTE DATA ===>
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x190c
rt: 0xa0563720:0xbfffec84:                upcall free(0x100370)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x100370)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x100350)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x100350)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_chan - task: 0x100240 retpc: x19a6
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall_new_chan(task=0x100240, port=0x100470)
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(52) -> 0x100370
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(32) -> 0x100350
rt: 0xa0563720:0xbfffec84:                new circular_buffer(buffer_sz=32, unread=0)-> circular_buffer=0x10037c
rt: 0xa0563720:0xbfffec84:0x00100240:     new rust_chan(task=0x100240, port=0x100470) -> chan=0x100370
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x19ea
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x1004a0
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('', 1) = 0x1004a0
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_send - task: 0x100240 retpc: x1a3a
rt: 0xa0563720:0xbfffec84:0x00100240:     chan: 0x100370, sptr: 0x801238, size: 4
rt: 0xa0563720:0xbfffec84:                circular_buffer enqueue unread: 0, buffer_sz: 32, unit_sz: 4
rt: 0xa0563720:0xbfffec84:                circular_buffer pushed data at index: 0
rt: 0xa0563720:0xbfffec84:0x00100240:     === WROTE DATA ===>
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x1004a0)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x1004a0)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_recv - task: 0x100240 retpc: x1b09
rt: 0xa0563720:0xbfffec84:0x00100240:     port: 0x100470, dptr: 0x801234, size: 0x4, chan_no: 2
rt: 0xa0563720:0xbfffec84:                shifted data from index 0
rt: 0xa0563720:0xbfffec84:                circular_buffer is shrinking to 16 bytes
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x1004a0
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x100330)
rt: 0xa0563720:0xbfffec84:0x00100240:     <=== READ DATA ===
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x1b48
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x1003b0
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('', 1) = 0x1003b0
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x1003b0)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x1003b0)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_recv - task: 0x100240 retpc: x1b09
rt: 0xa0563720:0xbfffec84:0x00100240:     port: 0x100470, dptr: 0x801234, size: 0x4, chan_no: 2
rt: 0xa0563720:0xbfffec84:                shifted data from index 0
rt: 0xa0563720:0xbfffec84:0x00100240:     <=== READ DATA ===
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x1b48
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x1003b0
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('', 1) = 0x1003b0
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x1003b0)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x1003b0)
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_recv - task: 0x100240 retpc: x1b09
rt: 0xa0563720:0xbfffec84:0x00100240:     port: 0x100470, dptr: 0x801234, size: 0x4, chan_no: 2
rt: 0xa0563720:0xbfffec84:                shifted data from index 0
rt: 0xa0563720:0xbfffec84:                circular_buffer is shrinking to 16 bytes
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x1003b0
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x10034f)
non_aligned_test(71537,0xa0563720) malloc: *** error for object 0x10034f: Non-aligned pointer being freed
*** set a breakpoint in malloc_error_break to debug
rt: 0xa0563720:0xbfffec84:0x00100240:     <=== READ DATA ===
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_new_str - task: 0x100240 retpc: x1b48
rt: 0xa0563720:0xbfffec84:                rust_dom::malloc(16) -> 0x100330
rt: 0xa0563720:0xbfffec84:0x00100240:     upcall new_str('', 1) = 0x100330
rt: 0xa0563720:0xbfffec84:0x00100240: > UPCALL upcall_free - task: 0x100240 retpc: x1f2f
rt: 0xa0563720:0xbfffec84:                upcall free(0x100330)
rt: 0xa0563720:0xbfffec84:                rust_dom::free(0x100330)
Segmentation fault
$ 
