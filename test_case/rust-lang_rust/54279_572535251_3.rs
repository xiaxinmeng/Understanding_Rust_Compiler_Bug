rust
    loop {
        subangles.sort_unstable_by_key(|ad| ad.1);
        subangles.sort_by_key(|ad| ad.0);
        let (left, right) =
            subangles.partition_dedup_by(|&mut ad1, &mut ad2| float_equals(ad1.0, ad2.0));
        if seek < left.len() {
            let (angle, distance) = left[seek];
            let coord = angle_distance_to_coord(part1_coord, angle, distance);
            return (coord, coord.x * 100 + coord.y);
        } else {
            seek -= left.len();
            subangles = right;
        }
    }
    