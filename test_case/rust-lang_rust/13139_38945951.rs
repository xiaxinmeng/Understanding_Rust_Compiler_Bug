
enum sty<T> { ty_tup(~[T]), ... } 
struct normal_type { ... sty<normal_type> ... }
struct inference_type { ... sty<inference_type> ... }
