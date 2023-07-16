
match msg {
    Resource{ method: verb, path: uri, payload: pl, .. } => {
        let request = Request { uid: uid, method: verb, path: uri, payload: pl };
        let result  = self.stack.process(request);
        debug!("status {}", result)
    },

    PasswordAuth{ password: pw, .. } => {
        spawn(handlers::connections::password_auth(self.context.clone(),
                                                   uid, pw, self.config.services_password));
    },

    _ => { info!("unhandled message type!"); }, 
}
