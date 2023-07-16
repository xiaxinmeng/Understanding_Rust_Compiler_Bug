rust
pub fn detect(&mut self, user_agent:&str){
    let agent = CString::new(user_agent).expect("CString::new failed");
    let agent_raw = agent.into_raw();
    forget(agent_raw); // not sure this helps this example, but i tried with it here and commented out

    unsafe {
        let cstring = CString::from_raw(agent_raw);
        drop(agent_raw); // make sure that rust doesn't try to do something with this when FFI panics

        let ws = fiftyoneDegreesProviderWorksetGet(&mut self.provider);

        fiftyoneDegreesWorksetRelease(ws);
    }
}
