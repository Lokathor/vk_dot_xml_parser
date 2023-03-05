use super::*;

pub(crate) fn do_extensions(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  'extensions: loop {
    match iter.next().unwrap() {
      EndTag { name: "extensions" } => break 'extensions,
      StartTag { name: "extension", attrs } => {
        let mut extension = Extension::from_attrs(attrs);
        'extension: loop {
          match iter.next().unwrap() {
            EndTag { name: "extension" } => {
              debug!("{extension:?}");
              registry.extensions.push(extension);
              break 'extension;
            }
            StartTag { name: "require", attrs } => {
              let mut requirement = Requirement::from_attrs(attrs);
              'require: loop {
                match iter.next().unwrap() {
                  EndTag { name: "require" } => {
                    trace!("{requirement:?}");
                    extension.requirements.push(requirement);
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
                    } else if TagAttributeIterator::new(attrs)
                      .any(|ta| ta.key == "bitpos")
                    {
                      let e = RequiredEnumBitpos::from_attrs(attrs);
                      trace!("{e:?}");
                      requirement.required_bitpos_enums.push(e);
                    } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "alias")
                    {
                      let e = RequiredEnumAlias::from_attrs(attrs);
                      trace!("{e:?}");
                      requirement.required_alias_enums.push(e);
                    } else if TagAttributeIterator::new(attrs).any(|ta| ta.key == "value")
                    {
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
            other => panic!("{other:?}"),
          }
        }
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtensionType {
  #[default]
  Instance,
  Device,
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
  pub name: StaticStr,
  pub number: StaticStr,
  pub ty: ExtensionType,
  pub author: StaticStr,
  pub contact: StaticStr,
  pub supported: StaticStr,
  pub requirements: Vec<Requirement>,
  pub depends: Option<StaticStr>,
  pub platform: Option<StaticStr>,
  pub comment: Option<StaticStr>,
  pub special_use: Option<StaticStr>,
  pub deprecated_by: Option<StaticStr>,
  pub promoted_to: Option<StaticStr>,
  pub obsoleted_by: Option<StaticStr>,
  pub provisional: bool,
  pub sort_order: Option<StaticStr>,
}
impl Extension {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "number" => x.number = value,
        "type" if value == "instance" => x.ty = ExtensionType::Instance,
        "type" if value == "device" => x.ty = ExtensionType::Device,
        "author" => x.author = value,
        "contact" => x.contact = value,
        "supported" => x.supported = value,
        "depends" => x.depends = Some(value),
        "platform" => x.platform = Some(value),
        "comment" => x.comment = Some(value),
        "specialuse" => x.special_use = Some(value),
        "deprecatedby" => x.deprecated_by = Some(value),
        "promotedto" => x.promoted_to = Some(value),
        "obsoletedby" => x.obsoleted_by = Some(value),
        "provisional" if value == "true" => x.provisional = true,
        "sortorder" => x.obsoleted_by = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
