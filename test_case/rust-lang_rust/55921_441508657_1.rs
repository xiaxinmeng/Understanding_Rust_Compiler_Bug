
            .                   let universes: IndexVec<ty::UniverseIndex, _> = std::iter::once(ty::UniverseIndex::ROOT)
6,503,751,448 ( 5.92%)              .chain((0..canonical.max_universe.as_u32()).map(|_| self.create_next_universe()))
            .                       .collect();
