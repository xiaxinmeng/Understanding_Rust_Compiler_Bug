 rust
    pub fn attrs(self) -> Vec<i32> {
        [
            self.alpha_mask_size,
            self.alpha_size,
            self.bind_to_texture_rgb,
            self.bind_to_texture_rgba,
            self.blue_size,
            self.buffer_size,
            self.color_buffer_type,
            self.config_caveat,
            self.config_id,
            self.conformant,
        ]
            .into_iter()
            .flat_map(|i| i)
            .flat_map(|i| i)
            .chain(&[0])
            .cloned()
            .collect()
    }
