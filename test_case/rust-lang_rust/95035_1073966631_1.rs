
this new std::sync::Mutex       avg 1.822707ms   min 1.674482ms   max 2.081187ms
current std::sync::Mutex (gnu)  avg 8.10534ms    min 7.774234ms   max 9.279657ms  
current std::sync::Mutex (musl) avg 7.08335ms    min 6.889245ms   max 7.264291ms  
parking_lot::Mutex              avg 8.138322ms   min 2.282807ms   max 10.593921ms 
spin::Mutex                     avg 2.129469ms   min 2.024791ms   max 2.266706ms 
AmdSpinlock                     avg 2.026798ms   min 1.921888ms   max 2.154696ms  
