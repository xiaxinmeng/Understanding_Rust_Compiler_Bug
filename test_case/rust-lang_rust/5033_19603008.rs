
12:22 < engla> An unmentioned(?) issue so far with external iterators is traits: We
               can't replace fn each, fn each_key etc in the Map trait, because we'd
               either need to specify a concrete iterator type as return value, or add
               a type variable for each iterator method in the trait
12:24 < engla> possibly the Map trait can afford to have one type variable for an
               .iter() method and let that be all
12:27 < engla> ah, the solution should of course be associated items
