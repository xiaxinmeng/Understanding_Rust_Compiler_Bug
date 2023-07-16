rust
fn hash_item(&self) -> String {
        println!("Start hash");
        let bytes = self.bytes();
        println!("BYTES: {:?}", bytes);
        let lyra2res = sum(bytes.clone());
        println!(
            "lyra2 finished: {}",
            bs58::encode(lyra2res.clone()).into_string()
        );
        let mut sk: u64 = 0;
        for byte in bytes.iter() {
            sk += *byte as u64;
        }

        let password = lyra2res;
        let salt = sk.to_string();
        let config = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 5,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };
        println!("Starting argon");
        let hash: String = bs58::encode(
            argon2::hash_encoded(&password, salt.as_bytes(), &config)
                .unwrap_or("".to_string())
                .as_bytes(),
        )
        .into_string();
        return hash;
    }
