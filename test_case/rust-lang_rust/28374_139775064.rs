
eibwen  savv: the problem is that SipHasher also defines its own "write" method, which is private
eibwen  savv: since that is an inherent method, it is preferred to a trait method, even though you can't access it
