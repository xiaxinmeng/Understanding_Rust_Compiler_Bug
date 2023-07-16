
// Stolen from http://en.cppreference.com/w/cpp/numeric/random, we don't have integer generic parameters yet
pub type mt19937 = mersenne_twister_engine<u32, 32, 624, 397, 31, 
                             0x9908b0df, 11, 
                             0xffffffff, 7, 
                             0x9d2c5680, 15, 
                             0xefc60000, 18, 1812433253>
