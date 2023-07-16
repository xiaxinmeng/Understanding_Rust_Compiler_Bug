
{-# LANGUAGE FlexibleInstances, UndecidableInstances #-}

class A self where foo :: self
class A self => B self where bar :: self
instance B self => A self where foo = bar

instance B Int where bar = 666

main = print (foo :: Int)
