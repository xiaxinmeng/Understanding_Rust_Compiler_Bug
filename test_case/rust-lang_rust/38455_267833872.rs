
let mut cmd = Command::new("env"); 
for (var, val) in vars { cmd.env(var, val); }
