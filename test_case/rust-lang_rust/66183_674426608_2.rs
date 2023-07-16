rust
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

pub struct PlanetFacts {
    pub mass: f64,
    pub radius: f64,
}

impl std::ops::Deref for Planet {
    type Target = PlanetFacts;

    fn deref(&self) -> &Self::Target {
        match self {
            Planet::Mercury => &PlanetFacts { mass: 3.301e+23, radius: 2.440e+6 },
            Planet::Venus   => &PlanetFacts { mass: 4.868e+24, radius: 6.052e+6 },
            Planet::Earth   => &PlanetFacts { mass: 5.972e+24, radius: 6.371e+6 },
            Planet::Mars    => &PlanetFacts { mass: 6.417e+23, radius: 3.390e+6 },
            Planet::Jupiter => &PlanetFacts { mass: 1.898e+27, radius: 6.991e+7 },
            Planet::Saturn  => &PlanetFacts { mass: 5.683e+26, radius: 5.823e+7 },
            Planet::Uranus  => &PlanetFacts { mass: 8.681e+25, radius: 2.536e+7 },
            Planet::Neptune => &PlanetFacts { mass: 1.024e+26, radius: 2.462e+7 },
        }
    }
}
