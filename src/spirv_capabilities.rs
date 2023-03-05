use super::*;

pub(crate) fn do_spirvcapabilities(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  'spirv_capabilities: loop {
    match iter.next().unwrap() {
      EndTag { name: "spirvcapabilities" } => break 'spirv_capabilities,
      StartTag { name: "spirvcapability", attrs } => {
        let mut spirv_capability = SpirvCapability::from_attrs(attrs);
        'spirv_capability: loop {
          match iter.next().unwrap() {
            EndTag { name: "spirvcapability" } => {
              debug!("{spirv_capability:?}");
              registry.spirv_capabilities.push(spirv_capability);
              break 'spirv_capability;
            }
            EmptyTag { name: "enable", attrs } => {
              if TagAttributeIterator::new(attrs).any(|ta| ta.key == "version") {
                spirv_capability.version = Some(
                  TagAttributeIterator::new(attrs)
                    .find(|ta| ta.key == "version")
                    .unwrap()
                    .value,
                );
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "struct") {
                let s = SpirvCapabilityStruct::from_attrs(attrs);
                spirv_capability.structs.push(s);
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "property") {
                let p = SpirvCapabilityProperty::from_attrs(attrs);
                spirv_capability.properties.push(p);
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "extension") {
                spirv_capability.extension = Some(
                  TagAttributeIterator::new(attrs)
                    .find(|ta| ta.key == "extension")
                    .unwrap()
                    .value,
                );
              } else {
                panic!("{attrs:?}");
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
pub struct SpirvCapability {
  pub name: StaticStr,
  pub version: Option<StaticStr>,
  pub extension: Option<StaticStr>,
  pub structs: Vec<SpirvCapabilityStruct>,
  pub properties: Vec<SpirvCapabilityProperty>,
}
impl SpirvCapability {
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

#[derive(Debug, Clone, Default)]
pub struct SpirvCapabilityStruct {
  pub name: StaticStr,
  pub feature: StaticStr,
  pub requires: StaticStr,
  pub alias: Option<StaticStr>,
}
impl SpirvCapabilityStruct {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "struct" => x.name = value,
        "feature" => x.feature = value,
        "requires" => x.requires = value,
        "alias" => x.alias = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct SpirvCapabilityProperty {
  pub name: StaticStr,
  pub member: StaticStr,
  pub value: StaticStr,
  pub requires: StaticStr,
}
impl SpirvCapabilityProperty {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "property" => x.name = value,
        "member" => x.member = value,
        "value" => x.value = value,
        "requires" => x.requires = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
