use edita_core::{Block, Command, Editor};
use hirola::{dom::Dom, prelude::*};
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::{nodes::EditorNode, state::EditorState};

#[derive(Clone, Serialize, Debug)]
pub struct Header {
    text: String,
    level: u8,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            text: String::default(),
            level: 1,
        }
    }
}

impl crate::nodes::Node for Header {
    fn render(&self) -> Dom {
        html! { <h1 data-ph="Heading">{self.text.clone()}</h1> }
    }
}

pub struct HeaderBlock;

impl Block for HeaderBlock {
    type Node = EditorNode;
    type Input = web_sys::Node;
    type State = EditorState;
    fn accepts(&self, node: &web_sys::Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            matches!(
                element.tag_name().as_str(),
                "H1" | "H2" | "H3" | "H4" | "H5" | "H6"
            )
        } else {
            false
        }
    }
    fn parse(
        &self,
        _editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        if let Some(element) = node.dyn_ref::<Element>() {
            let text = element.text_content().unwrap_or_default();
            let level = match element.tag_name().as_str() {
                "H1" => 1,
                "H2" => 2,
                "H3" => 3,
                "H4" => 4,
                "H5" => 5,
                "H6" => 6,
                _ => 0, // Default level, or you could handle this case differently
            };

            EditorNode::Heading(Header { text, level })
        } else {
            unreachable!("No such header")
        }
    }
}

impl Command<EditorState> for HeaderBlock {
    fn execute(&self, state: &mut EditorState) {
        state.add_node(Header::default())
    }
}
