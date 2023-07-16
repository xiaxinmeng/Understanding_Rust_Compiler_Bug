
echo 'fn main() { assert!(true); }' | rustc - --pretty typed
echo 'fn main() { let _: [(), ..1] = [()]; }' | rustc - --pretty typed
