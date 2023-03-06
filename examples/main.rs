use vk_dot_xml_parser::*;

fn main() {
  stderrlog::new().verbosity(0).init().unwrap();
  //
  let vk_xml: StaticStr =
    Box::leak(std::fs::read_to_string("vk.xml").unwrap().into_boxed_str());
  let registry = VulkanRegistry::from_static_str(vk_xml);
  println!(
    "// Instance Fn Table Count: {}",
    registry
      .commands
      .iter()
      .filter(|c| ["VkInstance", "VkPhysicalDevice"].contains(&c.params[0].ty))
      .count()
  );
  println!(
    "// Device Fn Table Count: {}",
    registry
      .commands
      .iter()
      .filter(|c| ["VkDevice", "VkQueue", "VkCommandBuffer"].contains(&c.params[0].ty))
      .count()
  );
  println!("// Total Command Count: {}", registry.commands.len());
  println!("{registry:#?}");
}
