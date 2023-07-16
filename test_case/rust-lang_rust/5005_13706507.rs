
$ ./src/test/bench/core-set 1000000                          
core::hashmap::LinearSet
               sequential_ints 0.474796 s
                   random_ints 0.235602 s
                   delete_ints 0.208556 s
            sequential_strings 2.108495 s
                random_strings 1.340088 s
                delete_strings 1.05437 s
std::treemap::TreeSet
               sequential_ints 0.739227 s
                   random_ints 0.747073 s
                   delete_ints 0.438264 s
            sequential_strings 3.898206 s
                random_strings 3.312389 s
                delete_strings 1.680569 s
std::bitv::BitvSet
               sequential_ints 0.111517 s
                   random_ints 0.112675 s
                   delete_ints 0.08035 s
            sequential_strings 0 s
                random_strings 0 s
                delete_strings 0 s
