use super::*;

pub fn do_enums(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  if TagAttributeIterator::new(attrs).find(|ta| ta.key == "name").unwrap().value
    == "API Constants"
  {
    'api_constants: loop {
      match iter.next().unwrap() {
        EndTag { name: "enums" } => {
          return;
        }
        EmptyTag { name: "enum", attrs } => {
          if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
            let alias = ApiConstantAlias::from_attrs(attrs);
            debug!("{alias:?}");
            registry.api_constant_aliases.push(alias);
          } else {
            let api_const = ApiConstant::from_attrs(attrs);
            debug!("{api_const:?}");
            registry.api_constants.push(api_const);
          }
        }
        other => panic!("{other:?}"),
      }
    }
  } else {
    let mut enums = EnumsGroup::from_attrs(attrs);
    'enums: loop {
      match iter.next().unwrap() {
        EndTag { name: "enums" } => {
          debug!("{enums:?}");
          registry.enums_groups.push(enums);
          return;
        }
        EmptyTag { name: "enum", attrs } => {
          do_enums_empty_enum(&mut enums, attrs);
        }
        StartTag { name: "comment", attrs: "" } => eat_to_end_of_comment(iter),
        EmptyTag { name: "unused", attrs: _ } => {
          (/* do we care to record something *not* used? */)
        }
        other => panic!("{other:?}"),
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ApiConstant {
  pub name: StaticStr,
  pub value: StaticStr,
  pub ty: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl ApiConstant {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "value" => x.value = value,
        "type" => x.ty = Some(value),
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct ApiConstantAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
}
impl ApiConstantAlias {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "alias" => x.alias_of = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

/// An `<enums>` tag and its inner data.
#[derive(Debug, Clone, Default)]
pub struct EnumsGroup {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
  pub values: Vec<EnumValue>,
  pub aliases: Vec<EnumAlias>,
  pub bit_positions: Vec<EnumBitPosition>,
  pub ty: Option<StaticStr>,
  pub is_64_bit: bool,
}
impl EnumsGroup {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "type" => x.ty = Some(value),
        "bitwidth" if value == "64" => x.is_64_bit = true,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumValue {
  pub name: StaticStr,
  pub value: StaticStr,
  pub comment: Option<StaticStr>,
}
impl EnumValue {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "value" => x.value = value,
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct EnumAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
  pub api: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
}
impl EnumAlias {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
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
pub struct EnumBitPosition {
  pub name: StaticStr,
  pub bit: StaticStr,
  pub comment: Option<StaticStr>,
}
impl EnumBitPosition {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "bitpos" => x.bit = value,
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_enums_empty_enum(enums: &mut EnumsGroup, attrs: StaticStr) {
  if TagAttributeIterator::new(attrs).any(|ta| ta.key == "value") {
    let value = EnumValue::from_attrs(attrs);
    debug!("{value:?}");
    enums.values.push(value);
  } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
    let alias = EnumAlias::from_attrs(attrs);
    debug!("{alias:?}");
    enums.aliases.push(alias);
  } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "bitpos") {
    let bit_pos = EnumBitPosition::from_attrs(attrs);
    debug!("{bit_pos:?}");
    enums.bit_positions.push(bit_pos);
  } else {
    todo!();
  }
}
