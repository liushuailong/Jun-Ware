// document 数据结构
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData{
            tag_name: name,
            attributes: attrs,
        }),
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        // HashMap的get方法？？？
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        // match的使用？？？深入理解
        match self.attributes.get("class") {
            Some(classList) => classList.split(" ").collect(),
            None => HashSet::new(),
        }
    }
}