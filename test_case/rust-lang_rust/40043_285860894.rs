
BOUND = PATH | LIFETIME | for<...> PATH | etc // everything that's currently supported
      | (BOUND) // new, supported in all bounds, not only trait objects
                // `('a) + (?Sized) + (for<'a> Trait<'a>) + NakedTrait + ((((TraitInFourPants))))`
