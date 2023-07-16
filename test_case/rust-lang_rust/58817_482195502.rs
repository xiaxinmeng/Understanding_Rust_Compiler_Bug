
    let pages : impl Iterator<Item=(&QuotedWare, Option<&QuoteGroup>, Option<&[&QuotedWare]>)> = quoted_wares_view_response.docs().map(|qw| {
      let (qg,related_qws) = if let Some(ref qg_id) = qw.quote_group_id {
        let o_qg = quote_groups_index.get(qg_id).map(|x| *x);
        let o_qws = quote_group_quoted_wares.get(qg_id).map(|x| &x[..]);
        (o_qg,o_qws)
      } else {
        (None,None)
      };
      counter += 1;
      (qw,qg,related_qws)
    });
