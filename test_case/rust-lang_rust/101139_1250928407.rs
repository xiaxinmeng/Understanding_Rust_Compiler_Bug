
ConstToPat::recur needs Cx
- called by ConstToPat::to_pat
  - called by PatCtxt::const_to_pat
    - PatCtxt::new
      - called by MatchVisitor::lower_pattern
        - MatchVisitor created by check_match
          - called by thir::pattern:::check_match
            - which is a provider method, and thus can't be given a Cx
