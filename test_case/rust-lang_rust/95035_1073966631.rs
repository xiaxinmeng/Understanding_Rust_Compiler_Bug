
this new std::sync::Mutex        avg 27.645506ms  min 26.492286ms  max 28.900528ms 
current std::sync::Mutex (gnu)   avg 58.506588ms  min 45.996402ms  max 64.50891ms  
current std::sync::Mutex (musl)  avg 89.146548ms  min 85.399849ms  max 90.148675ms
parking_lot::Mutex               avg 313.074149ms min 302.646339ms max 319.321742ms
spin::Mutex                      avg 42.427277ms  min 39.371153ms  max 44.045329ms 
AmdSpinlock                      avg 44.313832ms  min 40.812916ms  max 46.40493ms  
