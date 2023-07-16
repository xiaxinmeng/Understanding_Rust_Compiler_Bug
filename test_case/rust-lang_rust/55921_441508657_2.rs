
3,251,820,539 ( 2.96%)      pub fn create_next_universe(&self) -> ty::UniverseIndex {
            .                   let u = self.universe.get().next_universe();
            .                   self.universe.set(u);
            .                   u
6,503,641,078 ( 5.92%)      }
