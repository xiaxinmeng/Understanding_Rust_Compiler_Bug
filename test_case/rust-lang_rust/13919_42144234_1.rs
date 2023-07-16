
Additionally, the semantics around DNS resolution are a little tricky here, especially in the
timeout case. The timeout being supplied is not an overall timeout, but rather only for one
connect operation, not the DNS lookup, nor the whole batch of connects (if there are many).
It's close to the best that we can do, but I think that filling this out more will require a good
deal more code (just guessing, though).
