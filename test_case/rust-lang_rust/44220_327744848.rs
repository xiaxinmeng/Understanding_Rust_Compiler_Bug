rust
        let eighty_years = second * 60 * 60 * 24 * 365 * 80;
        assert_almost_eq!(a - eighty_years + eighty_years, a);
        assert_almost_eq!(a - (eighty_years * 10) + (eighty_years * 10), a);
