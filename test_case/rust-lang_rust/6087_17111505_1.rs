 rust
// Intiate the actual shooting of rays and tracing for a given pixel
fn doTrace(s: &Scene, sp: &SceneParams, posn: &Pixel) -> Colour {

    // If antialias is on break the pixel into 4 'sub pixels'
    let subPixels = if sp.antialias {
        ~[Vec2f32::new(posn.x + 0.25, posn.y + 0.25),
          Vec2f32::new(posn.x + 0.25, posn.y + 0.75),
          Vec2f32::new(posn.x + 0.75, posn.y + 0.25),
          Vec2f32::new(posn.x + 0.75, posn.y + 0.75)]
    } else {
        ~[Vec2f32::new(posn.x, posn.y)]
    };

    // Evenly weight the colour contribution of each sub pixel
    let coef = 1.0 / (subPixels.len() as f32);

    do subPixels.foldr(Vec3f32::zero()) |cs, results| {
        let currentPixel = sp.topPixel
                            .add_v(&sp.horVec.mul_t(sp.aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(-cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives, &s.ambient, &ray, &s.camera, s.lights);

        colour.mul_t(coef).add_v(&results)
    }
}
