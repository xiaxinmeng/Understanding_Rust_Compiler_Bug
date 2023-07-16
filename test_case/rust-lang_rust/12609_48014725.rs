 haskell
import Unsafe.Coerce

data Void

f :: Maybe Void -> Int
f Nothing = 0

main :: IO ()
main = putStr . show . f . unsafeCoerce $ 0
