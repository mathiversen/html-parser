use super::element::Element;
use crate::Text;
use crate::for_each::ForEach;
use html_escape::decode_html_entities;
use ownable::{IntoOwned, ToBorrowed, ToOwned};
#[cfg(feature = "test")]
use serde::Serialize;
use std::array;
use std::borrow::Cow;

#[derive(PartialEq, IntoOwned, ToBorrowed, ToOwned)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(untagged))]
pub enum Node<'a> {
    Text(Text<'a>),
    Element(Element<'a>),
    Comment(Text<'a>),
}

impl<'a> Node<'a> {
    pub fn text(&self) -> Option<&Text<'a>> {
        match self {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn text_str(&self) -> Option<&str> {
        match self {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn element(&self) -> Option<&Element<'a>> {
        match self {
            Node::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn comment(&self) -> Option<&Text<'a>> {
        match self {
            Node::Comment(t) => Some(t),
            _ => None,
        }
    }

    pub fn comment_str(&self) -> Option<&str> {
        match self {
            Node::Comment(t) => Some(t),
            _ => None,
        }
    }
}

impl<'a> ForEach<'a> for Node<'a> {
    #[inline]
    fn root(&self) -> &[Node<'a>] {
        array::from_ref(self)
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [Node<'a>] {
        array::from_mut(self)
    }
}

impl Node<'_> {
    #[inline]
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        self.write_html(&mut result);
        result
    }

    #[inline]
    pub fn write_html(&self, writer: &mut String) {
        match self {
            Node::Text(s) => {
                writer.push_str(s);
            }
            Node::Element(e) => e.write_html(writer),
            Node::Comment(c) => {
                writer.push_str("<!--");
                writer.push_str(c);
                writer.push_str("-->");
            }
        }
    }

    /// Strip all tags.
    ///
    /// Note that html entities are not removed, only tags.
    /// Use [`Self::to_text`], or [`Text::decode`] on the result.
    pub fn strip_tags(&self) -> Text<'_> {
        match self {
            Node::Text(t) => t.to_borrowed(),
            Node::Element(e) => e.strip_tags(),
            Node::Comment(_) => "".into(),
        }
    }

    /// Strip tags and decode html-entities.
    pub fn to_text(&self) -> Cow<'_, str> {
        let t = self.strip_tags().0;
        match decode_html_entities(&t) {
            Cow::Borrowed(_) => t, // it's only returned as borrowed if the whole input is passed though
            Cow::Owned(o) => Cow::Owned(o),
        }
    }
}

pub(crate) fn write_html_list(writer: &mut String, list: &[Node]) {
    for n in list {
        n.write_html(writer);
    }
}

impl<'a> IntoIterator for &'a Node<'a> {
    type Item = &'a Node<'a>;
    type IntoIter = NodeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIterator {
            node: self,
            index: vec![],
        }
    }
}

pub struct NodeIntoIterator<'a> {
    node: &'a Node<'a>,
    // We add/remove to this vec each time we go up/down a node three
    index: Vec<(usize, &'a Node<'a>)>,
}

impl<'a> Iterator for NodeIntoIterator<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get first child
        let child = match self.node {
            Node::Element(e) => e.children.first(),
            _ => None,
        };

        match child {
            // If element has child, return child
            Some(child) => {
                self.index.push((0, self.node));
                self.node = child;
                Some(child)
            }
            // If element doesn't have a child, but is a child of another node
            None if !self.index.is_empty() => {
                let mut has_finished = false;
                let mut next_node = None;

                while !has_finished {
                    // Try to get the next sibling of the parent node
                    if let Some((sibling_index, parent)) = self.index.pop() {
                        let next_sibling = sibling_index + 1;
                        let sibling = if let Node::Element(e) = parent {
                            e.children.get(next_sibling)
                        } else {
                            None
                        };
                        if sibling.is_some() {
                            has_finished = true;
                            self.index.push((next_sibling, parent));
                            next_node = sibling;
                        } else {
                            continue;
                        }
                    // Break of there are no more parents
                    } else {
                        has_finished = true;
                    }
                }

                if let Some(next_node) = next_node {
                    self.node = next_node;
                }

                next_node
            }
            _ => None,
        }
    }
}

#[cfg(feature = "test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_utillity_functions() {
        let node = Node::Text("test".into());

        assert_eq!(node.text_str(), Some("test"));
        assert_eq!(node.element(), None);
        assert_eq!(node.comment_str(), None);

        let node = Node::Element(Element::default());

        assert_eq!(node.text_str(), None);
        assert_eq!(node.element(), Some(&Element::default()));
        assert_eq!(node.comment_str(), None);

        let node = Node::Comment("test".into());

        assert_eq!(node.text_str(), None);
        assert_eq!(node.element(), None);
        assert_eq!(node.comment_str(), Some("test"));
    }
}
