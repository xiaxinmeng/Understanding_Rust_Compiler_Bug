rust
pickfire: Should we have join on iterator so that we could do the following and maybe have a bit of optimization without creating a Vec just to join an Iterator. Instead of ["hello", "world"].iter().collect::<Vec<_>>().join(" ") one could do ["hello", "world"].iter().join(" ")?

A rough idea on how it should look like:

pub trait Iterator {
    fn join<Separator>(self, sep: Separator) -> <[Self::Item] as Join<Separator>>::Output
    where
        [Self::Item]: Join<Separator>,
    {
        // ...
    }
}

I was surprised at first that I am not able to join into String from Iterator, I thought since Iterator may have a similar API to slice it would have join, I need to search to be able to know that I first need to get a Vec<String> to only be able to join, I wonder if it would be faster to join without first creating a Vec?

Steven Fackler: This has been something I've wished for as well, but I think it'd be better to separate it from the existing slice::Join trait so it doesn't have to buffer up the components first at all

cuviper: FWIW, itertools does have this already: https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.join

cuviper: format and format_with are neat too

pickfire: Does it requires a FCP to move it to core? This does seemed like something quite common, not sure if it is good to keep it on itertools. Also, I wouldn't suggest the one on itertools, I don't think we should only be looking on String, PathBuf could be the output as well.

pickfire: As for me, format and format_with does not seemed to be as useful. I don't quite sure why I would need that, I think dbg!() could do that better.

XAMPPRocky: @pickfire I'm not on the libs team but generally single method additions only require a PR.

cuviper: Note that they're defined in different crates, core::iter::Iterator vs. alloc::slice::Join

cuviper: I think coherence would prevent moving Join to core, since the impls need to be in alloc to refer to Vec and String

cuviper: I found a related issue: https://github.com/rust-lang/rust/issues/22754
(connect was later deprecated / renamed to join)

pickfire: Maybe let me try adding it to itertools first and then send a PR to libs if that works, sounds like it needs an RFC from the comment.

pickfire: I did some tests on this, it generates the same output as assembly except it requires writing fewer types.

#![feature(slice_concat_trait)]

use std::iter::FromIterator;
use std::slice::Join;

pub fn collect<T, B>(iter: impl Iterator<Item = T>) -> B
where
    B: FromIterator<T>,
{
    iter.collect()
}

pub fn join<T, Separator>(iter: impl Iterator<Item = T>, sep: Separator) -> <[T] as Join<Separator>>::Output
where
    [T]: Join<Separator>,
{
    // <[S] as std::slice::Join<&str>>
    // <[V] as std::slice::Join<&T>>
    // <[V] as std::slice::Join<&[T]>>
    Join::join(iter.collect::<Vec<T>>().as_slice(), sep)
}

#[inline(never)]
pub fn test_collect<'a>(mut iter: impl Iterator<Item = &'a str>) -> String {
    let v: Vec<_> = iter.collect();
    v.as_slice().join(" ")
}

#[inline(never)]
pub fn test_join<'a>(mut iter: impl Iterator<Item = &'a str>) -> String {
    join(iter, " ")
}

#[inline(never)]
pub fn test_manual<'a>(mut iter: impl Iterator<Item = &'a str>) -> String {
    let sep = " ";

    let mut buf = match iter.next() {
        Some(buf) => String::from(buf),
        None => return String::new(),
    };
    iter.for_each(|s| {
        buf.push_str(sep);
        buf.push_str(s);
    });
    buf
}

fn main() {
    assert_eq!(test_join(["a", "b"].iter().copied()), "a b".to_string());
    assert_eq!(test_collect(["a", "b"].iter().copied()), "a b".to_string());
    assert_eq!(test_manual(["a", "b"].iter().copied()), "a b".to_string());
}

In which case, rather than writing

let s = iter.collect::<Vec<_>>().join(" ");

One can just write the following with the same generated assembly

let s = iter.join(" ");

Current limitations are that Join API is currently too limited, it implements quite less stuff, I wish it could use AsRef so that we don't need to do conversion on the iterator level.

I wonder if we could do optimization such as not needing to build a Vec first and directly construct the output but I haven't figure out a way that it could be faster yet, probably performance could improve. CC @lzutao

However, this is different from the one provided in itertools, itertools already have join but that is only limited to String concatenation, this version is just Iterator with Join. What do you all think?

lzutao: I am not familiar with these APIs. You probably want to ping people like @Amanieu .

pickfire: @lzutao No, I was discussing the performance. I was wondering if we could directly construct the output from join without having a separate vec first to make it faster.

pickfire: @Amanieu Do I need to do an RFC for this?

lzutao: You could look at Zip trait and its module to get more inspiration.
Especially TrustedRandomAccess trait and its super trait ExactSizeIterator.

Amanieu: I don't think an RFC is needed.

pickfire: I don't understand how is ExactSizeIterator is useful here since we need to get the length of the elements in the vector plus N times the length of the separator? I wonder if we can get the total length mentioned while copying the data so we don't need to iterate two times.

