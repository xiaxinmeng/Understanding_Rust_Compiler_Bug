
    let _ = #[track_caller] async {
        //~^ ERROR attribute should be applied to a function definition [E0739]
    };
