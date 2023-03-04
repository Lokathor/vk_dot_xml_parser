#![allow(unused_mut)]
#![allow(unused_labels)]
#![allow(unused_imports)]
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

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub platforms: Vec<Platform>,
  pub vendors: Vec<VendorTag>,
  pub types: Vec<TypeEntry>,
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
        other => panic!("{other:?}"),
      }
    }
  }
}

/// A type declaration.
#[derive(Debug, Clone)]
pub enum TypeEntry {
  Include(Include),
  ExternType(ExternType),
  CppDefine(CppDefine),
  BaseType(BaseType),
  Bitmask(Bitmask),
  TypeAlias(TypeAlias),
  Handle(Handle),
  Enumeration(Enumeration),
  FuncPointer(FuncPointer),
}
impl TypeEntry {
  pub const fn name(&self) -> StaticStr {
    match self {
      Self::Include(Include{ name, ..})=> name,
      Self::ExternType(ExternType{ name, ..})=> name,
      Self::CppDefine(CppDefine{ name, ..})=> name,
      Self::BaseType(BaseType{ name, ..})=> name,
      Self::Bitmask(Bitmask{ name, ..})=> name,
      Self::TypeAlias(TypeAlias{ name, ..})=> name,
      Self::Handle(Handle{name, ..})=>name,
      Self::Enumeration(Enumeration{name, ..})=>name,
      Self::FnType(FnType{name, ..})=>name,
    }
  }
}

pub(crate) fn do_types(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return,
      StartTag { name: "comment", attrs: "" } => eat_to_end_of_comment(iter),
      StartTag { name: "type", attrs } => {
        let category = TagAttributeIterator::new(attrs)
          .find(|ta| ta.key == "category")
          .map(|ta| ta.value);
        match category {
          Some("include") => do_type_start_include(registry, attrs, iter),
          Some("define") => do_type_start_define(registry, attrs, iter),
          Some("basetype") => do_type_start_base(registry, attrs, iter),
          Some("bitmask") => do_type_start_bitmask(registry, attrs, iter),
          Some("handle") => do_type_start_handle(registry, attrs, iter),
          Some("funcpointer") => do_type_start_funcpointer(registry, attrs, iter),
          other => panic!("{other:?}"),
        }
      }
      EmptyTag { name: "type", attrs } => {
        let category = TagAttributeIterator::new(attrs)
          .find(|ta| ta.key == "category")
          .map(|ta| ta.value);
        match category {
          None => do_type_empty_none(registry, attrs),
          Some("handle") => {
            let type_alias = TypeAlias::from_attrs(attrs);
            debug!("{type_alias:?}");
            registry.types.push(TypeEntry::TypeAlias(type_alias));
          }
          Some("include") => do_type_empty_include(registry, attrs),
          Some("bitmask") => do_type_empty_bitmask(registry, attrs),
          Some("enum") => do_type_empty_enum(registry, attrs),
          other => panic!("{other:?}"),
        }
      }
      other => panic!("{other:?}"),
    }
  }
}
