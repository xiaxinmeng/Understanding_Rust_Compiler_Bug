cpp
for (auto it = values.begin (); it != values.end (); ++it)
  {
     auto &value = *it;

     // mark for stripping if required
     value->accept_vis (*this);
     if (value->is_marked_for_strip ())
     {
	it = values.erase (it);
        --it;
    }
  }
