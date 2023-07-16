rust
#[aux_enum_data]
pub enum Planet {
    Mercury = (3.301e+23, 2.440e+6),
    Venus   = (4.868e+24, 6.052e+6),
    Earth   = (5.972e+24, 6.371e+6),
    Mars    = (6.417e+23, 3.390e+6),
    Jupiter = (1.898e+27, 6.991e+7),
    Saturn  = (5.683e+26, 5.823e+7),
    Uranus  = (8.681e+25, 2.536e+7),
    Neptune = (1.024e+26, 2.462e+7),

    pub PlanetFacts {
        pub mass: f64,
        pub radius: f64,
    }
}
