use super::*;

/// A type declaration.
#[derive(Debug, Clone)]
pub enum TypeEntry {
  Include(Include),
  ExternType(ExternType),
  CppDefine(CppDefine),
  BaseType(BaseType),
  /// The "bitmask" category is weird.
  ///
  /// All bitmask types are "supposed" to be both a FooFlags and a FooFlagBits.
  /// The bit values are defined as a FooFlagBits value, and then FooFlags is an
  /// alias for FooFlagBits. However, when no bits are defined in any API level
  /// or extension for a given FlagBits then instead *only* the Flags type is
  /// defined. This difference somewhat matters even if a generator wants to
  /// create both forms unconditionally, because normally the FlagBits type
  /// within the HTML docs shows all the values and you'd also want the Flags
  /// type to link to the FlagBits page. However, when there's no bits values
  /// then the FlagBits page doesn't exist at all and linking there will give
  /// people a 404. So if a generator is making docs links it needs to pay close
  /// attention.
  Bitmask(Bitmask),
  TypeAlias(TypeAlias),
  Handle(Handle),
  Enumeration(Enumeration),
  FuncPointer(FuncPointer),
  Structure(Structure),
  Union(Union),
}
impl TypeEntry {
  pub const fn name(&self) -> StaticStr {
    match self {
      Self::Include(Include { name, .. }) => name,
      Self::ExternType(ExternType { name, .. }) => name,
      Self::CppDefine(CppDefine { name, .. }) => name,
      Self::BaseType(BaseType { name, .. }) => name,
      Self::Bitmask(Bitmask { name, .. }) => name,
      Self::TypeAlias(TypeAlias { name, .. }) => name,
      Self::Handle(Handle { name, .. }) => name,
      Self::Enumeration(Enumeration { name, .. }) => name,
      Self::FuncPointer(FuncPointer { name, .. }) => name,
      Self::Structure(Structure { name, .. }) => name,
      Self::Union(Union { name, .. }) => name,
    }
  }
}

