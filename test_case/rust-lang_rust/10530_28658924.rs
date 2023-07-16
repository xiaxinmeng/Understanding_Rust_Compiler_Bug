 c
typedef struct {
  void *data;
  uintptr_t len;
} RustSlice;

typedef struct {
  uintptr_t len;
  uintptr_t alloc;
  uint8_t data[0];
} RustString;
