` for the code fences and lines with HTML tags for `<details>`.

Regarding triage, I wouldn’t know how to tell whether or not the reproduction you have is *actually “the same”* issue as this one, so perhaps opening a new issue might be useful. I *can* reproduce it, and it’s nice (and interesting) that it’s appearing unconditionally, even though it looks similar to an incr-comp issue. Should make minimization easier though; if you open a new issue, make sure to tag it with `E-needs-mcve` :-) I don’t have time for minimizing myself today, but if no-one else picks it up, I might go for minimization within the next few days.

Let me quickly bisect your reproduction … ~~apparently it *is* a regression from around May, so feel free to add `regression-from-stable-to-stable` to a new issue, too; I will edit in bisection results once it’s complete.~~

---

*Edit1:* The “May” figure might be inaccurate, just noticed that it might have simply not ICE’d before May because it fails with `use of unstable library feature 'bool_to_option'`

*Edit2:* Fixing the unstable features, this goes a lot further back; I haven’t even confirmed it’s a regression.

*Edit3:* Probably needs some minimization to simplify testing earlier Rust versions for whether there’s a regression, I can confirm this ICE goes back at least 2 years.