pub(crate) fn do_types(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  loop {
    match iter.next().unwrap() {
      EndTag { name: "types" } => return,
      StartTag { name: "comment", attrs: "" } => eat_to_end_of_comment(iter),
      StartTag { name: "type", attrs } => {
        let category = TagAttributeIterator::new(attrs)
          .find(|ta| ta.key == "category")
          .map(|ta| ta.value);
        match category {
          Some("include") => do_type_start_include(registry, attrs, iter),
          Some("define") => do_type_start_define(registry, attrs, iter),
          Some("basetype") => do_type_start_base(registry, attrs, iter),
          Some("bitmask") => do_type_start_bitmask(registry, attrs, iter),
          Some("handle") => do_type_start_handle(registry, attrs, iter),
          Some("funcpointer") => do_type_start_funcpointer(registry, attrs, iter),
          Some("struct") => do_type_start_struct(registry, attrs, iter),
          Some("union") => do_type_start_union(registry, attrs, iter),
          other => panic!("{other:?}"),
        }
      }
      EmptyTag { name: "type", attrs } => {
        let category = TagAttributeIterator::new(attrs)
          .find(|ta| ta.key == "category")
          .map(|ta| ta.value);
        match category {
          None => do_type_empty_none(registry, attrs),
          Some("handle") => {
            let type_alias = TypeAlias::from_attrs(attrs);
            debug!("{type_alias:?}");
            registry.types.push(TypeEntry::TypeAlias(type_alias));
          }
          Some("struct") => {
            let type_alias = TypeAlias::from_attrs(attrs);
            debug!("{type_alias:?}");
            registry.types.push(TypeEntry::TypeAlias(type_alias));
          }
          Some("include") => do_type_empty_include(registry, attrs),
          Some("bitmask") => do_type_empty_bitmask(registry, attrs),
          Some("enum") => do_type_empty_enum(registry, attrs),
          other => panic!("{other:?}"),
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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

pub(crate) fn do_type_empty_include(registry: &mut VulkanRegistry, attrs: StaticStr) {
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

pub(crate) fn do_type_empty_none(registry: &mut VulkanRegistry, attrs: StaticStr) {
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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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
  /// Only 64-bit flags use this field.
  ///
  /// For 32-bit flags the bit source is based on the `requires` field.
  pub bit_values: Option<StaticStr>,
  pub flags64: bool,
  // TODO: can we merge the above two fields into one?
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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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

pub(crate) fn do_type_empty_bitmask(registry: &mut VulkanRegistry, attrs: StaticStr) {
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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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

pub(crate) fn do_type_empty_enum(registry: &mut VulkanRegistry, attrs: StaticStr) {
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

/// A "funcpointer" type declaration.
///
/// These declarations have way less tagging, making them harder to parse. They
/// also only show up a handful of times, so it maybe doesn't matter and they
/// can just be converted by hand.
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
  registry: &mut VulkanRegistry, attrs: StaticStr,
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
  f.text = f.text.replace('\n', "");
  let mut replacement = f.text.replace("  ", " ");
  while f.text != replacement {
    f.text = replacement;
    replacement = f.text.replace("  ", " ");
  }
  debug!("{f:?}");
  registry.types.push(TypeEntry::FuncPointer(f));
}

#[derive(Debug, Clone, Default)]
pub struct Structure {
  pub name: StaticStr,
  pub members: Vec<Member>,
  pub returned_only: bool,
  pub struct_extends: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub allow_duplicate: bool,
}
impl Structure {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "struct"),
        "name" => x.name = value,
        "returnedonly" if value == "true" => x.returned_only = true,
        "structextends" => x.struct_extends = Some(value),
        "allowduplicate" if value == "true" => x.allow_duplicate = true,
        "allowduplicate" if value == "false" => (),
        "comment" => x.comment = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TypeVariant {
  /// `T`
  #[default]
  Normal,
  /// `*const T`
  ConstPtr,
  /// `*mut T`
  MutPtr,
  /// `[T; CONST_NAME]`
  ArraySym(StaticStr),
  /// `[T; {usize}]`
  ArrayInt(usize),
  /// `[[T; {usize0}] {usize1}]`
  ArrayArrayInt(usize, usize),
  /// `*const *const T`
  ConstPtrConstPtr,
  /// `*mut *mut T`
  MutPtrMutPtr,
  /// `*const [T; {usize}]`
  ConstPtrArrayInt(usize),
}

#[derive(Debug, Clone, Default)]
pub struct Member {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub optional: Option<StaticStr>,
  pub no_auto_validity: bool,
  pub limit_type: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  /// This field should *always* contain the named enumeration value.
  ///
  /// If we're generating Default impls or something like that, we should set
  /// the field's default value to this value.
  pub value: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub deprecated: Option<StaticStr>,
  pub api: Option<StaticStr>,
  /// The name of the *other* field that determines the object type of this
  /// field.
  ///
  /// This field will be a `u64`.
  pub object_type: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  /// (union only) Names the enumeration value that the selecting field *of the
  /// containing struct* will have when this field is the intended field.
  pub selection: Option<StaticStr>,
  /// (struct only) Designates that this field holds the enumeration for what
  /// variant of the union field named is the intended variant.
  pub selector: Option<StaticStr>,
  pub bitfields: Option<u32>,
}
impl Member {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "optional" => x.optional = Some(value),
        "noautovalidity" if value == "true" => x.no_auto_validity = true,
        "limittype" => x.limit_type = Some(value),
        "values" => {
          assert!(!value.contains(','));
          x.value = Some(value);
        }
        "len" => x.len = Some(value),
        "altlen" => x.alt_len = Some(value),
        "deprecated" => x.deprecated = Some(value),
        "api" => x.api = Some(value),
        "objecttype" => x.object_type = Some(value),
        "externsync" => x.extern_sync = Some(value),
        "selection" => x.selection = Some(value),
        "selector" => x.selector = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
pub(crate) fn do_member_start(
  attrs: StaticStr, iter: &mut impl Iterator<Item = XmlElement<'static>>,
) -> Member {
  let mut m = Member::from_attrs(attrs);
  'member_ty: loop {
    match iter.next().unwrap() {
      Text("struct") => continue,
      Text("const struct") | Text("const") => {
        m.ty_variant = TypeVariant::ConstPtr;
      }
      StartTag { name: "type", attrs: "" } => {
        m.ty = iter.next().unwrap().unwrap_text();
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
        break 'member_ty;
      }
      other => panic!("{other:?}"),
    }
  }
  'member_name: loop {
    match iter.next().unwrap() {
      Text("*") if m.ty_variant == TypeVariant::Normal => {
        m.ty_variant = TypeVariant::MutPtr
      }
      Text("*") if m.ty_variant == TypeVariant::ConstPtr => {
        // no further changes happen here.
      }
      Text("* const*") | Text("* const *") if m.ty_variant == TypeVariant::ConstPtr => {
        m.ty_variant = TypeVariant::ConstPtrConstPtr;
      }
      StartTag { name: "name", attrs: "" } => {
        m.name = iter.next().unwrap().unwrap_text();
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
        break 'member_name;
      }
      other => panic!("{other:?}"),
    }
  }
  'member_cleanup: loop {
    match iter.next().unwrap() {
      EndTag { name: "member" } => break 'member_cleanup,
      Text("[") => {
        assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("enum", ""));
        match m.ty_variant {
          TypeVariant::Normal => {
            m.ty_variant = TypeVariant::ArraySym(iter.next().unwrap().unwrap_text())
          }
          other => panic!("{other:?}"),
        }
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "enum");
        assert_eq!(iter.next().unwrap().unwrap_text(), "]");
      }
      Text("[2]") if m.ty_variant == TypeVariant::Normal => {
        m.ty_variant = TypeVariant::ArrayInt(2);
      }
      Text("[3]") if m.ty_variant == TypeVariant::Normal => {
        m.ty_variant = TypeVariant::ArrayInt(3);
      }
      Text("[4]") if m.ty_variant == TypeVariant::Normal => {
        m.ty_variant = TypeVariant::ArrayInt(4);
      }
      Text("[3][4]") if m.ty_variant == TypeVariant::Normal => {
        m.ty_variant = TypeVariant::ArrayArrayInt(3, 4);
      }
      Text(":8") => {
        m.bitfields = Some(8);
      }
      Text(":24") => {
        m.bitfields = Some(24);
      }
      StartTag { name: "comment", attrs: "" } => {
        m.comment = Some(iter.next().unwrap().unwrap_text());
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
      }
      other => panic!("{other:?}"),
    }
  }
  trace!("{m:?}");
  m
}

pub(crate) fn do_type_start_struct(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut s = Structure::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      StartTag { name: "comment", attrs: "" } => {
        s.comment = Some(iter.next().unwrap().unwrap_text());
        assert_eq!(iter.next().unwrap().unwrap_end_tag(), "comment");
      }
      StartTag { name: "member", attrs } => s.members.push(do_member_start(attrs, iter)),
      other => panic!("{other:?}"),
    }
  }
  debug!("{s:?}");
  registry.types.push(TypeEntry::Structure(s));
}

#[derive(Debug, Clone, Default)]
pub struct Union {
  pub name: StaticStr,
  pub members: Vec<Member>,
  pub comment: Option<StaticStr>,
  pub returned_only: bool,
}
impl Union {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "category" => assert_eq!(value, "union"),
        "name" => x.name = value,
        "comment" => x.comment = Some(value),
        "returnedonly" if value == "true" => x.returned_only = true,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

pub(crate) fn do_type_start_union(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  let mut u = Union::from_attrs(attrs);
  'ty: loop {
    match iter.next().unwrap() {
      EndTag { name: "type" } => break 'ty,
      StartTag { name: "member", attrs } => u.members.push(do_member_start(attrs, iter)),
      other => panic!("{other:?}"),
    }
  }
  debug!("{u:?}");
  registry.types.push(TypeEntry::Union(u));
}
