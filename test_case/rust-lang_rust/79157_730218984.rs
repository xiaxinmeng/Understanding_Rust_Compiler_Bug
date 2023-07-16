rust
if rats_nest {
    // coder refactors this section and forgets the else branch:
    if cond {
         return Z;
    }
} else {
    if let Some(_) = other {
        return Y;
    } else {
        return X;
    }
}
unreachable!("Shouldn't be able to get here due to early returns");
