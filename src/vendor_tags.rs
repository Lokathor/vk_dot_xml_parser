use super::*;

/// We name the `<tag .. />` a "vendor tag", to avoid confusion with XML tags.
#[derive(Debug, Clone, Copy)]
pub struct VendorTag {
  pub name: StaticStr,
  pub author: StaticStr,
  pub contact: StaticStr,
}
pub(crate) fn do_tags(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "tags" } => return,
      EmptyTag { name: "tag", attrs } => {
        let mut name = None;
        let mut author = None;
        let mut contact = None;
        for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
          match key {
            "name" => name = Some(value),
            "author" => author = Some(value),
            "contact" => contact = Some(value),
            other => panic!("{other:?}"),
          }
        }
        let vendor_tag = VendorTag {
          name: name.unwrap(),
          author: author.unwrap(),
          contact: contact.unwrap(),
        };
        debug!("{vendor_tag:?}");
        registry.vendors.push(vendor_tag);
      }
      other => panic!("{other:?}"),
    }
  }
}
