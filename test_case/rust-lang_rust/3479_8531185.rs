 rust
fn main(){
    //First, we read the input line
    let my_number_string = (io::stdin() as io::ReaderUtil).read_line();
    io::println(my_number_string);
    //Then, we try to turn it into an int
    let maybe_my_int = int::from_str(my_number_string);

    if (option::is_some(maybe_my_int)) {
        match option::unwrap(maybe_my_int) {
            0       =>  io::println("zero"),
            1|2     =>  io::println("one or two"),
            3..10   =>  io::println("three to ten"),
            _       =>  io::println("something else")
        }
    } else {
        io::println("Not a number!");
    }
}
