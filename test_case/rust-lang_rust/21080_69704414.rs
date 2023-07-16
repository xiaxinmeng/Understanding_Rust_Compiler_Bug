
pub type MonitorMsg = (TestDesc, TestResult, Vec<u8> );

unsafe impl Send for MonitorMsg {}
