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
  pub block_size: u32,
  pub texels_per_block: u32,
  pub packed: u32,
  pub components: Vec<FormatComponent>,
  pub spirv_image_format: Option<StaticStr>,
  pub block_extent: Option<StaticStr>,
  pub compressed: Option<StaticStr>,
  pub chroma: Option<u32>,
  pub planes: Vec<FormatPlane>,
}
impl Format {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "class" => x.class = value,
        "blockSize" => x.block_size = value.parse().unwrap(),
        "texelsPerBlock" => x.texels_per_block = value.parse().unwrap(),
        "packed" => x.packed = value.parse().unwrap(),
        "blockExtent" => x.block_extent = Some(value),
        "compressed" => x.compressed = Some(value),
        "chroma" => {
          x.chroma = Some(match value {
            "420" => 420,
            "422" => 422,
            "444" => 444,
            other => panic!("{other:?}"),
          })
        }
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FormatComponentName {
  #[default]
  R,
  G,
  B,
  A,
  S,
  D,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NumericFormat {
  ///signed floating-point numbers
  SFLOAT,
  ///signed integer values in the range [-2n-1,2n-1-1]
  SINT,
  ///signed normalized values in the range [-1,1]
  SNORM,
  ///R, G, and B components are unsigned normalized values that represent
  /// values using sRGB nonlinear encoding, while the A component (if one
  /// exists) is a regular unsigned normalized value
  #[default]
  SRGB,
  ///signed integer values that get converted to floating-point in the range
  /// [-2n-1,2n-1-1]
  SSCALED,
  ///unsigned floating-point numbers (used by packed, shared exponent, and some
  /// compressed formats)
  UFLOAT,
  ///unsigned integer values in the range [0,2n-1]
  UINT,
  ///unsigned normalized values in the range [0,1]
  UNORM,
  ///unsigned integer values that get converted to floating-point in the range
  /// [0,2n-1]
  USCALED,
}

#[derive(Debug, Clone, Default)]
pub struct FormatComponent {
  pub name: FormatComponentName,
  /// This is either "compressed" or an actual number.
  pub bits: StaticStr,
  pub numeric_format: NumericFormat,
  pub plane_index: Option<u32>,
}
impl FormatComponent {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => {
          x.name = match value {
            "R" => FormatComponentName::R,
            "G" => FormatComponentName::G,
            "B" => FormatComponentName::B,
            "A" => FormatComponentName::A,
            "S" => FormatComponentName::S,
            "D" => FormatComponentName::D,
            other => panic!("{other:?}"),
          }
        }
        "bits" => x.bits = value,
        "numericFormat" => {
          x.numeric_format = match value {
            "SFLOAT" => NumericFormat::SFLOAT,
            "SINT" => NumericFormat::SINT,
            "SNORM" => NumericFormat::SNORM,
            "SRGB" => NumericFormat::SRGB,
            "SSCALED" => NumericFormat::SSCALED,
            "UFLOAT" => NumericFormat::UFLOAT,
            "UINT" => NumericFormat::UINT,
            "UNORM" => NumericFormat::UNORM,
            "USCALED" => NumericFormat::USCALED,
            other => panic!("{other:?}"),
          }
        }
        "planeIndex" => x.plane_index = Some(value.parse().unwrap()),
        other => panic!("{other:?}"),
      }
    }
    x
  }
}

#[derive(Debug, Clone, Default)]
pub struct FormatPlane {
  pub name: StaticStr,
  pub index: u32,
  pub width_divisor: u32,
  pub height_divisor: u32,
  pub compatible: StaticStr,
}
impl FormatPlane {
  pub fn from_attrs(attrs: StaticStr) -> Self {
    let mut x = Self::default();
    for TagAttribute { key, value } in TagAttributeIterator::new(attrs) {
      match key {
        "name" => x.name = value,
        "index" => x.index = value.parse().unwrap(),
        "widthDivisor" => x.width_divisor = value.parse().unwrap(),
        "heightDivisor" => x.height_divisor = value.parse().unwrap(),
        "compatible" => x.compatible = value,
        other => panic!("{other:?}"),
      }
    }
    x
  }
}
