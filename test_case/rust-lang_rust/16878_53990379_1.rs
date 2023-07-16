
for (int a = 0; a < 5; ++a) {
  ofstream matches_file("matches_cpp.txt", ios::out | ios::binary);

  for (const string& auction : auctions) {
    int i = -1;
    for (const auto& ts : terms) {
      ++i;
      for (const auto& s : ts) {
        if (auction.find(s) != string::npos) {
          matches_file << i << " ";
          break;
        }
      }
    }
    matches_file << endl;
  }
}