pickfire: By the way, Box<[T]>'s FromIterator looks way less efficient than Vec<T> when I looked into the generated code, I think I need to look into improving that while working on this.

pickfire: @Amanieu Does it need a tracking issue?

cuviper: Box<[T]> just collects a Vec<T> and converts -- the only way that could be less efficient is in the implied shrink

cuviper: new unstable APIs always need a tracking issue, but I usually wait until I have a feeling whether it's going to be accepted at all

pickfire: @cuviper I understand, but the assembly generated is 2x more (96 -> 184). https://godbolt.org/z/c9ffTT I will be opening an issue for this.

cuviper: hmm, some panic handling for "Tried to shrink to a larger capacity"

cuviper: (which should be statically unreachable)

pickfire: Is that even possible?

cuviper: no, but I guess it isn't seeing that len<=capacity

pickfire: @cuviper https://github.com/rust-lang/rust/issues/75636

cuviper: maybe it should be guarded like if self.len() < self.capacity() { self.shrink_to_fit(); } -- so the compiler may see that it's never greater

cuviper: @pickfire still, that should all be cold, not affecting real performance, except maybe icache pressure

pickfire: icache pressure?

cuviper: instruction cache memory

cuviper: pressure -> unused instructions still taking up some space in that cache, leaving less room for useful stuff

cuviper: but that's very much something that should be measured before it is targeted for optimization

pickfire: This issue https://github.com/rust-lang/rust/issues/75638

pickfire: Wow, my first time opening a tracking issue. Please let me work on the implementation, I want to try.

cuviper: oh, usually I do the implementation before a tracking issue, otherwise this is more of a request issue

pickfire: Ah, I thought it is the opposite.

pickfire: By the way, the POC is done so I don't see any issues doing the implementation.

pickfire: It's like a TODO issue for me, I just left it there, IIRC last time I just create a pull request without a tracking issue.

cuviper: The big question I still have is how you intend to reconcile this across different crates -- core::iter::Iterator can't reference alloc::slice::Join. The POC doesn't answer this at all.

pickfire: Oh, I didn't thought about that.

pickfire: Can we even implement it in std::iter::Iterator?

cuviper: std is just a re-export

cuviper: I mentioned this before: https://rust-lang.zulipchat.com/#narrow/stream/219381-t-libs/topic/join.20on.20Iterator/near/203757438

pickfire: Ah, I forgot we cannot use Vec in core, Vec is needed to run collect before joining, do we need to do extension trait for this in alloc and reuse it in std?
pickfire:

error[E0412]: cannot find type `Vec` in this scope
    --> library/core/src/iter/traits/iterator.rs:1676:35
     |
1676 |         Join::join(self.collect::<Vec<Self::Item>>().as_slice(), sep)
     |                                   ^^^ not found in this scope

error[E0038]: the trait `iter::traits::iterator::Iterator` cannot be made into an object
    --> library/core/src/iter/traits/iterator.rs:17:30
     |
