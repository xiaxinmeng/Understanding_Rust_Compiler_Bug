
//! Parser for (an empty) subset of the XML specification.
//! Demonstrates a multi-file library and doc comments
//! To build the library: cd sxml && rustc sxml.rc

// Modules named the same as the rc file are special in at least the following ways:
// 1) Items listed here are automatically used by other modules in the crate (e.g. Xml in this case).
// 2) This module does not need to be listed in the rc file.
// 3) Public items go into the library module, not a module nested inside the library.

use result::{Err, Ok, Result};
use std::map::{HashMap};

// We do this to export our public API to clients of our library. This allows
// clients to use our library without worrying about the details of how it
// organizes sub-modules.
pub use parsing::*;
pub use validation::*;

pub type Attributes = HashMap<@~str, @~str>;

/// Recursive enum which vaguely resembles XML.
pub enum Xml
{
    /// Text content
    Content(@~str),

    /// Element only content: element name, attributes, and child elements
    Element(@~str, Attributes, @[@Xml]),
}

// The /// doc comment applies to the next item.
// The //! doc comment applies to the thing the comment is within (e.g. a module or a function).
// Doc comments use markdown syntax, see http://daringfireball.net/projects/markdown/syntax

/// Parses and validates XML.
/// # Usage
///     match sxml::from_str(content)
///     {
///         Ok(xml) => process(xml),
///         Err(mesg) => fail mesg,
///     }
pub fn from_str(text: &str) -> Result<Xml, @~str>
{
    do parse_str(text).chain
    |xml|
    {
        match validate_xml(&xml)
        {
            @~""   => Ok(xml),
            errors => Err(errors),
        }
    }
}

