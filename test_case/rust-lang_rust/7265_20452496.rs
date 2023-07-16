
313         let incoming_streams_cell = Cell::new(self.incoming_streams.clone());
314 
315         let incoming_streams_cell = Cell::new(incoming_streams_cell.take());
