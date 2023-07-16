
let date: tiberius::time::Time = unsafe { std::mem::transmute(tiberius::time::time::Time::from_hms_micro(hour, minute, second, microsecond)?) };
