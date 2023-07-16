rust
#[unstable(feature = "transmutability", issue = "99571")]
#[rustc_const_unstable(feature = "transmutability", issue = "99571")]
impl const core::ops::Add for Assume {
    type Output = Assume;

    fn add(self, rhs: Assume) -> Assume {
        Assume {
            alignment: self.alignment || rhs.alignment,
            lifetimes: self.lifetimes || rhs.lifetimes,
            safety: self.safety || rhs.safety,
            validity: self.validity || rhs.validity,
        }
    }
}

#[unstable(feature = "transmutability", issue = "99571")]
#[rustc_const_unstable(feature = "transmutability", issue = "99571")]
impl const core::ops::Sub for Assume {
    type Output = Assume;

    fn sub(self, rhs: Assume) -> Assume {
        Assume {
            alignment: self.alignment && !rhs.alignment,
            lifetimes: self.lifetimes && !rhs.lifetimes,
            safety: self.safety && !rhs.safety,
            validity: self.validity && !rhs.validity,
        }
    }
}
