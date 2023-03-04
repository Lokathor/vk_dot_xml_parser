use super::*;

#[derive(Debug, Clone, Default)]
pub struct Include {
  pub name: StaticStr,
  pub text: Option<String>,
}
impl Include {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "include"),
        "name" => x.name = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_include(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut include = Include::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      Text(t) => include.text.get_or_insert(String::new()).push_str(t),
      other => panic!("{other:?}"),
    }
  }
  debug!("{include:?}");
  registry.types.push(TypeEntry::Include(include));
}

pub(crate) fn do_type_empty_include(registry: &mut Registry, attrs: StaticStr) {
  let include = Include::from_attrs(attrs);
  debug!("{include:?}");
  registry.types.push(TypeEntry::Include(include));
}

#[derive(Debug, Clone, Default)]
pub struct ExternType {
  pub name: StaticStr,
  pub requires_header: StaticStr,
}
impl ExternType {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "requires" => x.requires_header = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_empty_none(registry: &mut Registry, attrs: StaticStr) {
  let extern_type = ExternType::from_attrs(attrs);
  debug!("{extern_type:?}");
  registry.types.push(TypeEntry::ExternType(extern_type));
}

/// C Pre-Processor `#define`
#[derive(Debug, Clone, Default)]
pub struct CppDefine {
  pub name: StaticStr,
  pub text: String,
  pub deprecated: bool,
  pub requires: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub comment: Option<StaticStr>,
}
impl CppDefine {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "define"),
        "deprecated" => {
          x.deprecated = match value {
            "true" => true,
            "false" => false,
            other => panic!("{other:?}"),
          }
        }
        "requires" => x.requires = Some(value),
        "api" => x.api = Some(value),
        "comment" => x.comment = Some(value),
        "name" => x.name = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_define(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut cpp_define = CppDefine::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      Text(t) => {
        if !cpp_define.text.is_empty() && !t.starts_with('(') {
          cpp_define.text.push(' ');
        }
        cpp_define.text.push_str(&revert_xml_encoding(t))
      }
      StartTag { name: "name", attrs: "" } => {
        cpp_define.name = iter.next().unwrap().unwrap_text();
        cpp_define.text.push(' ');
        cpp_define.text.push_str(cpp_define.name);
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
      }
      StartTag { name: "type", attrs: "" } => {
        cpp_define.text.push(' ');
        cpp_define.text.push_str(iter.next().unwrap().unwrap_text());
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
      }
      other => panic!("{other:?}"),
    }
  }
  // normalize newlines
  cpp_define.text = cpp_define.text.replace("\r\n", "\n");
  debug!("{cpp_define:?}");
  registry.types.push(TypeEntry::CppDefine(cpp_define));
}

/// C Pre-Processor `#define`
#[derive(Debug, Clone, Default)]
pub struct BaseType {
  pub name: StaticStr,
  pub text: String,
}
impl BaseType {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "basetype"),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_base(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut base = BaseType::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      Text(t) => base.text.push_str(&revert_xml_encoding(t)),
      StartTag { name: "name", attrs: "" } => {
        base.name = iter.next().unwrap().unwrap_text();
        base.text.push(' ');
        base.text.push_str(base.name);
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
      }
      StartTag { name: "type", attrs: "" } => {
        base.text.push(' ');
        base.text.push_str(iter.next().unwrap().unwrap_text());
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
      }
      other => panic!("{other:?}"),
    }
  }
  // normalize newlines
  base.text = base.text.replace("\r\n", "\n");
  debug!("{base:?}");
  registry.types.push(TypeEntry::BaseType(base));
}

#[derive(Debug, Clone, Default)]
pub struct Bitmask {
  pub name: StaticStr,
  pub requires: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub bit_values: Option<StaticStr>,
  pub flags64: bool,
}
impl Bitmask {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "category" => assert!(["bitmask", "enum"].contains(&value)),
        "requires" => x.requires = Some(value),
        "api" => x.api = Some(value),
        "bitvalues" => x.bit_values = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_bitmask(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut bitmask = Bitmask::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      Text("typedef") => {
        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("type", ""));
        let t = iter.next().unwrap().unwrap_text();
        if t == "VkFlags64" {
          bitmask.flags64 = true;
        } else {
          assert_eq!(t, "VkFlags");
        }
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
        bitmask.name = iter.next().unwrap().unwrap_text();
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
        assert_eq!(iter.next().unwrap().unwrap_text(), ";");
      }
      other => panic!("{other:?}"),
    }
  }
  debug!("{bitmask:?}");
  registry.types.push(TypeEntry::Bitmask(bitmask));
}

