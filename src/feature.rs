use super::*;

pub(crate) fn do_feature(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut feature = Feature::from_attrs(attrs);
  'feature: loop {
    match iter.next().unwrap() {
      EndTag { name: "feature" } => {
        debug!("{feature:?}");
        registry.features.push(feature);
        return;
      }
      StartTag { name: "require", attrs } => {
        let mut requirement = Requirement::from_attrs(attrs);
        'require: loop {
          match iter.next().unwrap() {
            EndTag { name: "require" } => {
              debug!("{requirement:?}");
              feature.requirements.push(requirement);
              break 'require;
            }
            EmptyTag { name: "type", attrs } => {
              let t = RequiredType::from_attrs(attrs);
              trace!("{t:?}");
              requirement.required_types.push(t);
            }
            EmptyTag { name: "enum", attrs } => {
              if TagAttributeIterator::new(attrs).any(|ta| ta.key == "offset") {
                let e = RequiredEnumOffset::from_attrs(attrs);
                trace!("{e:?}");
                requirement.required_offset_enums.push(e);
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "bitpos") {
                let e = RequiredEnumBitpos::from_attrs(attrs);
                trace!("{e:?}");
                requirement.required_bitpos_enums.push(e);
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
                let e = RequiredEnumAlias::from_attrs(attrs);
                trace!("{e:?}");
                requirement.required_alias_enums.push(e);
              } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "value") {
                let e = RequiredEnumValue::from_attrs(attrs);
                trace!("{e:?}");
                requirement.required_value_enums.push(e);
              } else {
                let e = RequiredEnumPlain::from_attrs(attrs);
                trace!("{e:?}");
                requirement.required_plain_enums.push(e);
              }
            }
            EmptyTag { name: "command", attrs } => {
              let c = RequiredCommand::from_attrs(attrs);
              trace!("{c:?}");
              requirement.required_commands.push(c);
            }
            StartTag { name: "comment", attrs: "" } => {
              let _ = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
            }
            other => panic!("{other:?}"),
          }
        }
      }
      EmptyTag { name: "require", attrs } => {
        let mut requirement = Requirement::from_attrs(attrs);
        debug!("{requirement:?}");
        feature.requirements.push(requirement);
      }
      StartTag { name: "remove", attrs } => {
        assert_attrs_comment_only!(attrs);
        'remove: loop {
          match iter.next().unwrap() {
            EndTag { name: "remove" } => break 'remove,
            EmptyTag { name: "enum", attrs } => {
              assert_eq!(TagAttributeIterator::new(attrs).count(), 1);
              let ta =
                TagAttributeIterator::new(attrs).find(|ta| ta.key == "name").unwrap();
              feature.removed_enums.push(ta.value);
            }
            EmptyTag { name: "type", attrs } => {
              assert_eq!(TagAttributeIterator::new(attrs).count(), 1);
              let ta =
                TagAttributeIterator::new(attrs).find(|ta| ta.key == "name").unwrap();
              feature.removed_types.push(ta.value);
            }
            EmptyTag { name: "command", attrs } => {
              assert_eq!(TagAttributeIterator::new(attrs).count(), 1);
              let ta =
                TagAttributeIterator::new(attrs).find(|ta| ta.key == "name").unwrap();
              feature.removed_commands.push(ta.value);
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
pub struct Feature {
  pub name: StaticStr,
  pub number: StaticStr,
  pub api: StaticStr,
  pub comment: StaticStr,
  pub requirements: Vec<Requirement>,
  pub removed_types: Vec<StaticStr>,
  pub removed_enums: Vec<StaticStr>,
  pub removed_commands: Vec<StaticStr>,
}
impl Feature {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "number" => x.number = value,
        "api" => x.api = value,
        "comment" => x.comment = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct Requirement {
  pub comment: Option<StaticStr>,
  pub depends: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub required_types: Vec<RequiredType>,
  pub required_plain_enums: Vec<RequiredEnumPlain>,
  pub required_offset_enums: Vec<RequiredEnumOffset>,
  pub required_bitpos_enums: Vec<RequiredEnumBitpos>,
  pub required_alias_enums: Vec<RequiredEnumAlias>,
  pub required_value_enums: Vec<RequiredEnumValue>,
  pub required_commands: Vec<RequiredCommand>,
}
impl Requirement {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "comment" => x.comment = Some(value),
        "depends" => x.depends = Some(value),
        "api" => x.api = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredType {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
}
impl RequiredType {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnumPlain {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
}
impl RequiredEnumPlain {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnumOffset {
  pub name: StaticStr,
  pub extends: StaticStr,
  pub extension_number: i32,
  pub offset: i32,
  pub comment: Option<StaticStr>,
  pub is_negative: bool,
  pub api: Option<StaticStr>,
  pub protect: Option<StaticStr>,
}
impl RequiredEnumOffset {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "extends" => x.extends = value,
        "extnumber" => x.extension_number = value.parse().unwrap(),
        "offset" => x.offset = value.parse().unwrap(),
        "dir" if value == "-" => x.is_negative = true,
        "api" => x.api = Some(value),
        "protect" => x.protect = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnumBitpos {
  pub name: StaticStr,
  pub extends: StaticStr,
  pub bitpos: u32,
  pub comment: Option<StaticStr>,
  pub protect: Option<StaticStr>,
}
impl RequiredEnumBitpos {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "bitpos" => x.bitpos = value.parse().unwrap(),
        "extends" => x.extends = value,
        "protect" => x.protect = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnumAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
  pub extends: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
}
impl RequiredEnumAlias {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "extends" => x.extends = Some(value),
        "alias" => x.alias_of = value,
        "api" => x.api = Some(value),
        "deprecated" => x.deprecated = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredEnumValue {
  pub name: StaticStr,
  pub extends: StaticStr,
  pub value: StaticStr,
  pub comment: Option<StaticStr>,
  pub api: Option<StaticStr>,
}
impl RequiredEnumValue {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "extends" => x.extends = value,
        "value" => x.value = value,
        "api" => x.api = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredCommand {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
}
impl RequiredCommand {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
