
// Fixed size buffer: when it is at capacity push will drop the oldest element.
// Demonstrates custom data structure, custom iteration, operator overloading, struct encapsulation.
// To run execute: rustc --test ring_buffer.rs && ./ring_buffer
extern mod std;

// This contains the data that represents our ring buffer. In general only one 
// allocation occurs: when the struct is first created and buffer is allocated.
// Copying a RingBuffer will cause a heap allocation but the compiler will
// warn us on attempts to copy it implicitly.
struct RingBuffer<T: Copy>
{
    priv mut buffer: ~[T],
    priv capacity: uint,            // number of elements the buffer is able to hold (can't guarantee that vec capacity is exactly what we set it to)
    priv mut size: uint,            // number of elements with legit values in the buffer
    priv mut next: uint,            // index at which new elements land
}

// By convention structs are created with a function named after the type
// or with functions like from_foo.
fn RingBuffer<T: Copy>(capacity: uint) -> RingBuffer<T>
{
    let ring = RingBuffer {buffer: ~[], capacity: capacity, size: 0, next: 0};
    vec::reserve(ring.buffer, capacity);
    ring
}

// This is an impl which does not implement a trait: it merely provides some
// methods for our struct.
impl<T: Copy> RingBuffer<T>
{
    pure fn len() -> uint
    {
        self.size
    }

    pure fn is_empty() -> bool
    {
        self.size == 0
    }

    pure fn is_not_empty() -> bool
    {
        self.size != 0
    }

    pure fn buffer() -> uint
    {
        self.size
    }

    fn clear()
    {
        vec::truncate(self.buffer, 0);
        self.size = 0;
        self.next = 0;
    }

    fn push(element: T)
    {
        assert self.capacity > 0;

        if self.size < self.capacity
        {
            vec::push(self.buffer, element);
            self.size += 1;
        }
        else
        {
            self.buffer[self.next] = element;
        }
        self.next = (self.next + 1) % self.capacity;
    }
}

// This is how rust handles operator overloading. Here we provide
// an implementation for ops::Index which allows users to subscript
// a RingBuffer using the [] operator.
impl<T: Copy> RingBuffer<T> : ops::Index<uint, T>
{
    // The && represents an argument mode (in this case pass by pointer).
    // These are going away in rust 0.4.
    pure fn index(&&index: uint) -> T
    {
        assert index < self.size;

        if self.size < self.capacity
        {
            self.buffer[index]
        }
        else
        {
            self.buffer[(self.next + index) % self.capacity]
        }
    }
}

// Here we provide support for basic iteration. Doing this allows use of a bunch of
// handy functions from the iter module: eachi, all, any, filter_to_vec, map_to_vec, etc.
impl<T: Copy> RingBuffer<T> : BaseIter<T>
{
    // Typically iteration functions like each are called with the aid of the for keyword
    // which provides some sugar for looping: the closure used with for returns (),
    // break aborts the loop, loop skips to the start of the loop, and return returns a
    // value from the named function the loop was defined within.
    pure fn each(callback: fn(v: &T) -> bool)
    {
        let mut i = 0;
        while i < self.size
        {
            // T may be large or something that requires a heap allocation to copy.
            // So, by convention, each methods pass a borrowed pointer into the
            // closure.
            if !callback(&self[i])
            {
                break;
            }
            i += 1;
        }
    }

    pure fn size_hint() -> option::Option<uint>
    {
        option::Some(self.size)
    }
}

// Users can always use the %? format specifier to display the full details of
// structs (and any other type). But because of the way that elements wrap
// around this can be confusing. Here we provide a to_str method that shows
// the elements in the same order as they appear to users.
//
// Note that in this case we constrain T to be both a copyable type and a type
// that implements the ToStr trait. (The later allows the e.to_str() call to compile).
impl<T: Copy ToStr> RingBuffer<T> : ToStr
{
    fn to_str() -> ~str
    {
        let elements = do iter::map_to_vec(self) |e| {e.to_str()};
        fmt!("[%s]", str::connect(elements, ", "))
    }
}

#[test]
fn test_basics()
{
    // size 0
    let buffer: RingBuffer<int> = RingBuffer(0);    // rust type inference works very well, but not in this case
    assert buffer.len() == 0;

    // size 1
    let buffer = RingBuffer(1);
    assert buffer.len() == 0;

    buffer.push(2);
    assert buffer.len() == 1;
    assert buffer[0] == 2;

    buffer.push(3);
    assert buffer.len() == 1;
    assert buffer[0] == 3;

    // size 4
    let buffer = RingBuffer(4);
    assert buffer.len() == 0;

    buffer.push(1);
    assert buffer.len() == 1;
    assert buffer[0] == 1;

    buffer.push(2);
    assert buffer.len() == 2;
    assert buffer[0] == 1;
    assert buffer[1] == 2;

    buffer.push(3);
    assert buffer.len() == 3;
    assert buffer[0] == 1;
    assert buffer[1] == 2;
    assert buffer[2] == 3;

    buffer.push(4);
    assert buffer.len() == 4;
    assert buffer[0] == 1;
    assert buffer[1] == 2;
    assert buffer[2] == 3;
    assert buffer[3] == 4;

    // At this point the elements have wrapped around.
    buffer.push(5);
    assert buffer.len() == 4;
    assert buffer.buffer[0] == 5;

    // But the public API hides this from clients (and the private fields
    // can only be used within this module).
    assert buffer[0] == 2;
    assert buffer[1] == 3;
    assert buffer[2] == 4;
    assert buffer[3] == 5;
    assert buffer.to_str() == ~"[2, 3, 4, 5]";

    // clear
    buffer.clear();
    assert buffer.len() == 0;

    buffer.push(2);
    assert buffer.len() == 1;
    assert buffer[0] == 2;

    buffer.push(3);
    assert buffer.len() == 2;
    assert buffer[0] == 2;
    assert buffer[1] == 3;
}

// Rust uses a lot of functional programming idioms. One that takes some getting
// used to for imperative programmers is an avoidance of loops (loops rely on
// mutation of a loop variable which is not functional style). Instead looping is
// typically done with functions taking closures, the most common of which are: 
// each, map, filter, and fold.
#[test]
fn test_functional()
{
    let buffer: RingBuffer<int> = RingBuffer(4);
    buffer.push(1);
    buffer.push(3);
    buffer.push(5);
    buffer.push(2);

    // each calls a closure with each element
    // it is more functional than an explicit loop, but requires side effects in order to
    // do anything useful (because the closures user's give to each don't return values)
    let mut max = 0;
    for buffer.each |element|
    {
        if *element > max {max = *element}    // dereference because each returns elements by reference
    }
    assert max == 5;

    // map uses a closure to convert elements (possibly to different types)
    let odd = do iter::map_to_vec(buffer) |element| {element & 1 == 1};
    assert odd == ~[true, true, true, false];

    // filter returns elements for which the closure returns true
    let odd = do iter::filter_to_vec(buffer) |element| {element & 1 == 1};
    assert odd == ~[1, 3, 5];

    // fold uses the closure to combine elements together (possibly into a different type)
    // either forwards (foldl) or in reverse (foldr)
    let sum = do iter::foldl(buffer, 0) |current, element| {current + element};
    assert sum == 1 + 3 + 5 + 2;
}
