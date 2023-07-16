rust
pub fn detect(&mut self, user_agent:&str){
    unsafe {
        let ws = fiftyoneDegreesProviderWorksetGet(&mut self.provider);

        fiftyoneDegreesWorksetRelease(ws);
    }
}
