
diff --git a/coregrind/m_main.c b/coregrind/m_main.c
index f3a0d1c27..56f9c6cbf 100644
--- a/coregrind/m_main.c
+++ b/coregrind/m_main.c
@@ -3843,6 +3843,13 @@ UWord voucher_mach_msg_set ( UWord arg1 )
 #endif
 
 
+Word VG_(get_usrstack)(void)
+{
+   return VG_PGROUNDDN(the_iicii.clstack_end - the_iifii.clstack_max_size);
+}
+
+
+
 /*--------------------------------------------------------------------*/
 /*--- end                                                          ---*/
 /*--------------------------------------------------------------------*/
diff --git a/coregrind/m_syswrap/syswrap-freebsd.c b/coregrind/m_syswrap/syswrap-freebsd.c
index 318721f62..b9508a4d4 100644
--- a/coregrind/m_syswrap/syswrap-freebsd.c
+++ b/coregrind/m_syswrap/syswrap-freebsd.c
@@ -1983,6 +1983,19 @@ PRE(sys___sysctl)
       }
    }
 
+   if (SARG2 >= 2 && ML_(safe_to_deref)(name, 2*sizeof(int))) {
+      if (name[0] == 1 && name[1] == 33) {
+         // kern.userstack
+         Word tmp = VG_(get_usrstack)();
+         size_t* out = (size_t*)ARG3;
+         size_t* outlen = (size_t*)ARG4;
+         *out = tmp;
+         *outlen = sizeof(size_t);
+         SET_STATUS_Success(0);
+      }
+   }
+
+
    PRE_REG_READ6(int, "__sysctl", int *, name, vki_u_int32_t, namelen, void *, oldp,
                  vki_size_t *, oldlenp, void *, newp, vki_size_t, newlen);
 
diff --git a/coregrind/pub_core_aspacemgr.h b/coregrind/pub_core_aspacemgr.h
index 0f34782d3..cf25699ca 100644
--- a/coregrind/pub_core_aspacemgr.h
+++ b/coregrind/pub_core_aspacemgr.h
@@ -384,6 +384,9 @@ extern Bool VG_(am_search_for_new_segment)(Addr *start, SizeT *size,
                                            UInt *prot);
 #endif
 
+extern Word VG_(get_usrstack)(void);
+
+
 #endif   // __PUB_CORE_ASPACEMGR_H
 
 /*--------------------------------------------------------------------*/
