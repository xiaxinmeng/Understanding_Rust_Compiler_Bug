rust
// The variant is externally inhabited, the field addition doesn't change that
variant S { pub a: PubInhabited } -> variant S { pub a: PubInhabited, pub b: PubInhabited }

// The variant is externally inhabited, the field addition changes that
variant S { pub a: PubInhabited } -> variant S { pub a: PubInhabited, pub b: PubUninhabited }

---

// The variant is externally uninhabited, the field addition doesn't change that
variant S { pub a: PubUninhabited } -> variant S { pub a: PubInhabited, pub b: PubInhabited }

// The variant is externally uninhabited, the field addition doesn't change that
variant S { pub a: PubUninhabited } -> variant S { pub a: PubInhabited, pub b: PubUninhabited }
