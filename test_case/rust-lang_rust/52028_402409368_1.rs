
            .          for (region, location) in liveness_set {
            .              debug!("generate: {:#?} is live at {:#?}", region, location);
  777,018,978              let region_vid = regioncx.to_region_vid(region);
1,942,548,420              regioncx.add_live_point(region_vid, *location);
59,657,319,186  => /home/njn/moz/rust0/src/librustc_mir/borrow_check/nll/region_infer/mod.rs:rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext::add_live_point (388507821x)
            .          }
