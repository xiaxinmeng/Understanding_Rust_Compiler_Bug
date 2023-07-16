 diff
@@ -119,6 +123,7 @@ struct hoedown_renderer {
        int (*strikethrough)(hoedown_buffer *ob, const hoedown_buffer *text, void *opaque);
        int (*superscript)(hoedown_buffer *ob, const hoedown_buffer *text, void *opaque);
        int (*footnote_ref)(hoedown_buffer *ob, unsigned int num, void *opaque);
+       int (*math)(hoedown_buffer *ob, const hoedown_buffer *text, int displaymode, void *opaque);

        /* low level callbacks - NULL copies input directly into the output */
