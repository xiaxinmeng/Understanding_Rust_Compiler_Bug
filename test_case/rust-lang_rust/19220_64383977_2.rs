
struct Ref<T> { val: &T }
let ref: Ref<T>;
ref = ref;
*ref = ref.val;
**ref = *ref.val;
