rust
struct InnerInt {
  f1: usize,
};

struct InnerFloat {
  f1: f32,
}

struct NestedStruct {
  f1: InnerInt,
  f2: InnerFloat
}
