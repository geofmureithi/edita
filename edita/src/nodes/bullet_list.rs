use edita_core::{Block, Editor};
use hirola::prelude::*;
use js_sys::Object;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{nodes::EditorNode, EditorState};

use super::list_item::{ListItem, ListItemBlock};

pub struct BulletListBlock;

impl Block for BulletListBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "UL"
        } else {
            false
        }
    }

    fn parse(
        &self,
        editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::BulletList(BulletList {
            list: Object::entries(&node.child_nodes())
                .into_iter()
                .map(|item| ListItemBlock::parse(&ListItemBlock, editor, item.dyn_ref().unwrap()))
                .map(|node| match node {
                    EditorNode::ListItem(item) => Some(item),
                    _ => None,
                })
                .filter(|item| item.is_some())
                .map(|c| c.unwrap())
                .collect(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct BulletList {
    list: Vec<ListItem>,
}

impl crate::nodes::Node for BulletList {
    fn render(&self) -> hirola::dom::Dom {
        html! {
        <ul>
            {for item in &self.list {
                html! { <>{item.render()}</> }
            }}
        </ul>
    }
    }
}
