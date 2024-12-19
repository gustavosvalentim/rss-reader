use std::option::Option::Some;

use roxmltree::{Document, Node};

#[derive(Debug, Clone, Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
}

impl Channel {
    /// Creates a Channel from a XML string
    ///
    /// # Examples
    ///
    /// ## From strings
    ///
    /// ```
    /// use rss_parser::Channel;
    ///
    /// let content = "<channel><title>Test</title></channel>";
    /// let channel = Channel::from(content);
    /// ```
    pub fn from(content: &str) -> Option<Channel> {
        let doc = match Document::parse(content) {
            Ok(doc) => doc,
            Err(_) => return None,
        };

        // TODO: Read attributes like channel link, item content and item creator
        // from `xmlns` attributes from the `rss` element.
        //
        // the example below shows how to get the namespace URI from the root element
        //
        // for ns in doc.root_element().namespaces() {
        //     println!("doc(ns): {}", ns.uri());
        // }

        let mut channel = Channel::default();

        let elements = doc
            .descendants()
            .find(|n| n.tag_name().name() == "channel")?
            .children();

        for element in elements {
            match element.tag_name().name() {
                "title" => {
                    if let Some(title) = element.text() {
                        channel.title = String::from(title);
                    }
                }
                "description" => {
                    if let Some(description) = element.text() {
                        channel.description = String::from(description);
                    }
                }
                "link" => {
                    if element.tag_name().namespace() != Some("http://www.w3.org/2005/Atom") {
                        continue;
                    }

                    if let Some(link) = element.attribute("href") {
                        channel.link = String::from(link);
                    }
                }
                "item" => {
                    if let Some(item) = Item::from(element) {
                        channel.items.push(item);
                    }
                }
                _ => continue,
            }
        }

        Some(channel)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub content: String,
    pub author: String,
    pub categories: Vec<String>,
}

impl Item {
    fn from(node: Node) -> Option<Item> {
        let mut item = Item::default();

        for element in node.children() {
            match element.tag_name().name() {
                "title" => {
                    if let Some(content) = element.text() {
                        item.title = String::from(content)
                    }
                }
                "encoded" => {
                    if let Some(content) = element.text() {
                        item.content = String::from(content)
                    }
                }
                "description" => {
                    if let Some(content) = element.text() {
                        item.description = String::from(content)
                    }
                }
                "creator" => {
                    if let Some(content) = element.text() {
                        item.author = String::from(content)
                    }
                }
                "category" => {
                    if let Some(content) = element.text() {
                        item.categories.push(String::from(content));
                    }
                }
                _ => continue,
            }
        }

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::Channel;
    use tokio;

    #[tokio::test]
    async fn test_channel_from_xml_string() {
        let content = "<channel><title>Test channel</title><description>This is a test channel</description></channel>";
        let channel = Channel::from(content).unwrap();

        assert_eq!(channel.title, "Test channel");
        assert_eq!(channel.description, "This is a test channel");
    }
}
