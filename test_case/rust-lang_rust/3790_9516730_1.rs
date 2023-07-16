
    pure fn union(other: &Rect<T>) -> Rect<T> {   
        Rect {
            origin: Point2D(min(self.origin.x, other.origin.x),
                            min(self.origin.y, other.origin.y)),
            size:   Size2D(max(self.origin.x.add(&self.size.width),
                               other.origin.x.add(&other.size.width).sub(&min(self.origin.x, other.origin.x))),
                           max(self.origin.y.add(&self.size.height),
                               other.origin.y.add(&other.size.height).sub(&min(self.origin.y, other.origin.y))))
        }
    }
