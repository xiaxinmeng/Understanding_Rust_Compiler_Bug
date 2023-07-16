
instance (Applicative f, Num n) => Num (f n) where
    (+) = liftA2 (+)
    (*) = liftA2 (*)
    ...
