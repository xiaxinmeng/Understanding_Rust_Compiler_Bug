rust
            let mut nsec_info: Option<(&Name, Vec<RecordType>)> = None;
            for key in self.records.keys() {
                match nsec_info {
                    None => nsec_info = Some((&key.name, vec![key.record_type])),
                    Some((name, ref mut vec)) if name == &key.name => vec.push(key.record_type),
                    Some((name, vec)) => {
                        // names aren't equal, create the NSEC record
                        let mut record = Record::with(name.clone(), RecordType::NSEC, ttl);
                        let rdata = NSEC::new(key.name.clone(), vec);
                        record.set_rdata(RData::NSEC(rdata));
                        records.push(record);

                        // new record...
                        nsec_info = Some((&key.name, vec![key.record_type]))
                    }
                }
            }
