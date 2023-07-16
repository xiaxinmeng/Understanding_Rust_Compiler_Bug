
 fn get_person(
        &self,
        #[path_param = "name"] name: String, // <-- formal function parameter.
        #[query_param = "limit"] limit: Option<u32>, // <-- here too.
    ) {
        ...
    }
