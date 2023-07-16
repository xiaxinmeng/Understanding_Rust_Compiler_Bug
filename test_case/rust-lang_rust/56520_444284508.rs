rust
    fn extract_keys(&self, iv: &Option<[u8; shared::IV_SIZE]>) -> Result<Keys, Error> {
        let mut encryption_key = [0u8; shared::KEY_LENGTH];
        let mut hmac_key = [0u8; shared::KEY_LENGTH];

        match &Self {
            KeySource::Generate => {
            },
            KeySource::File(path) => {
            },
            KeySource::Password(password) => {
            }
        };

        Ok(Keys {
            encryption: encryption_key,
            hmac: hmac_key,
        })
    }
