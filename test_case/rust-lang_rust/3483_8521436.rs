
use std;
use io::*;

struct Point {x:float, y:float}

enum Shape{
   Circle(Point, float),
   Rectangle(Point, Point)
}

fn area(sh: Shape) -> float {
   match sh {
      Circle(_,size) => float::consts::pi*size*size,
      Rectangle(point1, point2) => (point2.x-point1.x)*(point2.y-point1.y)
   }
}

fn main() {
   let pt: Point = Point {x:4.5, y:5.5};
   let circle: Shape = Circle(pt,1.7);
   area(circle);

   let pi = float::consts::pi;

   if area(circle) - 1.7 * 1.7 * pi < 0.1  { io::println("OK"); }
}
