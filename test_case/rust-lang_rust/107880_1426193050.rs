rust
        for module in self.r.arenas.local_modules().iter() {
            self.finalize_resolutions_in(module);
        }
