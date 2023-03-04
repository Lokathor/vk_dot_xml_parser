use super::*;

pub fn do_enums(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut enums = Enums::from_attrs(attrs);
  'enums: loop {
    match iter.next().unwrap() {
      EndTag { name: "enums" } => {
        debug!("{enums:?}");
        registry.enums.push(enums);
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

#[derive(Debug, Clone, Default)]
pub struct Enums {
  pub name: StaticStr,
  pub comment: Option<StaticStr>,
  pub entries: Vec<EnumsEntry>,
  pub ty: Option<StaticStr>,
  pub is_64_bit: bool,
}
impl Enums {
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

#[derive(Debug, Clone)]
pub enum EnumsEntry {
  Value(Value),
  ValueAlias(ValueAlias),
  BitPosition(BitPosition),
}

#[derive(Debug, Clone, Default)]
pub struct Value {
  pub name: StaticStr,
  pub value: StaticStr,
  pub ty: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl Value {
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
pub struct ValueAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
  pub api: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
}
impl ValueAlias {
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
pub struct BitPosition {
  pub name: StaticStr,
  pub bit: StaticStr,
  pub ty: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl BitPosition {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "bitpos" => x.bit = value,
        "type" => x.ty = Some(value),
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_enums_empty_enum(enums: &mut Enums, attrs: StaticStr) {
  if TagAttributeIterator::new(attrs).any(|ta| ta.key == "value") {
    let value = Value::from_attrs(attrs);
    debug!("{value:?}");
    enums.entries.push(EnumsEntry::Value(value));
  } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
    let alias = ValueAlias::from_attrs(attrs);
    debug!("{alias:?}");
    enums.entries.push(EnumsEntry::ValueAlias(alias));
  } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "bitpos") {
    let bit_pos = BitPosition::from_attrs(attrs);
    debug!("{bit_pos:?}");
    enums.entries.push(EnumsEntry::BitPosition(bit_pos));
  } else {
    todo!();
  }
}
