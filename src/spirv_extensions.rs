use super::*;

pub(crate) fn do_spirvextensions(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  'spirv_extensions: loop {
    match iter.next().unwrap() {
      EndTag { name: "spirvextensions" } => break 'spirv_extensions,
      StartTag { name: "spirvextension", attrs } => {
        let mut spirv_extension = SpirvExtension::from_attrs(attrs);
        'spirv_extension: loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvextension" } => {
              registry.spirv_extensions.push(spirv_extension);
              break 'spirv_extension;
            }
            EmptyTag { name: "enable", attrs } => {
              assert_eq!(TagAttributeIterator::new(attrs).count(), 1);
              for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
                match key {
                  "version" => spirv_extension.version = Some(value),
                  "extension" => spirv_extension.extension = Some(value),
                  other => panic!("{other:?}"),
                }
              }
            }
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvExtension {
  pub name: StaticStr,
  pub version: Option<StaticStr>,
  pub extension: Option<StaticStr>,
}
impl SpirvExtension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
