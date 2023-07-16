
fn macros() {
    #macro[
        [#assert[cond],
         (#assert[cond, "%s", #stringify(cond)])],
        [#assert[cond, message, args, ...],
         {
             if !(cond) { fail #fmt("Assertion failed at %s:%u: %s",
                                    #file(), #line(),
                                    /* #stringify(cond) */
                                   #fmt(message, args, ...)) }
         }]
    ];
}

fn main() {
    #assert(true);
    #assert("123" == "4 5 6",
            "This was supposed to go wrong.");
    #assert("1" == "2");
    #assert("123" == "3 4 5", "This was supposed to go wrong");
}
