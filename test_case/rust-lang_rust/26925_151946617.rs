
module DeriveShowPrivate(IntoIter(IntoIter), f) where
data Item a = Item { itemInner :: a } deriving Show
data IntoIter a = IntoIter { inner :: Item a } deriving Show
f v = IntoIter {inner = Item {itemInner = v}}
