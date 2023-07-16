
    #[test]
    fn test_hashmap_with_enum_key() {
        use std::collections::HashMap;
        use json;
        #[derive(RustcEncodable, Eq, Hash, PartialEq, RustcDecodable, Debug)]
        enum Enum {
            Foo,
            #[allow(dead_code)]
            Bar,
        }
        let mut map = HashMap::new();
        map.insert(Enum::Foo, 0);
        let result = json::encode(&map).unwrap();
        assert_eq!(&result[..], r#"{"Foo":0}"#);
        let decoded: HashMap<Enum, _> = json::decode(&result).unwrap();
        assert_eq!(map, decoded);
    }
