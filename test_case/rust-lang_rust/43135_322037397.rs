rust
    #[inline]
    fn login<'a>(&'a self) -> impl Future<Item = String, Error = Error> + 'a {
        let req = self.prepare_login_request();
        result(req)
            .and_then(move |resp| check_http_status_parse_body(resp, hyper::Ok))
            .and_then(move |api_resp| {

                // check if the authentication was successful
                if &api_resp.status.reqStatus[..] == "SUCCESS" {

                    if api_resp.status.credentials.is_some() {

                        // save a copy of the token to be returned later
                        let to_return = api_resp.status.credentials.as_ref().unwrap().clone();

                        // add the data to cache
                        debug!("Login successful! Caching the response");
                        let to_store = Arc::new(api_resp);

                        // add the data to the cache
                        self.cache.insert_new(self.auth_url.clone(), to_store);

                        // return the token
                        ok(to_return)

                    } else {
                        err(Error::Auth)
                    }
                } else {
                    err(Error::Auth)
                }
            })
    }
