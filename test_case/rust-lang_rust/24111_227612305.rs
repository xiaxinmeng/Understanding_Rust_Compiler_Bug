
// Standalone example.
struct Point { x: i32, y: i32 }

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

const ORIGIN: Point = Point::new(0, 0); // works because 0, 0 are both known at compile time 
const ORIGIN2: Point = Point::new(0, 0); // ditto

const ANOTHER: Point = ORIGIN.add(ORIGIN2); // works because ORIGIN and ORIGIN2 are both const.
{
    let x: i32 = 42;
    let y: i32 = 24;
    const SOME_POINT: Point = Point::new(x, y); // Error: x and y are not known at compile time
}
{
    const x: i32 = 42;
    const y: i32 = 24;
    const SOME_POINT: Point = Point::new(x, y); // Works x and y are both known at compile time.
}
