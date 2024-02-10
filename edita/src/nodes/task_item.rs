use edita_core::{Block, Editor};
use hirola::prelude::*;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{nodes::EditorNode, EditorState};

pub struct TaskItemBlock;

impl Block for TaskItemBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "LI"
                && element
                    .get_attribute("data-type")
                    .is_some_and(|v| &v == "taskItem")
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::TaskItem(TaskItem {
            text: node.text_content().unwrap_or_default(),
            checked: true,
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct TaskItem {
    text: String,
    checked: bool,
}

impl crate::nodes::Node for TaskItem {
    fn render(&self) -> hirola::dom::Dom {
        html! {
            <li data-type="dataItem">
                <input type="checkbox" checked=&self.checked.to_string()/>
                {&self.text}
            </li>
        }
    }
}
