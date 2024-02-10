use edita_core::{process_nodes, Block, Editor, Command};
use hirola::{dom::Dom, prelude::*};
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::{
    nodes::{EditorNode, Node},
    state::EditorState,
};

#[derive(Clone, Serialize, Default, Debug)]
pub struct Paragraph {
    elements: Vec<EditorNode>,
}

impl Node for Paragraph {
    fn render(&self) -> Dom {
        html! {
        <p data-ph="A paragraph goes here">
            {for element in &self.elements {
                element.render()
            }}
        </p>
    }
    }
}

pub struct ParagraphBlock;

impl Block for ParagraphBlock {
    type Node = EditorNode;
    type Input = web_sys::Node;
    type State = EditorState;
    fn accepts(&self, node: &web_sys::Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "P"
        } else {
            false
        }
    }

    fn parse(
        &self,
        editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        let mut paragraph = Paragraph { elements: vec![] };

        if let Some(element) = node.dyn_ref::<Element>() {
            let child_nodes = element.child_nodes();
            let mut nodes = vec![];
            for i in 0..child_nodes.length() {
                nodes.push(child_nodes.get(i).unwrap());
            }
            let nodes = process_nodes(editor, nodes);
            paragraph.elements = nodes;
        }

        EditorNode::Paragraph(paragraph)
    }
}

impl Command<EditorState> for ParagraphBlock {
    fn execute(&self, state: &mut EditorState) {
        state.add_node(Box::<Paragraph>::default())
    }
}
