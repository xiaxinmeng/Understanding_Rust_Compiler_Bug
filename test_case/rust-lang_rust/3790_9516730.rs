
    pure fn union(other: &Rect<T>) -> Rect<T> {   
        let upper_left = Point2D(min(self.origin.x, other.origin.x),
                                 min(self.origin.y, other.origin.y));

        let lower_right = Point2D(max(self.origin.x + self.size.width,
                                     other.origin.x + other.size.width,
                                  max(self.origin.y + self.size.height,
                                     other.origin.y + other.size.height));

        Rect {
            origin: upper_left,
            size: Size2D(lower_right.x - upper_left.x, lower_right.y - upper_left.y))
        }
    }
