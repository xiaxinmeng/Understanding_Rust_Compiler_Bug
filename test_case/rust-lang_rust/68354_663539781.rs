rust
#[derive(PartialEq, Eq)]
struct Finished {}

#[derive(PartialEq, Eq)]
struct Processing {
    status: ProcessStatus,
}

#[derive(PartialEq, Eq)]
enum ProcessStatus {
    One,
    Two,
    Three,
}

#[derive(PartialEq, Eq)]
enum Status {
    Finished(Finished),
    Processing(Processing),
}

fn check_result(_url: &str) -> Status {
    // fetch status from some server
    Status::Processing(Processing {
        status: ProcessStatus::One,
    })
}

fn wait_for_result(url: &str) -> Finished {
    let mut previous_status = None;
    loop {
        match check_result(url) {
            Status::Finished(f) => return f,
            Status::Processing(p) => {
                match (&mut previous_status, p.status) {
                    (None, status) => previous_status = Some(status), // first status
                    (Some(previous), status) if *previous == status => {} // no change, ignore
                    (Some(previous), status) => { // error!
                        // new status
                        *previous = status;
                    }
                }
            }
        }
    }
}
