haskell
mut_scanl :: (a -> State b ()) -> b -> [a] -> [b]
mut_scanl mut_f b as = scanl f b as
    where f b a = next_b
            where ((), next_b) = runState (mut_f a) b
