 haskell
f d =
  let x = 0
  in let y = 1
     in case d of
        x -> True
        y -> False

main = putStrLn $ show (f 3)
