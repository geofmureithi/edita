use edita_core::{Block, Editor};
use hirola::prelude::*;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{nodes::EditorNode, EditorState};

pub struct ListItemBlock;

impl Block for ListItemBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "LI"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::ListItem(ListItem {
            text: node.text_content().unwrap_or_default(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct ListItem {
    text: String,
}

impl crate::nodes::Node for ListItem {
    fn render(&self) -> hirola::dom::Dom {
        html! { <li>{&self.text}</li> }
    }
}
