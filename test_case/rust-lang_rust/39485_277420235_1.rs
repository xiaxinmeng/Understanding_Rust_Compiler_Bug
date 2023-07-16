rust
pub fn index_colors<Pix>(image: &ImageBuffer<Pix, Vec<u8>>)
                         -> ImageBuffer<Luma<u8>, Vec<u8>>
where Pix: Pixel<Subpixel=u8> + 'static,
{
    let mut indices: ImageBuffer<_,Vec<_>> = loop { };
    for (pixel, idx) in image.pixels().zip(indices.pixels_mut()) {
        // failured occurred here ^^ because we were requiring that we
        // could project Pixel or Subpixel from `T_indices` (type of
        // `indices`), but the type is insufficiently constrained
        // until we reach the return below.
    }
    indices
}
