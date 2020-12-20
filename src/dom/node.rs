use super::element::Element;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a Node;
    type IntoIter = NodeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIterator {
            node: self,
            index: vec![],
        }
    }
}

pub struct NodeIntoIterator<'a> {
    node: &'a Node,
    // We add/remove to this vec each time we go up/down a node three
    index: Vec<(usize, &'a Node)>,
}

impl<'a> Iterator for NodeIntoIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        // Get first child
        let child = match self.node {
            Node::Element(ref e) => e.children.get(0),
            _ => None,
        };

        let result = match child {
            // If element has child, return child
            Some(child) => {
                self.index.push((0, self.node));
                self.node = child;
                Some(child)
            }
            // If element doesn't have a child, but is a child of another node
            None if self.index.len() > 0 => {
                let mut has_finished = false;
                let mut next_node = None;

                while !has_finished {
                    // Try to get the next sibling of the parent node
                    if let Some((sibling_index, parent)) = self.index.pop() {
                        let next_sibling = sibling_index + 1;
                        let sibling = if let Node::Element(ref e) = parent {
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
        };

        result
    }
}
