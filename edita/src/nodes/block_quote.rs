use edita_core::{Block, Editor};
use hirola::prelude::*;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{nodes::EditorNode, EditorState};

pub struct BlockQuoteBlock;

impl Block for BlockQuoteBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "BLOCKQUOTE"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::BlockQuote(BlockQuote {
            text: node.text_content().unwrap_or_default(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct BlockQuote {
    text: String,
}

impl crate::nodes::Node for BlockQuote {
    fn render(&self) -> hirola::dom::Dom {
        html! { <blockquote>{&self.text}</blockquote> }
    }
}
