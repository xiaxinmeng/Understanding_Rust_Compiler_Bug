
< Dabo> This used to work, but doesn't anymore with the latest nightly: 
              some_set.iter().collect::<Vec<_>>().connect(", ")
< Dabo> `type `collections::vec::Vec<&collections::string::String>` does not implement any 
              method in scope named `connect``
< Dabo> I have rustc 1.1.0-nightly (dc630d01e 2015-05-09) (built 2015-05-10)
