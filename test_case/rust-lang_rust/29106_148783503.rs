
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:414:48: 414:52 error: `arc1` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:414     arc0.0.write().unwrap().children.0 = Some(&arc1);
                                                                                                              ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400:32: 442:2 note: reference must be valid for the block suffix following statement 133 at 400:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:401     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:402     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:403     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:404     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:405     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410:51: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 140 at 410:50
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410     let (arc0, arc1, arc2): (ARCRW, ARCRW, ARCRW);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:411     arc0 = ARCRW::new("arcrw0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:412     arc1 = ARCRW::new("arcrw1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:413     arc2 = ARCRW::new("arcrw2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:414     arc0.0.write().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:415     arc0.0.write().unwrap().children.1 = Some(&arc2);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:415:48: 415:52 error: `arc2` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:415     arc0.0.write().unwrap().children.1 = Some(&arc2);
                                                                                                              ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400:32: 442:2 note: reference must be valid for the block suffix following statement 133 at 400:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:401     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:402     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:403     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:404     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:405     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410:51: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 140 at 410:50
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410     let (arc0, arc1, arc2): (ARCRW, ARCRW, ARCRW);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:411     arc0 = ARCRW::new("arcrw0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:412     arc1 = ARCRW::new("arcrw1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:413     arc2 = ARCRW::new("arcrw2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:414     arc0.0.write().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:415     arc0.0.write().unwrap().children.1 = Some(&arc2);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:416:48: 416:52 error: `arc0` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:416     arc2.0.write().unwrap().children.0 = Some(&arc0);
                                                                                                              ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400:32: 442:2 note: reference must be valid for the block suffix following statement 133 at 400:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:400     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:401     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:402     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:403     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:404     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:405     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410:51: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 140 at 410:50
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:410     let (arc0, arc1, arc2): (ARCRW, ARCRW, ARCRW);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:411     arc0 = ARCRW::new("arcrw0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:412     arc1 = ARCRW::new("arcrw1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:413     arc2 = ARCRW::new("arcrw2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:414     arc0.0.write().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:415     arc0.0.write().unwrap().children.1 = Some(&arc2);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:432:47: 432:51 error: `arc1` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:432     arc0.1.lock().unwrap().children.0 = Some(&arc1);
                                                                                                             ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418:32: 442:2 note: reference must be valid for the block suffix following statement 147 at 418:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:419     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:420     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:421     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:422     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:423     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428:48: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 154 at 428:47
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428     let (arc0, arc1, arc2): (ARCM, ARCM, ARCM);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:429     arc0 = ARCM::new("arcm0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:430     arc1 = ARCM::new("arcm1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:431     arc2 = ARCM::new("arcm2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:432     arc0.1.lock().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:433     arc0.1.lock().unwrap().children.1 = Some(&arc2);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:433:47: 433:51 error: `arc2` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:433     arc0.1.lock().unwrap().children.1 = Some(&arc2);
                                                                                                             ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418:32: 442:2 note: reference must be valid for the block suffix following statement 147 at 418:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:419     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:420     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:421     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:422     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:423     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428:48: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 154 at 428:47
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428     let (arc0, arc1, arc2): (ARCM, ARCM, ARCM);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:429     arc0 = ARCM::new("arcm0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:430     arc1 = ARCM::new("arcm1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:431     arc2 = ARCM::new("arcm2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:432     arc0.1.lock().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:433     arc0.1.lock().unwrap().children.1 = Some(&arc2);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:434:47: 434:51 error: `arc0` does not live long enough
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:434     arc2.1.lock().unwrap().children.0 = Some(&arc0);
                                                                                                             ^~~~
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418:32: 442:2 note: reference must be valid for the block suffix following statement 147 at 418:31...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:418     let mut c = c_orig.clone();
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:419     c.control_bits = 0b1;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:420     c.curr_mark = 110;
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:421     assert!(!c.saw_prev_marked);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:422     arc0.descend_into_self(&mut c);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:423     assert!(c.saw_prev_marked);
                                                               ...
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428:48: 442:2 note: ...but borrowed value is only valid for the block suffix following statement 154 at 428:47
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:428     let (arc0, arc1, arc2): (ARCM, ARCM, ARCM);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:429     arc0 = ARCM::new("arcm0");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:430     arc1 = ARCM::new("arcm1");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:431     arc2 = ARCM::new("arcm2");
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:432     arc0.1.lock().unwrap().children.0 = Some(&arc1);
/home/andrew/rust/src/test/run-pass/dropck_legal_cycles.rs:433     arc0.1.lock().unwrap().children.1 = Some(&arc2);
                                                               ...
