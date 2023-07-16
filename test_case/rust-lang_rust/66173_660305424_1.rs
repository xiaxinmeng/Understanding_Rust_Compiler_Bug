

However, as far as I know we've never observed a pattern like this in the wild, but we have observed many examples where the warning here would be a false warning because the fn just never returned an `Err` variant (most of these were the variant constructors themselves).