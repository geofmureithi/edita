use edita_core::{Block, Editor};
use hirola::prelude::*;
use js_sys::Object;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::{nodes::EditorNode, EditorState};

use super::task_item::{TaskItem, TaskItemBlock};

pub struct TaskListBlock;

impl Block for TaskListBlock {
    type Node = EditorNode;
    type State = EditorState;
    type Input = web_sys::Node;
    fn accepts(&self, node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "UL"
                && element
                    .get_attribute("data-type")
                    .is_some_and(|v| &v == "taskList")
        } else {
            false
        }
    }

    fn parse(
        &self,
        editor: &Editor<Self::Node, Self::State, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        EditorNode::TaskList(TaskList {
            list: Object::entries(&node.child_nodes())
                .into_iter()
                .map(|item| TaskItemBlock::parse(&TaskItemBlock, editor, item.dyn_ref().unwrap()))
                .map(|node| match node {
                    EditorNode::TaskItem(item) => Some(item),
                    _ => None,
                })
                .filter(|item| item.is_some())
                .map(|c| c.unwrap())
                .collect(),
        })
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct TaskList {
    list: Vec<TaskItem>,
}

impl crate::nodes::Node for TaskList {
    fn render(&self) -> hirola::dom::Dom {
        html! {
            <ul data-type="taskList">
                {for item in &self.list {
                    html! { <>{item.render()}</> }
                }}
            </ul>
        }
    }
}
