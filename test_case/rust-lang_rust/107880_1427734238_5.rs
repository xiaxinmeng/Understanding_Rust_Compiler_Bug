
    #[aggregate]
    fn sum<ST: Foldable>(expr: ST) -> ST::Sum;
}
