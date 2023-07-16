
valgrind: m_debuginfo/storage.c:389 (vgModuleLocal_addLineInfo): Assertion 'lineno >= 0' failed.
==19583==    at 0x38028210: report_and_quit (m_libcassert.c:193)
==19583==    by 0x38028477: vgPlain_assert_fail (m_libcassert.c:267)
==19583==    by 0x3805E09A: vgModuleLocal_addLineInfo (storage.c:389)
==19583==    by 0x380AB666: vgModuleLocal_read_debuginfo_dwarf3 (readdwarf.c:770)
==19583==    by 0x3805A836: vgModuleLocal_read_elf_debug_info (readelf.c:2206)
==19583==    by 0x38051C99: vgPlain_di_notify_mmap (debuginfo.c:822)
==19583==    by 0x3806D451: vgModuleLocal_generic_PRE_sys_mmap (syswrap-generic.c:2065)
==19583==    by 0x3809E9C1: vgSysWrap_x86_linux_sys_mmap2_before (syswrap-x86-linux.c:1381)
==19583==    by 0x38069F35: vgPlain_client_syscall (syswrap-main.c:1443)
==19583==    by 0x3806638D: handle_syscall (scheduler.c:895)
==19583==    by 0x380682BD: vgPlain_scheduler (scheduler.c:1091)
==19583==    by 0x38079F60: run_a_thread_NORETURN (syswrap-linux.c:94)

sched status:
  running_tid=1

Thread 1: status = VgTs_Runnable
==19583==    at 0x4416BA3: mmap (mmap.S:65)
==19583==    by 0x4406582: _dl_map_object_from_fd (dl-load.c:1240)
==19583==    by 0x4407EE2: _dl_map_object (dl-load.c:2250)
==19583==    by 0x440CD5F: openaux (dl-deps.c:65)   
==19583==    by 0x440E7E5: _dl_catch_error (dl-error.c:178)
==19583==    by 0x440CF09: _dl_map_object_deps (dl-deps.c:247)
==19583==    by 0x4402BB6: dl_main (rtld.c:1809)
==19583==    by 0x441427D: _dl_sysdep_start (dl-sysdep.c:244)
==19583==    by 0x4404A5F: _dl_start (rtld.c:336)   
==19583==    by 0x4400856: ??? (in /lib32/ld-2.13.so)
