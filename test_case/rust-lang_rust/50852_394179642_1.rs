
///
fn find_barycentric(points: &Vec<Point3<f64>>, point: &Point3<f64>) -> Point3<f64> {
    let u = Vector3::new(points[2].x - points[0].x, points[1].x - points[0].x, points[0].x - point.x);
    let v = Vector3::new(points[2].y - points[0].y, points[1].y - points[0].y, points[0].y - point.y);

    let w = u.cross(&v);

    if (w.z).abs() < 1.0 {
        return Point3::new(-1.0, 1.0, 1.0);
    } else {
        return Point3::new(1.0 - (w.x + w.y) as f64 / w.z as f64, w.y as f64 / w.z as f64, w.x as f64 / w.z as f64);
    }

}
