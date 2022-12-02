use indextree::NodeId;
use std::io::Write;

use crate::document::Document;
use crate::error::Error;
use crate::xmlnode::XmlNode;

impl<'a> Document<'a> {
    pub fn serialize(
        self: &Document<'a>,
        node_id: NodeId,
        w: &mut impl Write,
    ) -> Result<(), Error> {
        let xml_node = self.data.arena.get(node_id).unwrap().get();
        match xml_node {
            XmlNode::Root => {
                for child in node_id.children(&self.data.arena) {
                    self.serialize(child, w)?;
                }
            }
            XmlNode::Element(element) => {
                let fullname = self.fullname(node_id, element.name_id)?;
                write!(w, "<{}", fullname)?;
                for (prefix_id, namespace_id) in element.namespace_info.to_namespace.iter() {
                    let prefix = self.data.prefix_lookup.get_value(*prefix_id);
                    let namespace = self.data.namespace_lookup.get_value(*namespace_id);
                    write!(w, " xmlns:{}=\"{}\"", prefix, namespace)?;
                }
                let mut children_ids = node_id.children(&self.data.arena).peekable();
                if children_ids.peek().is_none() {
                    write!(w, "/>")?;
                } else {
                    write!(w, ">")?;
                    for child_id in children_ids {
                        self.serialize(child_id, w)?;
                    }
                    write!(w, "</{}>", fullname)?;
                }
            }
            XmlNode::Text(text) => {
                write!(w, "{}", text)?;
            }
        }
        Ok(())
    }
}
