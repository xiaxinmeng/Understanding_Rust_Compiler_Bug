
struct Handle { field: () }
struct Something { handle: Handle }
fn x(handl: Handle) -> Something { 
     Something { handle: Handle } // could be both forgotten initialization and typo, and Handle can be initialized, because its interior not private in this context.
}
