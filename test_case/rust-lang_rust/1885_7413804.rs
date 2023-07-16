
// Old comment:
// This program hangs in the compilation phase.
// The only difference is that the closure in #macro is replaced with a block

use std;
import rand;
import io::println;

iface is_even { fn is_even() -> bool; }

impl of is_even for int { fn is_even() -> bool { self % 2 == 0 } }

fn main() {
    let rng = rand::rng();

    let gen_even = {|| (rng.next() as int) * 2};
    let gen_float = {|| rng.gen_float()};

    fn assert_even<T: is_even>(num_gen: fn() -> T) -> bool {
        let num = num_gen();
        let prop_holds = num.is_even();
        if !prop_holds {
            println(#fmt("Failure: %? is not even", num));
        }
        ret prop_holds;
    }

    fn assert_divisible(num_gen1: fn() -> float,
                        num_gen2: fn() -> float) -> bool {
        let dividend = num_gen1(),
            divisor = num_gen2();
        let prop_holds = dividend / divisor == 0f;
        if !prop_holds {
            println(#fmt("Failure: %? are not divisible", (dividend, divisor)));
        }
        ret prop_holds;
    }

                                        // Change closure to block
    #macro[[#for_all[prop, [gen, ...]], {
        let mut passed_tests = 0;
        let mut prop_holds = true;
        while passed_tests < 100 && prop_holds {
            prop_holds = prop(gen, ...);
            if prop_holds { passed_tests += 1; }
        }
        println(#fmt("Tests passed: %d", passed_tests));
    }]]; // Close block

    #for_all[assert_even, [gen_even]];
    #for_all[assert_divisible, [gen_float, gen_float]];
}
