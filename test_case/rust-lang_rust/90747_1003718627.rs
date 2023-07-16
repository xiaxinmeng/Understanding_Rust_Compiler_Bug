rust
    fn groups(
        &mut self,
        #[cfg(not(target_os = "vxworks"))] groups: &[u32],
        #[cfg(target_os = "vxworks")] groups: &[u16],
    ) -> &mut process::Command;
