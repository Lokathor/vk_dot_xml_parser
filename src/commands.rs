use super::*;

pub fn do_commands(
  registry: &mut Registry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  'commands: loop {
    match iter.next().unwrap() {
      EndTag { name: "commands" } => return,
      StartTag { name: "command", attrs } => {
        let mut command = Command::from_attrs(attrs);
        'command: loop {
          match iter.next().unwrap() {
            EndTag { name: "command" } => {
              debug!("{command:?}");
              registry.commands.push(command);
              break 'command;
            }
            StartTag { name: "proto", attrs: "" } => {
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("type", ""));
              command.return_ty = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("name", ""));
              command.name = iter.next().unwrap().unwrap_text();
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
              //
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "proto");
            }
            StartTag { name: "param", attrs } => {
              let mut param = Param::from_attrs(attrs);
              'ty: loop {
                match iter.next().unwrap() {
                  Text("struct") => (),
                  Text("const") | Text("const struct") => {
                    param.ty_variant = TypeVariant::ConstPtr
                  }
                  StartTag { name: "type", attrs: "" } => {
                    param.ty = iter.next().unwrap().unwrap_text();
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "type");
                    break 'ty;
                  }
                  other => panic!("{other:?}"),
                }
              }
              'name: loop {
                match iter.next().unwrap() {
                  Text("*") if param.ty_variant == TypeVariant::Normal => {
                    param.ty_variant = TypeVariant::MutPtr
                  }
                  Text("*") if param.ty_variant == TypeVariant::ConstPtr => {
                    // no further changes happen here.
                  }
                  Text("**") if param.ty_variant == TypeVariant::Normal => {
                    param.ty_variant = TypeVariant::MutPtrMutPtr
                  }
                  Text("* const*") | Text("* const *")
                    if param.ty_variant == TypeVariant::ConstPtr =>
                  {
                    param.ty_variant = TypeVariant::ConstPtrConstPtr;
                  }
                  StartTag { name: "name", attrs: "" } => {
                    param.name = iter.next().unwrap().unwrap_text();
                    assert_eq!(iter.next().unwrap().unwrap_end_tag(), "name");
                    break 'name;
                  }
                  other => panic!("{other:?}"),
                }
              }
              'cleanup: loop {
                match iter.next().unwrap() {
                  EndTag { name: "param" } => break 'cleanup,
                  Text("[2]") if param.ty_variant == TypeVariant::ConstPtr => {
                    param.ty_variant = TypeVariant::ConstPtrArrayInt(2);
                  }
                  Text("[3]") if param.ty_variant == TypeVariant::ConstPtr => {
                    param.ty_variant = TypeVariant::ConstPtrArrayInt(3);
                  }
                  Text("[4]") if param.ty_variant == TypeVariant::ConstPtr => {
                    param.ty_variant = TypeVariant::ConstPtrArrayInt(4);
                  }
                  other => panic!("{other:?}"),
                }
              }
              trace!("{param:?}");
              command.params.push(param);
            }
            StartTag { name: "implicitexternsyncparams", attrs: "" } => {
              assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("param", ""));
              command.implicit_extern_sync_params =
                Some(iter.next().unwrap().unwrap_text());
              assert_eq!(iter.next().unwrap().unwrap_end_tag(), "param");
              assert_eq!(
                iter.next().unwrap().unwrap_end_tag(),
                "implicitexternsyncparams"
              );
            }
            other => panic!("{other:?}"),
          }
        }
      }
      EmptyTag { name: "command", attrs } => {
        let alias = CommandAlias::from_attrs(attrs);
        debug!("{alias:?}");
        registry.command_aliases.push(alias);
      }
      other => panic!("{other:?}"),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct CommandAlias {
  pub name: StaticStr,
  pub alias_of: StaticStr,
}
impl CommandAlias {
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

#[derive(Debug, Clone, Default)]
pub struct Command {
  pub name: StaticStr,
  pub params: Vec<Param>,
  pub return_ty: StaticStr,
  pub comment: Option<StaticStr>,
  pub success_codes: Option<StaticStr>,
  pub error_codes: Option<StaticStr>,
  pub implicit_extern_sync_params: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub queues: Option<StaticStr>,
  pub render_pass: Option<StaticStr>,
  pub cmd_buffer_level: Option<StaticStr>,
  pub tasks: Option<StaticStr>,
  pub video_coding: Option<StaticStr>,
}
impl Command {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "successcodes" => x.success_codes = Some(value),
        "errorcodes" => x.error_codes = Some(value),
        "api" => x.api = Some(value),
        "queues" => x.queues = Some(value),
        "renderpass" => x.render_pass = Some(value),
        "cmdbufferlevel" => x.cmd_buffer_level = Some(value),
        "tasks" => x.tasks = Some(value),
        "comment" => x.comment = Some(value),
        "videocoding" => x.video_coding = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct Param {
  pub name: StaticStr,
  pub ty: StaticStr,
  pub ty_variant: TypeVariant,
  pub optional: Option<StaticStr>,
  pub extern_sync: Option<StaticStr>,
  pub len: Option<StaticStr>,
  pub alt_len: Option<StaticStr>,
  pub api: Option<StaticStr>,
  pub no_auto_validity: bool,
  pub stride: Option<StaticStr>,
  pub object_type: Option<StaticStr>,
  pub valid_structs: Option<StaticStr>,
}
impl Param {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "optional" => x.optional = Some(value),
        "externsync" => x.extern_sync = Some(value),
        "len" => x.len = Some(value),
        "api" => x.api = Some(value),
        "noautovalidity" if value == "true" => x.no_auto_validity = true,
        "stride" => x.stride = Some(value),
        "objecttype" => x.object_type = Some(value),
        "altlen" => x.alt_len = Some(value),
        "validstructs" => x.valid_structs = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
