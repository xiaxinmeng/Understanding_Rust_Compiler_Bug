
pub fn channel_profile_type(&self) -> Option<(&str, &str)> {
        self.profile_type
            .as_ref()
            .map(|profile_type|  { (&self.channel, profile_type) })
    }

