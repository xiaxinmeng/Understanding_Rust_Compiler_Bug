rust
use std::any::TypeId;

struct A0;
struct A1;
struct A2;
struct A3;
struct A4;
struct A5;
struct A6;
struct A7;
struct A8;
struct A9;
struct A10;
struct A11;
struct A12;
struct A13;
struct A14;
struct A15;
struct A16;
struct A17;
struct A18;
struct A19;

pub fn is_unique_pub() -> bool {
    let t0 = TypeId::of::<A0>();
    let t1 = TypeId::of::<A1>();
    let t2 = TypeId::of::<A2>();
    let t3 = TypeId::of::<A3>();
    let t4 = TypeId::of::<A4>();
    let t5 = TypeId::of::<A5>();
    let t6 = TypeId::of::<A6>();
    let t7 = TypeId::of::<A7>();
    let t8 = TypeId::of::<A8>();
    let t9 = TypeId::of::<A9>();
    let t10 = TypeId::of::<A10>();
    let t11 = TypeId::of::<A11>();
    let t12 = TypeId::of::<A12>();
    let t13 = TypeId::of::<A13>();
    let t14 = TypeId::of::<A14>();
    let t15 = TypeId::of::<A15>();
    let t16 = TypeId::of::<A16>();
    let t17 = TypeId::of::<A17>();
    let t18 = TypeId::of::<A18>();
    let t19 = TypeId::of::<A19>();

    let b1 = t1 == t0;
    let b2 = (t2 == t0) | (t2 == t1);
    let b3 = (t3 == t0) | (t3 == t1) | (t3 == t2);
    let b4 = (t4 == t0) | (t4 == t1) | (t4 == t2) | (t4 == t3);
    let b5 = (t5 == t0) | (t5 == t1) | (t5 == t2) | (t5 == t3) | (t5 == t4);
    let b6 = (t6 == t0) | (t6 == t1) | (t6 == t2) | (t6 == t3) | (t6 == t4) | (t6 == t5);
    let b7 =
        (t7 == t0) | (t7 == t1) | (t7 == t2) | (t7 == t3) | (t7 == t4) | (t7 == t5) | (t7 == t6);
    let b8 = (t8 == t0)
        | (t8 == t1)
        | (t8 == t2)
        | (t8 == t3)
        | (t8 == t4)
        | (t8 == t5)
        | (t8 == t6)
        | (t8 == t7);
    let b9 = (t9 == t0)
        | (t9 == t1)
        | (t9 == t2)
        | (t9 == t3)
        | (t9 == t4)
        | (t9 == t5)
        | (t9 == t6)
        | (t9 == t7)
        | (t9 == t8);
    let b10 = (t10 == t0)
        | (t10 == t1)
        | (t10 == t2)
        | (t10 == t3)
        | (t10 == t4)
        | (t10 == t5)
        | (t10 == t6)
        | (t10 == t7)
        | (t10 == t8)
        | (t10 == t9);
    let b11 = (t11 == t0)
        | (t11 == t1)
        | (t11 == t2)
        | (t11 == t3)
        | (t11 == t4)
        | (t11 == t5)
        | (t11 == t6)
        | (t11 == t7)
        | (t11 == t8)
        | (t11 == t9)
        | (t11 == t10);
    let b12 = (t12 == t0)
        | (t12 == t1)
        | (t12 == t2)
        | (t12 == t3)
        | (t12 == t4)
        | (t12 == t5)
        | (t12 == t6)
        | (t12 == t7)
        | (t12 == t8)
        | (t12 == t9)
        | (t12 == t10)
        | (t12 == t11);
    let b13 = (t13 == t0)
        | (t13 == t1)
        | (t13 == t2)
        | (t13 == t3)
        | (t13 == t4)
        | (t13 == t5)
        | (t13 == t6)
        | (t13 == t7)
        | (t13 == t8)
        | (t13 == t9)
        | (t13 == t10)
        | (t13 == t11)
        | (t13 == t12);
    let b14 = (t14 == t0)
        | (t14 == t1)
        | (t14 == t2)
        | (t14 == t3)
        | (t14 == t4)
        | (t14 == t5)
        | (t14 == t6)
        | (t14 == t7)
        | (t14 == t8)
        | (t14 == t9)
        | (t14 == t10)
        | (t14 == t11)
        | (t14 == t12)
        | (t14 == t13);
    let b15 = (t15 == t0)
        | (t15 == t1)
        | (t15 == t2)
        | (t15 == t3)
        | (t15 == t4)
        | (t15 == t5)
        | (t15 == t6)
        | (t15 == t7)
        | (t15 == t8)
        | (t15 == t9)
        | (t15 == t10)
        | (t15 == t11)
        | (t15 == t12)
        | (t15 == t13)
        | (t15 == t14);
    let b16 = (t16 == t0)
        | (t16 == t1)
        | (t16 == t2)
        | (t16 == t3)
        | (t16 == t4)
        | (t16 == t5)
        | (t16 == t6)
        | (t16 == t7)
        | (t16 == t8)
        | (t16 == t9)
        | (t16 == t10)
        | (t16 == t11)
        | (t16 == t12)
        | (t16 == t13)
        | (t16 == t14)
        | (t16 == t15);
    let b17 = (t17 == t0)
        | (t17 == t1)
        | (t17 == t2)
        | (t17 == t3)
        | (t17 == t4)
        | (t17 == t5)
        | (t17 == t6)
        | (t17 == t7)
        | (t17 == t8)
        | (t17 == t9)
        | (t17 == t10)
        | (t17 == t11)
        | (t17 == t12)
        | (t17 == t13)
        | (t17 == t14)
        | (t17 == t15)
        | (t17 == t16);
    let b18 = (t18 == t0)
        | (t18 == t1)
        | (t18 == t2)
        | (t18 == t3)
        | (t18 == t4)
        | (t18 == t5)
        | (t18 == t6)
        | (t18 == t7)
        | (t18 == t8)
        | (t18 == t9)
        | (t18 == t10)
        | (t18 == t11)
        | (t18 == t12)
        | (t18 == t13)
        | (t18 == t14)
        | (t18 == t15)
        | (t18 == t16)
        | (t18 == t17);
    let b19 = (t19 == t0)
        | (t19 == t1)
        | (t19 == t2)
        | (t19 == t3)
        | (t19 == t4)
        | (t19 == t5)
        | (t19 == t6)
        | (t19 == t7)
        | (t19 == t8)
        | (t19 == t9)
        | (t19 == t10)
        | (t19 == t11)
        | (t19 == t12)
        | (t19 == t13)
        | (t19 == t14)
        | (t19 == t15)
        | (t19 == t16)
        | (t19 == t17)
        | (t19 == t18);
    b1 | b2
        | b3
        | b4
        | b5
        | b6
        | b7
        | b8
        | b9
        | b10
        | b11
        | b12
        | b13
        | b14
        | b15
        | b16
        | b17
        | b18
        | b19
}

