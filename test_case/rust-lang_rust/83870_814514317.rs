rust
   let run_server: fn(
        stream: ControlRequestStream,
        control: Arc<Mutex<dyn ControlTrait>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> =
        |stream, control| Box::pin(run_brightness_server(stream, control));
