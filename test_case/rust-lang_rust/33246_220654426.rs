 diff
diff --git a/src/rt/miniz.c b/src/rt/miniz.c
index 2b803b0..b3a453d 100644
--- a/src/rt/miniz.c
+++ b/src/rt/miniz.c
@@ -575,7 +575,10 @@ tinfl_status tinfl_decompress(tinfl_decompressor *r, const mz_uint8 *pIn_buf_nex
       {
         mz_uint8 *p = r->m_tables[0].m_code_size; mz_uint i;
         r->m_table_sizes[0] = 288; r->m_table_sizes[1] = 32; TINFL_MEMSET(r->m_tables[1].m_code_size, 5, 32);
-        for ( i = 0; i <= 143; ++i) *p++ = 8; for ( ; i <= 255; ++i) *p++ = 9; for ( ; i <= 279; ++i) *p++ = 7; for ( ; i <= 287; ++i) *p++ = 8;
+        for ( i = 0; i <= 143; ++i) *p++ = 8;
+        for ( ; i <= 255; ++i) *p++ = 9;
+        for ( ; i <= 279; ++i) *p++ = 7;
+        for ( ; i <= 287; ++i) *p++ = 8;
       }
       else
       {
@@ -1393,7 +1396,10 @@ static MZ_FORCEINLINE void tdefl_find_match(tdefl_compressor *d, mz_uint lookahe
         if ((d->m_dict[probe_pos + match_len] == c0) && (d->m_dict[probe_pos + match_len - 1] == c1)) break;
       TDEFL_PROBE; TDEFL_PROBE; TDEFL_PROBE;
     }
-    if (!dist) break; p = s; q = d->m_dict + probe_pos; for (probe_len = 0; probe_len < max_match_len; probe_len++) if (*p++ != *q++) break;
+    if (!dist) break;
+    p = s;
+    q = d->m_dict + probe_pos;
+    for (probe_len = 0; probe_len < max_match_len; probe_len++) if (*p++ != *q++) break;
     if (probe_len > match_len)
     {
       *pMatch_dist = dist; if ((*pMatch_len = match_len = probe_len) == max_match_len) return;
