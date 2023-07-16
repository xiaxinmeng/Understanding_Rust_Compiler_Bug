haskell
class Applicative f => Alternative f where 
  empty :: f a
  (<|>) :: f a -> f a -> f a
