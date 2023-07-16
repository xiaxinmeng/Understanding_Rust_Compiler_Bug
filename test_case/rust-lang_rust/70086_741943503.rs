Haskell
liftA :: Applicative f => (a -> b) -> f a -> f b
liftA3 :: Applicative f => (a -> b -> c -> d) -> f a -> f b -> f c -> f d
liftA2 :: (a -> b -> c) -> f a -> f b -> f c