17   | fn _assert_is_object_safe(_: &dyn Iterator<Item = ()>) {}
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `iter::traits::iterator::Iterator` cannot be made into an object
...
98   | pub trait Iterator {
     |           -------- this trait cannot be made into an object...
...
1672 |     fn join<Separator>(self, sep: Separator) -> <[Self::Item] as Join<Separator>>::Output
     |        ---- ...because method `join` has generic type parameters
     |
     = help: consider moving `join` to another trait

error: aborting due to 2 previous errors

pickfire: I think we cannot put join into Iterator because the output is generic, maybe an extension trait? But I never seen any extension trait for Iterator.

lcnr: considering that we don't have collections in core, would it make sense to instead add fn intersperse to Iterator, so the example would be

["hello", "world"].iter().intersperse(" ").fold(String::new(), |s, e| { s.append_str(e); s });

lcnr: fn intersperse(self, e: Self::Item) -> ... where Self::Item: Clone and fn intersperse_with<F>(self, f: F) -> ... where F: FnMut() -> Self::Item

lcnr: https://hackage.haskell.org/package/base-4.14.0.0/docs/Data-List.html#v:intersperse

pickfire: So it shouldn't be called join but instead intersperse?

pickfire: The term sounds complicated to me.

pickfire: But doesn't that only allows the same type of item to be the separator?

lcnr: Afaik, unlike intersperse, join is eager in other languages

lcnr: so it makes sense to take the name already used in haskell. We already mostly use that their names for Iterator methods

pickfire: Well, collect + join is eager too.

lcnr: exactly, and intersperse is lazy

pickfire: Yes, then join is the correct term.

lcnr: We seem to talk about slightly different things rn, sorry

lcnr: My concern is that we can't really implement join as part of Iterator as it requires allocations, is that right?

pickfire: Ah, I mean the process is supposed to be eager, not lazy. So it shouldn't be called intersperse.

pickfire: Yes.

pickfire: Unless we changed it like collect.

lcnr: So intersperse is a way to add this to core in a clean manner imo

pickfire: But I don't see what intersperse solve.

lcnr: what does join solve :sweat_smile:

pickfire: It should allow joining with different type.
pickfire:

intersperse ',' "abcde"

That looks like the same type to me.

lcnr: ah, hmm :thinking:

pickfire: The other issue I faced is that the join I can think of have generics return.
pickfire:

fn join<Separator>(self, sep: Separator) -> <[Self::Item] as Join<Separator>>::Output

pickfire: But at the end, it probably still needs allocation.

pickfire: So I don't think it is possible to put it into core.

lcnr: wouldn't you want <Self::Item as Join<Separater>>::Output?

I am personally quite interested in adding intersperse to iterator as I needed a few times in the past and I think it can mostly replace join when dealing with iterators.

    pickfire:

    wouldn't you want <Self::Item as Join<Separater>>::Output?

Yes, I want that.

pickfire: But the tests does not want that.

pickfire: I mean the compiler.

lcnr: but yeah, don't know how I feel about adding join to Iterator. I am also not part of T-libsso :shrug:

pickfire: @lcnr Can you do intersperse with different type?

lcnr: I doubt it

pickfire: Doubt? So no? Yes?

    lcnr:

    But the tests does not want that.

why
pickfire:

error[E0038]: the trait `iter::traits::iterator::Iterator` cannot be made into an object
    --> library/core/src/iter/traits/iterator.rs:17:30
     |
17   | fn _assert_is_object_safe(_: &dyn Iterator<Item = ()>) {}
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `iter::traits::iterator::Iterator` cannot be made into an object
...
98   | pub trait Iterator {
     |           -------- this trait cannot be made into an object...
...
1672 |     fn join<Separator>(self, sep: Separator) -> <[Self::Item] as Join<Separator>>::Output
     |        ---- ...because method `join` has generic type parameters
     |
     = help: consider moving `join` to another trait

lcnr: you need Self: Sized on join in this case

lcnr: I think

pickfire: Oh, collect have that too.

pickfire: Nice, it worked.

pickfire: Only one thing left.

error[E0412]: cannot find type `Vec` in this scope
    --> library/core/src/iter/traits/iterator.rs:1677:35
     |
1677 |         Join::join(self.collect::<Vec<Self::Item>>().as_slice(), sep)
     |                                   ^^^ not found in this scope

error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.
error: could not compile `core`.

    lcnr:

    Doubt? So no? Yes?

intersperse with different types seems difficult to do cleanly

lcnr: at least I can't think of a good API rn

pickfire: Join already have those.

pickfire: That's why I am just making use of the existing stuff to make this.

pickfire: https://github.com/rust-lang/rust/pull/75738

Joshua Nelson: FWIW I think intersperse is a great idea even if it requires the same types

Joshua Nelson: s.split().intersperse (" ").collect() is a lot easier to write than turbo fish for Vec

Joshua Nelson: And more efficient too I think since you only collect once

lcnr: intersperse is already part of Itertools, it might make sense to promote it to std

lzutao: yeah, it is brilliant idea, but let's cc @bluss on that

pickfire: @Joshua Nelson The join I sent also collect once, intersperse looks useful but I see it as being very useful yet, at least not in my case.

pickfire: I only wish to not type .collect::<Vec<_>>().join(sep), just .join(sep) would be more than good.

lzutao: the problem I see with join method on Iterator is that it requires allocation and an extension Trait.
Where people use core only, they don't have a lazy iterator to loop over.

Joshua Nelson: @pickfire no, your suggestion collects twice: once to Vec and once again to join them all by a separator. So it allocates twice.

Joshua Nelson: It improves the ergnomics but not the behavior

Jonas Schievink: Why would join need an extension trait but not collect?

cuviper: @Jonas Schievink because collect just calls FromIterator, generic to any output. This join creates a Vec intermediate, so it can't be in core.

cuviper: IMO, this isn't worth it just to save a little typing. It would be more motivating if it could build dynamically to the final result, but that will probably need changes to Join.

Jonas Schievink: Isn't join(sep) just intersperse(sep).collect()?

Jonas Schievink: Ah, this one uses slice::Join

cuviper: With itertools, yes, but then you also have Itertools::join

Jonas Schievink: My take on this is that we should totally add intersperse and format from itertools to libcore, but nothing that requires allocation

pickfire: Yeah, I think changes to Join is required. @Joshua Nelson I didn't noticed that. I am looking for a way to improve it, then all the users that currently do .collect::<Vec<_>>().join() needs to allocate twice then.

pickfire: @Jonas Schievink No, Join allows different types unlike intersperse.

Jonas Schievink: .collect::<Vec<_>>().join() already allocates twice

pickfire: Well, all the users have to do this right now which is the easiest method.

Jonas Schievink: pickfire said:

    Jonas Schievink No, Join allows different types unlike intersperse.

ah, true

pickfire: @Joshua Nelson Yes, indeed. What you said is correct. I am looking for a way to both improve the ergonomics and performance. But I don't know how to know how much to allocate at first without having the first Vec? How do you know how much to allocate for the final Vec?
