/*!

Kuchiki (朽木), a HTML/XML tree manipulation library for Rust.

*/

#![deny(missing_docs)]

#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate matches;

mod attributes;
mod cell_extras;
pub mod iter;
mod node_data_ref;
mod parser;
mod select;
mod serializer;
#[cfg(test)]
mod tests;
mod tree;

pub use crate::attributes::{Attribute, Attributes, ExpandedName};
pub use crate::node_data_ref::NodeDataRef;
pub use crate::parser::{parse_html, parse_html_with_options, ParseOpts};
pub use crate::select::{Selector, Selectors, Specificity};
pub use crate::tree::{Doctype, DocumentData, ElementData, Node, NodeData, NodeRef};

/// This module re-exports a number of traits that are useful when using Kuchiki.
/// It can be used with:
///
/// ```rust
/// use kuchiki::traits::*;
/// ```
pub mod traits {
    pub use crate::iter::{ElementIterator, NodeIterator};
    pub use html5ever::tendril::TendrilSink;
}
