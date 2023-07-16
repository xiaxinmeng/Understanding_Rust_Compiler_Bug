 C
void foo() {}

struct Pointers {
  void (*fun)();
};

static struct Pointers pointers = {
  foo
};

void call(struct Pointers* pointers) {
  pointers->fun();
}

void main() {
  call(&pointers);
}
