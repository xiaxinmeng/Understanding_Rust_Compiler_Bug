rust
pub trait Coord {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

pub trait Rect<C: Coord> {
    fn min(&self) -> C;
    fn max(&self) -> C;
}

pub trait Line<C: Coord> {
    fn start(&self) -> C;
    fn end(&self) -> C;
}

pub enum Geometry<C, R, L>
where C: Coord,
      R: Rect<C>,
      L: Line<C>
{
    Rect(R),
    Line(L),
}
