
#[test]
pub fn rec_bytel_timeshift_live() {
    logger::singleton(LOG_LEVEL::DEBUG, None);
    let service_comm = ServiceComm::connect(false, None).unwrap();
    let pool = ThreadPool::new(4);

    pool.execute(|| {
        let now = Utc::now().timestamp_millis();
        let recording = Recording {
            id: 0,
            epg_id: 192,
            url: String::from(""),
            backup_url: None,
            title: String::from(""),
            summary: String::from(""),
            start_time: now as u64,
            end_time: (now + 1 * 60 * 1000) as u64,
            repeat: None,
            r_occ: 0,
            status: RecStatus::PENDING,
            extras: None,
            internal_link: None,
            ext_link: None,
            duration: None,
            size: None,
            progress: None,
        };
        let license_req = LicenseRequestMessage {
            op: "".to_string(),
            user_client: String::from(""),
            user_address: String::from(""),
            services: Some(vec![Service {
                name: "".to_string(),
                infos: String::from(""),
                device: None,
                pass: None,
            }]),
            license_req: Some(LicenseRequest {
                url: String::from(""),
                device: None,
                host: None,
                lret: None,
                common: None,
                backup: None,
                extras: None,
            }),
        };
        let notifier = Arc::new(RecNotifierRef {
            notifier: Mutex::new(RefCell::new(Notifier::new())),
        });
        let recordings = RecordingInfos {
            owner: "".to_string(),
            shareable: None,
            records: vec![recording],
            license_infos: license_req,
        };
        let (recordable, save_to_db, next_recording) =
            records_manager::has_recordable(0, &recordings)
                .unwrap()
                .unwrap();
        assert_eq!(true, save_to_db);
        assert_eq!(i64::max_value(), next_recording);
        assert_eq!(recordable.recording.url, recordable.manifest.url);
        assert_eq!(
            recordable.recording.backup_url,
            Some("")
        );
        match records_manager::start_recording_th(
            &(String::from("/tmp/"), None),
            &notifier,
            service_comm,
            recordable,
            false,
            (0, 0),
        ) {
            Some(join_handle) => match join_handle.join() {
                Ok(err) => match err {
                    Err(err) => {
                        println!("{:?}", err);
                        assert!(false);
                    }
                    Ok(res) => {
                        println!("{:?}", res);
                    }
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                    assert!(false)
                }
            },
            None => {
                assert!(false);
            }
        }
    });
}
