rust
pub fn detect(&mut self, user_agent:&str){
    //let agent = CString::new(user_agent).expect("CString::new failed");
    //let agent_raw = agent.as_ptr();

    unsafe {
        let ws = fiftyoneDegreesProviderWorksetGet(&mut self.provider);

        fiftyoneDegreesWorksetRelease(ws);
    }
}
