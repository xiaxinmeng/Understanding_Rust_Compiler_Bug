 rust
#[crate_type="lib"];
#[crate_id="old"];

extern mod extra;

use extra::test::BenchHarness;
use std::rand::{weak_rng, Rng};

use std::cell::Cell;

#[bench]
fn vec_speed_vec2(bh: &mut BenchHarness) {
    let mut rng = weak_rng();

    let a = &Vector::new(rng.gen_vec(10).slice(0, 10));

    bh.iter(|| {
        for _ in range(0, 100)
        {
            let m = Multiplier::new(a, a);
            for i in range(0u, 10)
            {
                unsafe
                {
                    a.unsafe_set(i, m.unsafe_get(i));
                }
            }
        }

        let mut sum = 0f32;
        for i in range(0u, 10)
        {
            unsafe
            {
                sum += a.unsafe_get(i);
            }
        }
        assert!(sum != 96.0);
    })
}
pub trait VectorGet
{
    unsafe fn unsafe_get(&self, idx: uint) -> f32;
}

pub trait VectorSet
{
    unsafe fn unsafe_set(&self, idx: uint, val: f32);
}

pub struct Vector
{
    data: ~[Cell<f32>]
}

impl Vector
{
    pub fn new(data: &[f32]) -> Vector
    {
        Vector{ data: data.iter().map(|&v| Cell::new(v)).collect() }
    }
}

impl<'l>
VectorGet for
&'l Vector
{
    unsafe fn unsafe_get(&self, idx: uint) -> f32
    {
        (*self.data.as_slice().unsafe_ref(idx)).get()
    }
}

impl<'l>
Container for
&'l Vector
{
    fn len(&self) -> uint
    {
        self.data.len()
    }
}

impl<'l>
VectorSet for
Vector
{
    unsafe fn unsafe_set(&self, idx: uint, val: f32)
    {
        self.data.as_slice().unsafe_ref(idx).set(val);
    }
}

impl<'l>
Container for
Vector
{
    fn len(&self) -> uint
    {
        self.data.len()
    }
}

pub struct Multiplier<TA, TB>
{
    a: TA,
    b: TB,
}

impl<TA: Container,
     TB: Container>
Multiplier<TA, TB>
{
    pub fn new(a: TA, b: TB) -> Multiplier<TA, TB>
    {
        assert!(a.len() == b.len());
        Multiplier{ a: a, b: b }
    }
}

impl<'l,
     TA: VectorGet + Container,
     TB: VectorGet + Container>
VectorGet for
Multiplier<TA, TB>
{
    unsafe fn unsafe_get(&self, idx: uint) -> f32
    {
        self.a.unsafe_get(idx) * self.b.unsafe_get(idx)
    }
}

impl<'l,
     TA: Container,
     TB: Container>
Container for Multiplier<TA, TB>
{
    fn len(&self) -> uint
    {
        self.a.len()
    }
}
