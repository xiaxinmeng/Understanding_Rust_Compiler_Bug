
assign-super.rs:2:    let mut x: ~[mut int] = ~[mut 3];
borrowck-assign-comp-idx.rs:4:    let mut p = ~[mut 1];
borrowck-assign-comp-idx.rs:18:    let mut p = ~[mut 1];
borrowck-assign-comp-idx.rs:28:    let mut p = ~[mut 1];
borrowck-loan-vec-content.rs:10:    let v = ~[mut 1, 2, 3];
borrowck-loan-vec-content.rs:16:    let v = ~[mut 1, 2, 3];
borrowck-loan-vec-content.rs:27:    let v = ~[mut 1, 2, 3];
borrowck-mut-slice-of-imm-vec.rs:1:fn write(v: &[mut int]) {
borrowck-mut-vec-as-imm-slice-bad.rs:7:fn has_mut_vec(+v: @~[mut int]) -> int {
borrowck-mut-vec-as-imm-slice-bad.rs:13:    assert has_mut_vec(@~[mut 1, 2, 3]) == 6;
issue-2548.rs:26:        let mut v = ~[mut];
issue-2548.rs:27:        v = move ~[mut (move res)] + v; //~ ERROR instantiating a type parameter with an incompatible type (needs `copy`, got `owned`, missing `copy`)
lub-in-args.rs:4:    let x: ~[mut int] = ~[mut 3];
mutable-huh-variance-box.rs:7:        *v = ~[mut 3]
mutable-huh-variance-deep.rs:4:    let v = ~[mut @mut ~mut ~[0]];
mutable-huh-variance-deep.rs:6:    fn f(&&v: ~[mut @mut ~mut ~[const int]]) {
mutable-huh-variance-ptr.rs:11:            *v = ~[mut 3]
mutable-huh-variance-rec.rs:7:        v.g = ~[mut 3]
mutable-huh-variance-unique.rs:7:        *v = ~[mut 3]
mutable-huh-variance-vec1.rs:4:    // and assigns a type of ~[mut ~[const int]].
mutable-huh-variance-vec1.rs:5:    let v: ~[mut ~[int]] = ~[mut ~[0]];
mutable-huh-variance-vec1.rs:7:    fn f(&&v: ~[mut ~[const int]]) {
mutable-huh-variance-vec1.rs:8:        v[0] = ~[mut 3]
mutable-huh-variance-vec2.rs:4:    // and assigns a type of ~[mut ~[const int]].
mutable-huh-variance-vec2.rs:5:    let v: ~[mut ~[mut int]] = ~[mut ~[mut 0]];
mutable-huh-variance-vec2.rs:7:    fn f(&&v: ~[mut ~[const int]]) {
mutable-huh-variance-vec3.rs:4:    // and assigns a type of ~[mut ~[const int]].
mutable-huh-variance-vec3.rs:5:    let v: ~[mut ~[mut ~[int]]] = ~[mut ~[mut ~[0]]];
mutable-huh-variance-vec3.rs:7:    fn f(&&v: ~[mut ~[mut ~[const int]]]) {
mutable-huh-variance-vec3.rs:8:        v[0][1] = ~[mut 3]
mutable-huh-variance-vec4.rs:6:    let v = ~[mut ~[0]];
mutable-huh-variance-vec4.rs:7:    let w = ~[mut ~[mut 0]];
mutable-huh-variance-vec4.rs:8:    let x = ~[mut ~[mut 0]];
mutable-huh-variance-vec4.rs:10:    fn f(&&v: ~[mut ~[int]]) {
mutable-huh-variance-vec4.rs:17:    fn h(&&v: ~[mut ~[mut int]]) {
mutable-huh-variance-vec4.rs:18:        v[0] = ~[mut 3]
mutable-huh-variance-vec4.rs:21:    fn i(&&v: ~[mut ~[const int]]) {
mutable-huh-variance-vec4.rs:22:        v[0] = ~[mut 3]
mutable-huh-variance-vec4.rs:41:    // x to have the type ~[mut ~[const int]], and thus we can safely
non-const.rs:37:    foo(~[mut 1]); //~ ERROR missing `const`
regions-infer-invariance-due-to-mutability-2.rs:2:    f: @[mut &int]
vec-add.rs:7:fn add(i: ~[int], m: ~[mut int], c: ~[const int]) {
vec-add.rs:17:   add(i + ~[mut 3],
vec-add.rs:18:       m + ~[mut 3],
vec-add.rs:19:       ~[mut 3]);
vec-add.rs:47:   add(m + ~[mut 3], //~ ERROR mismatched types
vec-add.rs:48:       m + ~[mut 3],
vec-add.rs:49:       m + ~[mut 3]);
vec-add.rs:51:   add(i + ~[mut 3],
vec-add.rs:52:       i + ~[mut 3], //~ ERROR mismatched types
vec-add.rs:53:       i + ~[mut 3]);
vec-add.rs:55:   add(c + ~[mut 3], //~ ERROR binary operation + cannot be applied
vec-add.rs:57:       c + ~[mut 3], //~ ERROR binary operation + cannot be applied
vec-add.rs:59:       ~[mut 3]);
