
enum E1 { V1(E2<E1>), }
enum E2<T> { V2(E2<E1>), }
