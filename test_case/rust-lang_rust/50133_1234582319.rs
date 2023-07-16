
// entities.rs

pub trait Tuple {
    type Tuple;

    fn to_tuple(&self) -> Self::Tuple;
}

impl Tuple for Script {
    type Tuple = (
        Option<i32>,
        String,
        String,
        Option<String>,
        bool,
        Option<i32>,
        Option<i32>,
    );

    fn to_tuple(&self) -> <Self as Tuple>::Tuple {
        (
            self.id.0.clone(),
            self.name.0.clone(),
            self.source.0.clone(),
            self.description.0.clone(),
            self.is_active,
            self.owner_id,
            self.output_id,
        )
    }
}
