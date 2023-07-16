
fn add_two_numbers(x: usize, y: usize) -> usize {
  x + y
}

struct Options {
  add_numbers: bool,
  left: usize,
  right: usize,
}

fn do_it(options: Options) -> usize {
  if options.add_numbers {
    add_two_numbers(options.left, options.right)
  } else {
    options.left - options.right
  }
}

fn main() {
    let result = do_it(Options { add_numbers: true, left: 10, right: 3 });
    let result_2 = do_it(Options { add_numbers: false, left: 9, right: 5 });

    println!("result = {}, result_2 = {}", result, result_2);
}

