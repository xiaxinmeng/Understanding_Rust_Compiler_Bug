rust
let a = [];
let b = [10];
let c = [11, 12];
t!(s1.write_vectored(&[IoVec::new(&a), IoVec::new(&b), IoVec::new(&c)]));

let mut buf = [0; 4];
let len = t!(s2.read(&mut buf));
// some implementations don't support writev, so we may only write the first buffer
if len == 1 {
    assert_eq!(buf, [10, 0, 0, 0]);
} else {
    assert_eq!(len, 3);
    assert_eq!(buf, [10, 11, 12, 0]);
}
