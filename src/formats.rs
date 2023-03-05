use std::path::Component;

use super::*;

pub(crate) fn do_formats(
  registry: &mut VulkanRegistry, attrs: StaticStr,
  iter: &mut impl Iterator<Item = XmlElement<'static>>,
) {
  assert_attrs_comment_only!(attrs);
  'formats: loop {
    match iter.next().unwrap() {
      EndTag { name: "formats" } => break 'formats,
      StartTag { name: "format", attrs } => {
        let mut format = Format::from_attrs(attrs);
        'format: loop {
          match iter.next().unwrap() {
            EndTag { name: "format" } => {
              debug!("{format:?}");
              registry.formats.push(format);
              break 'format;
            }
            EmptyTag { name: "component", attrs } => {
              let component = FormatComponent::from_attrs(attrs);
              trace!("{component:?}");
              format.components.push(component);
            }
            EmptyTag { name: "spirvimageformat", attrs } => {
              assert_eq!(TagAttributeIterator::new(attrs).count(), 1);
              let ta =
                TagAttributeIterator::new(attrs).find(|ta| ta.key == "name").unwrap();
              format.spirv_image_format = Some(ta.value);
            }
            EmptyTag { name: "plane", attrs } => {
              let plane = FormatPlane::from_attrs(attrs);
              trace!("{plane:?}");
              format.planes.push(plane);
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
pub struct Format {
  pub name: StaticStr,
  pub class: StaticStr,
  pub block_size: StaticStr,
  pub texels_per_block: StaticStr,
  pub packed: StaticStr,
  pub components: Vec<FormatComponent>,
  pub spirv_image_format: Option<StaticStr>,
  pub block_extent: Option<StaticStr>,
  pub compressed: Option<StaticStr>,
  pub chroma: Option<StaticStr>,
  pub planes: Vec<FormatPlane>,
}
impl Format {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "class" => x.class = value,
        "blockSize" => x.block_size = value,
        "texelsPerBlock" => x.texels_per_block = value,
        "packed" => x.packed = value,
        "blockExtent" => x.block_extent = Some(value),
        "compressed" => x.compressed = Some(value),
        "chroma" => x.chroma = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormatComponent {
  pub name: StaticStr,
  pub bits: StaticStr,
  pub numeric_format: StaticStr,
  pub plane_index: Option<StaticStr>,
}
impl FormatComponent {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "bits" => x.bits = value,
        "numericFormat" => x.numeric_format = value,
        "planeIndex" => x.plane_index = Some(value),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormatPlane {
  pub name: StaticStr,
  pub index: StaticStr,
  pub width_divisor: StaticStr,
  pub height_divisor: StaticStr,
  pub compatible: StaticStr,
}
impl FormatPlane {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "index" => x.index = value,
        "widthDivisor" => x.width_divisor = value,
        "heightDivisor" => x.height_divisor = value,
        "compatible" => x.compatible = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
