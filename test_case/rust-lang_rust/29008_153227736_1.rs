
    boost::filesystem::path p1("/a/b/c");
    boost::filesystem::path p2("/a/b/c/");
    cout << p1.filename() << "\n" << p2.filename();
