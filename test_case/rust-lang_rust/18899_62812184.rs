 rust
// Defining max_by as a function appears not to work because it refuses the `|n|n` closure due to some lifetime issue
trait MaxBye<A> {
    fn max_bye<B: Ord + std::fmt::Show>(&mut self, f: |&A| -> B) -> Option<A>;
}

impl <A: std::fmt::Show, T: Iterator<A>> MaxBye<A> for T {
    fn max_bye<B: Ord + std::fmt::Show>(&mut self, f: |&A| -> B) -> Option<A> {
        self.fold(None, |max: Option<(A, B)>, x| {
                for &(_,ref b) in max.iter() { println!("Current: {}, previous: {}",&x, b); }
                let x_val = f(&x);
                match max {
                    None => Some((x, x_val)),
                    Some((y, y_val)) => {
                        if x_val > y_val {
                            Some((x, x_val))
                        } else {
                            Some((y, y_val))
                        }
                    }
                }
            }
        ).map(|(x, _)| x)
    }
}

fn main() {
    let v = vec!(1u8 , 2 , 3);
    println!("{}" , v.iter().max_bye(|n|n).map(|&e|e));
}
