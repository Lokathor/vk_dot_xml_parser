use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Platform {
  pub name: StaticStr,
  pub protect: StaticStr,
  pub comment: StaticStr,
}

pub(crate) fn do_platforms(
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
        debug!("{platform:?}");
        registry.platforms.push(platform);
      }
      other => panic!("{other:?}"),
    }
  }
}
