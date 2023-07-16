rust
// compile
Command::new("g++")
    .arg("-std=c++17")
    .arg("-o")
    .arg("app.o")
    .arg("main.cpp")
    .status()
    .expect("Compiling C++ error");

// timeout 10sec
let timeout = 10000;
let now: Instant = Instant::now();

// run generator
let child = Command::new("app.o")
         .stdout(Stdio::piped())
         .stderr(Stdio::piped())
         .spawn()
         .unwrap();

// output file
let stdout = Some(PathBuf::from("output.txt"));

let output_file: Option<Arc<Mutex<File>>> = match stdout {
	Some(file) => {
	    let output = File::create(stdout).unwrap();
	    Some(Arc::new(Mutex::new(output)))
	},
	_ => None,
};

// check timeout
let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || {
	for _ in 0..timeout {
	    if let Ok(Some(_)) = child.try_wait() {
		if let Ok(response) = child.wait_with_output() {
		    match output_file {
		        Some(f) => {
                            // write the output data
		            let mut file: std::sync::MutexGuard<File> = f.lock().unwrap();
		            file.write_all(&response.stdout).unwrap();
		        },
		        None => (),
		    }
		}
		return;
	    }
	    std::thread::sleep(std::time::Duration::from_millis(1));
	}
	child.kill().unwrap();
});

thread.join().unwrap();
let new_now: Instant = Instant::now();
let time: Duration = new_now.duration_since(now);

println!("{:?}", time);
