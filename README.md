# vk_dot_xml_parser

Parser for the Vulkan `vk.xml` file

## Stability

This library is not necessarily stable, nor is it likely to ever be fully stable.

As `vk.xml` updates, the library will update as well.
If the XML adds attributes, we may have to add fields.
If the XML changes the nature of an attribute, we may have to change the type of existing fields.
Because we're at the whim of what Khronos does with `vk.xml`, we can't be sure of stability.
Similarly, things become simpler when there's less "string-ly typed data",
so converting fields away from being strings will happen when possible.

All that said, normal semver will of course be used.
