use edita_core::Block;
use hirola::prelude::html;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlImageElement};

use edita_core::{Editor};

use crate::{node::EditorNode, EditorState};

pub struct ImageBlock;

impl Block for ImageBlock {
    type Input = web_sys::Node;
    type Node = EditorNode;
    type State = EditorState;
    fn accepts(&self, node: &web_sys::Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            element.tag_name() == "IMG"
        } else {
            false
        }
    }

    fn parse(
        &self,
        _editor: &Editor<EditorNode, EditorState, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        if let Some(img_element) = node.dyn_ref::<HtmlImageElement>() {
            EditorNode::Image(Image {
                src: img_element.src(),
                alt: img_element.alt(),
                // Add more attributes if needed
            })
        } else {
            panic!("not an image")
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct Image {
    pub src: String,
    pub alt: String,
}

impl crate::node::Node for Image {
    fn render(&self) -> hirola::dom::Dom {
        use hirola::prelude::DefaultAttrStr;
        use hirola::prelude::DefaultAttributeEffect;
        html! { <img src=&self.src alt=&self.alt/> }
    }
}
