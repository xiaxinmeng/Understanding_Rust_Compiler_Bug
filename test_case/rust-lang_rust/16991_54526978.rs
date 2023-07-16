 diff
+typedef void *(*hoedown_realloc_callback)(void *, size_t);
+typedef void (*hoedown_free_callback)(void *);
+
 /* hoedown_buffer: character array buffer */
 struct hoedown_buffer {
        uint8_t *data;  /* actual character data */
        size_t size;    /* size of the string */
        size_t asize;   /* allocated size (0 = volatile buffer) */
        size_t unit;    /* reallocation unit size (0 = read-only buffer) */
+
+       hoedown_realloc_callback data_realloc;
+       hoedown_free_callback data_free;
+       hoedown_free_callback buffer_free;
 };
