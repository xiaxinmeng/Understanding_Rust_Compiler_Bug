
trait Canvas {

    fn add_point(point: &int);

    fn add_points(shapes: &[int]) {
        for shapes.each |pt| {
            self.add_point(pt)
        }
    }

}
