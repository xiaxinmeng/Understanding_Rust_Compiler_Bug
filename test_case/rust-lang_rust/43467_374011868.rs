C
typedef struct c_void c_void;

c_void* malloc(unsigned long size);

void call_malloc() {
	malloc(1);
}
