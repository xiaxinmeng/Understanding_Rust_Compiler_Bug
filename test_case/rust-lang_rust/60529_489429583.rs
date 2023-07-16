rust
// The struct is externally inhabited, the field addition doesn't change that
struct S { a: PrivDoesntmatter } -> struct S { a: PrivDoesntmatter, b: PrivDoesntmatter }

// The struct is externally inhabited, the field addition doesn't change that
struct S { a: PrivDoesntmatter } -> struct S { a: PrivDoesntmatter, pub b: PubInhabited }

// The struct is externally inhabited, the field addition changes that
struct S { a: PrivDoesntmatter } -> struct S { a: PrivDoesntmatter, pub b: PubUninhabited }

---

// The struct is externally inhabited, the field addition doesn't change that
struct S { pub a: PubInhabited } -> struct S { pub a: PubInhabited, b: PrivDoesntmatter }

// The struct is externally inhabited, the field addition doesn't change that
struct S { pub a: PubInhabited } -> struct S { pub a: PubInhabited, pub b: PubInhabited }

// The struct is externally inhabited, the field addition changes that
struct S { pub a: PubInhabited } -> struct S { pub a: PubInhabited, pub b: PubUninhabited }

---

// The struct is externally uninhabited, the field addition doesn't change that
struct S { pub a: PubUninhabited } -> struct S { pub a: PubInhabited, b: PrivDoesntmatter }

// The struct is externally uninhabited, the field addition doesn't change that
struct S { pub a: PubUninhabited } -> struct S { pub a: PubInhabited, pub b: PubInhabited }

// The struct is externally uninhabited, the field addition doesn't change that
struct S { pub a: PubUninhabited } -> struct S { pub a: PubInhabited, pub b: PubUninhabited }
