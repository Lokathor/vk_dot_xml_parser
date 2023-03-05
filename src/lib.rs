#![allow(unused_mut)]
#![allow(unused_labels)]
#![allow(unused_imports)]
#![allow(clippy::unused_unit)]
#![allow(clippy::single_match)]
#![allow(clippy::match_single_binding)]

macro_rules! assert_attrs_comment_only {
  ($attrs:expr) => {
    for TagAttribute { key, value } in TagAttributeIterator::new($attrs) {
      match key {
        "comment" => (),
        _ => panic!("{key:?} = {value:?}"),
      }
    }
  };
}

use log::{debug, trace};
use magnesium::{XmlElement::*, *};

mod utils;
pub use utils::*;

mod platforms;
pub use platforms::*;

mod vendor_tags;
pub use vendor_tags::*;

mod type_entry;
pub use type_entry::*;

mod enums;
pub use enums::*;

mod commands;
pub use commands::*;

mod feature;
pub use feature::*;

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub platforms: Vec<Platform>,
  pub vendors: Vec<VendorTag>,
  pub types: Vec<TypeEntry>,
  pub enums: Vec<Enums>,
  pub commands: Vec<Command>,
  pub command_aliases: Vec<CommandAlias>,
  pub features: Vec<Feature>,
}
impl Registry {
  pub fn from_static_str(s: StaticStr) -> Self {
    let mut iter = ElementIterator::new(s)
      .filter_map(skip_comments)
      .map(trim_text)
      .filter_map(skip_empty_text_elements);
    assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("registry", ""));
    let mut registry = Self::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: "" } => eat_to_end_of_comment(&mut iter),
        StartTag { name: "platforms", attrs } => {
          do_platforms(&mut registry, attrs, &mut iter)
        }
        StartTag { name: "tags", attrs } => do_tags(&mut registry, attrs, &mut iter),
        StartTag { name: "types", attrs } => do_types(&mut registry, attrs, &mut iter),
        StartTag { name: "enums", attrs } => do_enums(&mut registry, attrs, &mut iter),
        StartTag { name: "commands", attrs } => {
          do_commands(&mut registry, attrs, &mut iter)
        }
        StartTag { name: "feature", attrs } => {
          do_feature(&mut registry, attrs, &mut iter)
        }
        other => panic!("{other:?}"),
      }
    }
  }
}
