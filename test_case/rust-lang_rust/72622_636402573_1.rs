rust
impl ScaledHead {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <>
                <Group name="Grid">
                    {crate::slider!("Size", 8.0, 32.0, self.scale)}
                </Group>
                {self.head.controls(value_changed.clone())}
            </>
        )
    }
