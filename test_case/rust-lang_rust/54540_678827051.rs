rust
   let results = future::join_all(peers.into_iter().map(|peer| async move {
        let span = info_span!("peer", name = peer.name.as_str());
        let result = run(&peer, keylog).instrument(span).await;
        (peer, result)
    }));
 