
async fn run(&mut self) {
    loop {
        match self.target_status {
            Some(_) => self.update_status().await,
            None => match &self.current_status {
                OperationStatus::PENDING => self.set_target_status(OperationStatus::EXECUTING),
                OperationStatus::EXECUTING => match self.operation.run().await {
                    Ok(()) => self.set_target_status(OperationStatus::SUCCESSFUL),
                    Err(e) => self.set_target_status(OperationStatus::FAILED(format!("{:?}", e))),
                },
                _ => return,
            },
        }
    }
}
