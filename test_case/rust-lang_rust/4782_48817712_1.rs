
#[test(MutableMap)]
    fn test_len<MapImpl: MutableMap<int> + Collection<int> + Default<int>>(){
        let map = MapImpl::default();
        assert!(map.is_empty());
        assert!(map.insert(1));
        assert_eq!(map.len(),1);
        ...
    }
