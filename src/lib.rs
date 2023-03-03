#![allow(clippy::single_match)]

use magnesium::{XmlElement::*, *};

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

pub type StaticStr = &'static str;

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub platforms: Vec<Platform>,
}
impl Registry {
  pub fn from_static_str(s: StaticStr) -> Self {
    let mut iter = ElementIterator::new(s)
      .filter_map(skip_comments)
      .map(trim_text)
      .filter_map(skip_empty_text_elements);
    assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("registry", ""));
    #[allow(unused_mut)]
    let mut registry = Self::default();
    loop {
      match iter.next().unwrap() {
        EndTag { name: "registry" } => return registry,
        StartTag { name: "comment", attrs: "" } => eat_to_end_of_comment(&mut iter),
        StartTag { name: "platforms", attrs } => {
          do_platforms(&mut registry, attrs, &mut iter)
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

fn eat_to_end_of_comment(iter: &mut impl Iterator<Item = XmlElement<'static>>) {
  loop {
    if let EndTag { name: "comment" } = iter.next().unwrap() {
      return;
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Platform {
  pub name: StaticStr,
  pub protect: StaticStr,
  pub comment: StaticStr,
}

fn do_platforms(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "platforms" } => return,
      EmptyTag { name: "platform", attrs } => {
        let mut name = None;
        let mut protect = None;
        let mut comment = None;
        for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
          match key {
            "name" => name = Some(value),
            "protect" => protect = Some(value),
            "comment" => comment = Some(value),
            other => panic!("{other:?}"),
          }
        }
        let platform = Platform {
          name: name.unwrap(),
          protect: protect.unwrap(),
          comment: comment.unwrap(),
        };
        registry.platforms.push(dbg!(platform));
      }
      other => panic!("{other:?}"),
    }
  }
}
