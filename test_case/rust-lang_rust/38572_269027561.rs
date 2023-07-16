
---- thread::JoinHandle<T>::thread_0 stdout ----

	error: use of unstable library feature 'thread_id' (see issue #21507)

  --> <anon>:13:36

   |

13 | println!("thread id: {:?}", thread.id());

   |                                    ^^

   |

   = help: add #![feature(thread_id)] to the crate attributes to enable
