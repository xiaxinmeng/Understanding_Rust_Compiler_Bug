
type putter[K, V] = fn(K, V) -> ();

type mapper[in_t, K, V] = 
    fn(in_t, putter[K,V]);

type getter[V] = fn() -> option[V];

type reducer[K, V] = fn(K, getter[V]);

fn map_reduce[I, K, V] (vec[I] inputs,
                        mapper[I, K, V] f,
                        reducer[K, V] reduce) {

}
