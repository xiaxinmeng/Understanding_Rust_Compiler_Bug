C
typedef unsigned __INT32_TYPE__ size_t;

typedef struct header {
    char *name;
    size_t name_len;
    char *val;
    size_t val_len;
} header;

typedef struct header_indices {
    size_t name_a;
    size_t name_b;
    size_t val_a;
    size_t val_b;
} header_indices;

void record_header_indices(size_t bytes_ptr, const header *headers, header_indices *indices, size_t len) {
    for (size_t i = 0; i < len; i++) {
        header_indices *idxs = &indices[i];
        const header *hdr = &headers[i];
        size_t name_start = ((size_t)hdr->name) - bytes_ptr;
        size_t name_end = name_start + hdr->name_len;
        idxs->name_a = name_start;
        idxs->name_b = name_end;
        size_t val_start = ((size_t)hdr->val) - bytes_ptr;
        size_t val_end = name_start + hdr->val_len;
        idxs->val_a = name_start;
        idxs->val_b = name_end;
    }
}
