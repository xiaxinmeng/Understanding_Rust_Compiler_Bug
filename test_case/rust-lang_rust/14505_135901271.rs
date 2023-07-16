
loop
{
    match write!(&mut io::stdout(), "{}\n", output)
    {
        Ok(_) => {},
        Err(e) => {
            if e.kind() == io::ErrorKind::BrokenPipe {
                break
            } else {
                panic!("My {}", e)
            }
        },
    }
