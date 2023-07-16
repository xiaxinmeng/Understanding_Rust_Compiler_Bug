rust
			let (filename, mut file) = loop {
				let uuid = Uuid::new_v4().simple().to_string();
				let filename = format!("{}.{}", uuid, ImageFormat::from_image_format(fmt).as_str());
				let fpath = ::helper::filename_to_local(&filename);
				if let Ok(f) = fs::OpenOptions::new().write(true).create_new(true).open(&fpath) {
					break (filename, f);
				}
			};
			img.save(&mut file, fmt)?;
