use super::*;

pub type StaticStr = &'static str;

pub(crate) fn eat_to_end_of_comment(
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  loop {
    if let EndTag { name: "comment" } = iter.next().unwrap() {
      return;
    }
  }
}
