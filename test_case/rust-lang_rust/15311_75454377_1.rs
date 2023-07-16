 haskell
map fst . maximumBy (comparing snd) $ map (\x -> (x, f x)) i
