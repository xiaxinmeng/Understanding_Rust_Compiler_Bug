
// Searches text files for "bad" words.
// Demonstrates tasks and sharing large objects across tasks (ARC).
// To run execute: rustc --test bad_words.rs && ./bad_words
extern mod std;

use comm::{Chan, Port};
use io::WriterUtil;
use mod std::arc;
use std::arc::{ARC};

// Contains details about a file (or the unit test equivalent).
struct Contents
{
    fname: ~str,
    lines: ~[~str],
}

// Used to lazily load file contents.
type FileLoader = fn~ () -> ~Contents;

// This function runs within a dedicated task, scans through lines
// looking for words in bad, and sends a report with the results
// using chan.
fn scan_words(loader: FileLoader, bad: &~[~str], chan: Chan<~str>)
{
    let mut results = ~"";

    // Defer loading files until it's neccesary (we don't want to read in 
    // thousands of files before we even spin up a task).
    let contents = loader();
    for contents.lines.eachi
    |i, line|
    {
        for bad.each
        |b|
        {
            if line.contains(*b)
            {
                // TODO: is it efficient to concantenate strings like this?
                results += fmt!("%s:%? found %s\n", contents.fname, i+1, b.trim_right());
            }
        }
    }

    chan.send(move results);
}

// Uses multiple tasks to find the bad words in each file. Reports from tasks
// that find bad words are returned.
fn scan_files(+loaders: ~[FileLoader], +bad: ~[~str]) -> ~str
{
    fn massage_results(result: ~[@~str]) -> ~str
    {
        // We'll get the results back in a non-deterministic order so to simplify unit testing
        // we'll do a sort.
        let result = std::sort::merge_sort(|x, y| {x <= y}, result);

        // Convert to ~str so that connect works. This does do a copy, but it's not a copy
        // of a full line from a file.
        let result = do result.map |r| {copy *r};

        move str::connect(result, "")
    }

    let mut outstanding_tasks = loaders.len();
    let port = Port();
    let chan = Chan(port);

    // We take care throughout to avoid copying potentially large objects. Sending 
    // objects to tasks is especially delicate for the bad word vector because we
    // need to share it across tasks and tasks do not normally share state. However
    // we can use the std::arc module to share immutable data across tasks.
    let bad = ARC(bad);

    // Kick off a task to process each file. This can potentially kick off thousands
    // of tasks, but that's OK because tasks are much lighter-weight than threads
    // and tasks run until they finish or block (in which case another task on the
    // thread wakes up).
    //
    // We use consume here which acts like eachi except that the elements are moved 
    // into the closure instead of copied (and the vector is emptied afterwards).
    do vec::consume(loaders)
    |_i, loader|
    {
        // arc::clone is efficient because it clones the ARC, not the data the ARC contains.
        let bad = arc::clone(&bad);

        do task::spawn |move loader, move bad| {scan_words(loader, arc::get(&bad), chan)};
    }

    // Collect results until every task has finished.
    let mut result = ~[];
    while outstanding_tasks > 0
    {
        let r = port.recv();
        if r.is_not_empty()
        {
            // Push an @~str onto result so that sort will work (sort requires implicitly copyable strings).
            vec::push(result, @r);
        }
        outstanding_tasks -= 1;
    }

    move massage_results(result)
}

#[cfg(test)]
pub fn check_strs(actual: &str, expected: &str) -> bool
{
    if actual != expected
    {
        io::stderr().write_line(fmt!("Found:\n%s\nbut expected\n%s", actual, expected));
        return false;
    }
    return true;
}

// Make files large enough that the tasks actually have to do a significant amount of work.
#[cfg(test)]
fn make_good_file(fname: &str) -> FileLoader
{
    const num_files: uint = 20_000;    // underscores are ignored in numeric literals

    let fname = fname.to_unique();
    let f: FileLoader =
    ||
    {
        move ~Contents
        {
            fname: fname,
            lines: do vec::build_sized(num_files)
                |push|
                {
                    for num_files.times {push(move ~"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")}
                },
        }
    };
    f
}

// Type inference usually works very well, but closures sometimes need help which
// is why we have these two helper functions.
#[cfg(test)]
fn make_bad1(fname: &str) -> FileLoader
{
    let fname = fname.to_unique();
    let f: FileLoader = || {move ~Contents {fname: copy fname, lines: ~[~"this line is ok", ~"but ugh is not", ~"another good line"]}};
    f
}

#[cfg(test)]
fn make_bad2(fname: &str) -> FileLoader
{
    let fname = fname.to_unique();
    let f: FileLoader = || {move ~Contents {fname: copy fname, lines: ~[~"this line is ok", ~"another good line", ~"ack is not cool"]}};
    f
}

// Load num_times files (and spawn that many tasks). Up to two files
// will include bad words.
#[cfg(test)]
fn run_test(num_files: uint)
{
    let mut files = do vec::build_sized(num_files)
        |push|
        {
            for num_files.timesi
            |i|
            {
                let fname = fmt!("f%?", i);
                let loader: FileLoader =
                    match i
                    {
                        1  => make_bad1(fname),
                        42 => make_bad2(fname),
                        _  => make_good_file(fname),
                    };
                push(move loader);
            }
        };

    let bad = ~[~"ugh ", ~"ack"];
    let actual = scan_files(files, bad);
    if num_files >= 42
    {
        assert check_strs(actual, "f1:2 found ugh\nf42:3 found ack\n");
    }
    else if num_files >= 1
    {
        assert check_strs(actual, "f1:2 found ugh\n");
    }
    else
    {
        assert check_strs(actual, "");
    }
}

#[test]
fn test_empty()
{
    assert check_strs(scan_files(~[], ~[~"ack", ~"ugh"]), "");
}

#[test]
fn test_few()
{
    run_test(4);
}

#[test]
fn test_many()
{
    // On an 8-core Mac 10 system threads are created to run these 200 tasks.
    run_test(200);
}
