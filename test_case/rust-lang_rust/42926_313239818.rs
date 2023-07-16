
<Havvy> "If you want to check for errors, call fs::metadata and check the Result for that for errors. Then call fs::Metadata::is_file on that."
<Havvy> Can I get some editorial opinion on this?
<imperio> Havvy: seems like super heavy O.o
<imperio> (super heavy way to say it to be more clear)
<Havvy> (For context, this is for Path::is_file and variants)
<projektir> Result seems like a fairly established pattern in Rust, idk if it's worth pointing out too much.
<projektir> calling "metadata" to get errors is the weird part here
<Havvy> So s/and check the result for//
<projektir> so metadata will only error if a) you have no permissions or b) path doesn't exist
<projektir> but that seems like an odd way to check those things
<Havvy> AFAICT, Path::is_file is a convenience function to call when you don't care about errors, and if you do, you need to call fs::metadata, check the error, and then call fs::Metadata::is_file on that.
<imperio> projektir: it's doc so let's be as much explicit as possible
<Havvy> Hmm...
<projektir> imperio: most docs don't seem to mention this. They just return, well, "Result"
<projektir> I don't know if docs is the place to explain what Result/Option are
<projektir> api docs*
<imperio> projektir: no no, my point was, if you want to check error, let's explain how
<imperio> but otherwise, we don't care
<imperio> no and it shouldn't
<Havvy> Hmm... "This is a convenience function that ignores errors. If you want to check errors, call fs::metadata and handle its Result. Then call fs::Metadata::is_file if it was Ok."
<projektir> Havvy: for the record that reads very odd to me. Not the docs you're writing, but that the function is used that way
<projektir> it seems very, very odd to call something called "fs::metadata" to tell that your paths are messed up lol
<projektir> I would expect "is_valid_path" or w/e
<projektir> "can I get this metadata?" can be used in the process, but that's more for sanity
<Havvy> projektir: If it helps... Path::is_file is `fs::metadata(self).map(|m| m.is_file).unwrap_or(false)`
<projektir> yeah, I see how you got there
<projektir> but isn't the point of is_file() that you don't care if there are errors? :P
<imperio> Havvy: seems better
<Havvy> projektir: Yes, but that's only after you know there's fs::Metadata::is_file. It's not possible to know which function the reader will find first.
<projektir> seems like the situation is "is_file will also return false if calling fs:metadata resulted in errors"
<Havvy> Anecdotatly, at least one person has stumbled upon Path::is_file without knowing about fs::Metadata::is_file.
<projektir> but this should be wrapped by "don't use this if you care about which exact error it was"
<projektir> yeah current doc (on stable) doesn't seem to covert this well
<projektir> it just says false on symbolic links and doesn't really warn that is_file will eat your errors
<Havvy> projektir: Which is why I'm working on that: https://github.com/rust-lang/rust/pull/42926
<projektir> so steveklabnik seems in favor of including hte internal function (I am a bit iffy on this: what if it changes? generally, you document the behavior not the inner workings I would think)
<Havvy> projektir: Even if the internal function changes, it'll still have to be correct because otherwise you are breaking code.
<projektir> yeah but then you're referencing a function that's no longer in use
<projektir> "This will return false if the internal call to fs::metadata fails, such as in the case of broken symbolic links, invalid paths, or permission problems."
<projektir> In case of broken symbolic links this will return false.
<projektir> Oh ignore that last sentence
<Havvy> Also, can just update the docs at that point.
<projektir> then link to fs::metadata and they can see it returns Result
<projektir> Havvy: yes, this doesn't really matter I'm just being on the paranoid side (I'm used to docs being forgotten)
<Havvy> Are we okay with explicitly saying there's an internal call like that?
<projektir> Havvy: well, that's my interpretation of steveklabnik's approval of "
<projektir>     Should I add a See also link to the original functions that do return Results?
<projektir> "
<projektir> That's why I brought up the question of treating this as a black box api call vs getting into what's called internally
<projektir> because if I did the former I'd never mention fs::metadata at all, I'd just say what makes it error
<projektir> s/makes it error/makes it return false
<Havvy> projektir: I'm reading it differently, as the See Also section is a different section than the parts that say why it returns false.
<projektir> I guess I'm finding that kind of oddly coincidental
<projektir> if you're just saying "see also" to say "here's another useful function you can call if you want to know more about this path" that's dif
