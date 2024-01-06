use edita_core::Block;
use hirola::prelude::html;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{Editor, EditorState, node::EditorNode};

pub struct TextNodeBlock;

#[derive(Clone, Serialize, Debug)]
pub struct TextNode {
    text: String,
}

impl Block for TextNodeBlock {
    type Node = EditorNode;
    type Input = web_sys::Node;
    type State = EditorState;
    fn accepts(&self, node: &Node) -> bool {
        node.node_type() == Node::TEXT_NODE
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &Node,
    ) -> EditorNode {
        if let Some(text_content) = node.text_content() {
            EditorNode::Text(TextNode { text: text_content })
        } else {
            panic!("not text")
        }
    }
}

impl crate::node::Node for TextNode {
    fn render(&self) -> hirola::dom::Dom {
        html! { <>{&self.text}</> }
    }
}

pub struct BoldBlock;

impl Block for BoldBlock {
    type Node = EditorNode;
    type Input = web_sys::Node;
    type State = EditorState;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "B" || element.tag_name() == "STRONG"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &Node,
    ) -> EditorNode {
        EditorNode::Bold(Bold {
            text: node.text_content().unwrap_or_default(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Bold {
    text: String,
}

impl crate::node::Node for Bold {
    fn render(&self) -> hirola::dom::Dom {
        html! { <b>{&self.text}</b> }
    }
}

pub struct ItalicBlock;

impl Block for ItalicBlock {
    type Node = EditorNode;
    type Input = Node;
    type State = EditorState;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "I" || element.tag_name() == "EM"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &Node,
    ) -> EditorNode {
        EditorNode::Italic(Italic {
            text: node.text_content().unwrap_or_default(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Italic {
    text: String,
}

impl crate::node::Node for Italic {
    fn render(&self) -> hirola::dom::Dom {
        html! { <em>{&self.text}</em> }
    }
}

pub struct InlineCodeBlock;

impl Block for InlineCodeBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "CODE"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::InlineCode(InlineCode {
            text: node.text_content().unwrap_or_default(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct InlineCode {
    text: String,
}

impl crate::node::Node for InlineCode {
    fn render(&self) -> hirola::dom::Dom {
        html! { <code>{&self.text}</code> }
    }
}
