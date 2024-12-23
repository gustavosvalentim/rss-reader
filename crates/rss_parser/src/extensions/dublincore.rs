use roxmltree::Node;

pub const NAMESPACE: &'static str = "http://purl.org/dc/elements/1.1/";

#[derive(Debug, Default, Clone)]
pub struct DublinCoreExtension {}

impl DublinCoreExtension {
    pub fn get_value(element: Node) -> Option<String> {
        if let Some(content) = element.text() {
            return Some(content.to_string());
        }

        None
    }
}
