rust
        if const {
            mem::size_of::<T>() == 0
                || mem::size_of::<T>()
                    != mem::size_of::<<<I as SourceIter>::Source as AsIntoIter>::Item>()
                || mem::align_of::<T>()
                    != mem::align_of::<<<I as SourceIter>::Source as AsIntoIter>::Item>()
        } {
            // fallback to more generic implementations
            return SpecFromIterNested::from_iter(iterator);
        }