#[derive(Debug, Clone, Default)]
pub struct TypeAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
  pub category: Option<StaticStr>,
}
impl TypeAlias {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "alias" => x.alias_of = value,
        "category" => x.category = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_empty_bitmask(registry: &mut Registry, attrs: StaticStr) {
  if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
    let type_alias = TypeAlias::from_attrs(attrs);
    debug!("{type_alias:?}");
    registry.types.push(TypeEntry::TypeAlias(type_alias));
  } else {
    let bitmask = Bitmask::from_attrs(attrs);
    debug!("{bitmask:?}");
    registry.types.push(TypeEntry::Bitmask(bitmask));
  }
}

#[derive(Debug, Clone, Default)]
pub struct Handle {
  pub name: StaticStr,
  pub obj_ty_enum: StaticStr,
  pub parent: Option<StaticStr>,
  pub non_dispatchable: bool,
}
impl Handle {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "handle"),
        "name" => x.name = value,
        "objtypeenum" => x.obj_ty_enum = value,
        "parent" => x.parent = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_handle(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut handle = Handle::from_attrs(attrs);
  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("type", ""));
  match iter.next().unwrap().unwrap_text() {
    "VK_DEFINE_HANDLE" => (),
    "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => handle.non_dispatchable = true,
    other => panic!("{other:?}"),
  }
  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
  assert_eq!(iter.next().unwrap().unwrap_text(), "(");
  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
  handle.name = iter.next().unwrap().unwrap_text();
  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
  assert_eq!(iter.next().unwrap().unwrap_text(), ")");
  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
  debug!("{handle:?}");
  registry.types.push(TypeEntry::Handle(handle));
}

#[derive(Debug, Clone, Default)]
pub struct Enumeration {
  pub name: StaticStr,
}
impl Enumeration {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "enum"),
        "name" => x.name = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_empty_enum(registry: &mut Registry, attrs: StaticStr) {
  if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias") {
    let type_alias = TypeAlias::from_attrs(attrs);
    debug!("{type_alias:?}");
    registry.types.push(TypeEntry::TypeAlias(type_alias));
  } else {
    let name = TagAttributeIterator::new(attrs)
      .find(|ta| ta.key == "name")
      .map(|ta| ta.value)
      .expect("No `name` present in empty `enum` tag.");
    if name.contains("Flags") || name.contains("FlagBits") {
      let bitmask = Bitmask::from_attrs(attrs);
      debug!("{bitmask:?}");
      registry.types.push(TypeEntry::Bitmask(bitmask));
    } else {
      let e = Enumeration::from_attrs(attrs);
      debug!("{e:?}");
      registry.types.push(TypeEntry::Enumeration(e));
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TypeVariant {
  /// `T`
  #[default]
  Normal,
  /// `*mut T`
  MutPtr,
}

#[derive(Debug, Clone, Default)]
pub struct FuncPointer {
  pub name: StaticStr,
  pub text: String,
  pub requires: Option<StaticStr>,
}
impl FuncPointer {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "funcpointer"),
        "requires" => x.requires = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_funcpointer(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut f = FuncPointer::from_attrs(attrs);
  f.text.push_str(iter.next().unwrap().unwrap_text());
  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
  f.name = iter.next().unwrap().unwrap_text();
  f.text.push(' ');
  f.text.push_str(f.name);
  f.text.push(' ');
  assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      Text(t) => f.text.push_str(t),
      StartTag { name: "type", attrs: "" } => {
        f.text.push(' ');
        f.text.push_str(iter.next().unwrap().unwrap_text());
        f.text.push(' ');
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
      }
      other => panic!("{other:?}"),
    }
  }
  // cut whitespace
  f.text = f.text.replace("\r\n", "");
  f.text = f.text.replace("\n", "");
  let mut replacement = f.text.replace("  "," ");
  while f.text != replacement {
    f.text = replacement;
    replacement = f.text.replace("  "," ");
  }
  debug!("{f:?}");
  registry.types.push(TypeEntry::FuncPointer(f));
}
