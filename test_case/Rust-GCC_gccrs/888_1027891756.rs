
  static MatchArm create_error ()
  {
    Location locus = Location();
    return MatchArm (std::vector<std::unique_ptr<Pattern> > (), locus);
  }
