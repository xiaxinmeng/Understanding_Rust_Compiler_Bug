haskell
mut_scanl :: (a -> State b c) -> b -> [a] -> [c]
mut_scanl mut_f b = join . map pick_c . scanl f (b, Nothing)
    where f (b, _) x = (next_b, Just next_c)
            where (next_c, next_b) = runState (mut_f x) b
          pick_c (_, c) = maybeToList c
