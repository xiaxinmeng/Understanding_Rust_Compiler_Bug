
BOUND = PATH | LIFETIME | for<...> PATH | etc // everything that's currently supported
      | (BOUND*S*) // new, supported in all bounds, not only trait objects
                // `('a + Trait) + 'b + Trait + ('c + Trait)`
