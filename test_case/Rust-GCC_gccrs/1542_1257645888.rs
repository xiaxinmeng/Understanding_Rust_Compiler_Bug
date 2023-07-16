
2022-09-24T19:09:17.3973890Z In file included from ../../../libgcc/config/unwind-dw2-fde-darwin.c:27:
2022-09-24T19:09:17.4027160Z ../../../libgcc/unwind-dw2-fde.c: In function ‘__deregister_frame_info_bases’:
2022-09-24T19:09:17.4031700Z ../../../libgcc/unwind-dw2-fde.c:287:15: error: ‘in_shutdown’ undeclared (first use in this function)
2022-09-24T19:09:17.4075590Z   287 |   gcc_assert (in_shutdown || ob);
2022-09-24T19:09:17.4128400Z       |               ^~~~~~~~~~~
2022-09-24T19:09:17.4133290Z ../../../libgcc/../gcc/tsystem.h:122:36: note: in definition of macro ‘gcc_assert’
2022-09-24T19:09:17.4176800Z   122 | #define gcc_assert(EXPR) ((void)(!(EXPR) ? abort (), 0 : 0))
2022-09-24T19:09:17.4229550Z       |                                    ^~~~
2022-09-24T19:09:17.4234830Z ../../../libgcc/unwind-dw2-fde.c:287:15: note: each undeclared identifier is reported only once for each function it appears in
2022-09-24T19:09:17.4252620Z   287 |   gcc_assert (in_shutdown || ob);
2022-09-24T19:09:17.4253380Z       |               ^~~~~~~~~~~
2022-09-24T19:09:17.4254390Z ../../../libgcc/../gcc/tsystem.h:122:36: note: in definition of macro ‘gcc_assert’
2022-09-24T19:09:17.4255280Z   122 | #define gcc_assert(EXPR) ((void)(!(EXPR) ? abort (), 0 : 0))
2022-09-24T19:09:17.4256020Z       |                                    ^~~~
2022-09-24T19:09:17.4256990Z make[2]: *** [unwind-dw2-fde-darwin.o] Error 1
