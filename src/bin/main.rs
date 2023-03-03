use vk_dot_xml_parser::*;

fn main() {
  stderrlog::new().verbosity(4).init().unwrap();
  //
  let vk_xml: StaticStr =
    Box::leak(std::fs::read_to_string("vk.xml").unwrap().into_boxed_str());
  let _registry = Registry::from_static_str(vk_xml);
  println!("Parsed The Registry!");
}
