
struct box_header {
    type_desc *td;
    int ref_count;
    box_header *prev;
    box_header *next;
}
