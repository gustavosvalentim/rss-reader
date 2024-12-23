use roxmltree::{Document, Node};

pub mod extensions;

use crate::extensions::atom;
use crate::extensions::dublincore;

/// Channel of an RSS feed
#[derive(Debug, Clone, Default)]
pub struct Channel {
    /// Title of the channel
    pub title: String,
    /// The URL for the website of this channel
    pub link: String,
    /// Description of the channel
    pub description: String,
    /// List of items (or articles) in the channel
    pub items: Vec<Item>,
    /// Atom extension for the channel
    pub atom: Option<String>,
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
            Err(error) => {
                eprintln!("Failed to parse XML: {}", error);
                return None;
            }
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
                    // TODO: ideally this would be handled by the base condition
                    // but `roxmltree` does not return tag names with prefixes
                    // so `atom:link` is returned as `link`
                    if element.tag_name().namespace() == Some(atom::NAMESPACE) {
                        if let Some(link) = atom::AtomExtension::get_value(element) {
                            channel.atom = Some(link);
                            continue;
                        }
                    }

                    if let Some(link) = element.text() {
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
    // Title of the item
    pub title: String,
    // Description of the item
    pub description: String,
    // The HTML content
    pub content: String,
    // Email address of the author of the item
    pub author: String,
    // Categories this item belongs to
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
                "category" => {
                    if let Some(content) = element.text() {
                        item.categories.push(String::from(content));
                    }
                }
                _ => match element.tag_name().namespace() {
                    Some(dublincore::NAMESPACE) => {
                        if let Some(content) = dublincore::DublinCoreExtension::get_value(element) {
                            item.author = content;
                        }
                    }
                    _ => continue,
                },
            }
        }

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::Channel;
    use tokio;

    const TEST_CHANNEL: &'static str = r#"
        <rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:dc="http://purl.org/dc/elements/1.1/">
            <channel>
                <title>Test channel</title>
                <description>This is a test channel</description>
                <link>https://example.com</link>
                <atom:link href="https://example.com/feed.xml"/>
                <item>
                    <title>Test item</title>
                    <description>This is a test item</description>
                    <dc:creator>Test author</dc:creator>
                    <category>Test category</category>
                </item>
            </channel>
        </rss>
    "#;

    #[tokio::test]
    async fn test_channel_from_xml_string() {
        let channel = Channel::from(TEST_CHANNEL).unwrap();

        assert_eq!(channel.title, "Test channel");
        assert_eq!(channel.description, "This is a test channel");
    }
}
