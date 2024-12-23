use roxmltree::Node;

pub const NAMESPACE: &'static str = "http://www.w3.org/2005/Atom";

#[derive(Debug, Default, Clone)]
pub struct AtomExtension {}

impl AtomExtension {
    pub fn get_value(element: Node) -> Option<String> {
        if let Some(link) = element.attribute("href") {
            return Some(link.to_string());
        }

        None
    }
}
