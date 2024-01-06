use edita_core::process_nodes;
use edita_core::Block;
use edita_core::Editor;
use hirola::dom::Html;
use hirola::dom::XEffect;
use hirola::prelude::*;
use hirola::signal::LocalBoxSignal;
use hirola::signal::SignalExt;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::node::EditorNode;
use crate::state::EditorState;

pub struct HtmlBlock;

#[derive(Clone, Serialize, Debug)]
pub struct HtmlNode(String);

impl crate::node::Node for HtmlNode {
    fn render(&self) -> hirola::dom::Dom {
        html! { <div data-node-atom="true" contenteditable="false" x:html=self.0.as_str()></div> }
    }
}

impl Block for HtmlBlock {
    type Node = EditorNode;
    type Input = web_sys::Node;
    type State = EditorState;
    fn accepts(&self, _node: &web_sys::Node) -> bool {
        true
    }

    fn parse(
        &self,
        _editor: &Editor<Self::Node, EditorState, web_sys::Node>,
        node: &web_sys::Node,
    ) -> EditorNode {
        let element: &HtmlElement = node.dyn_ref().unwrap();
        EditorNode::Html(HtmlNode(element.outer_html()))
    }
}

pub trait EditorExt {
    fn export(&self) -> Vec<EditorNode>;
    fn signal(&self) -> LocalBoxSignal<Vec<EditorNode>>;
}

impl EditorExt for Editor<EditorNode, EditorState, web_sys::Node> {
    fn export(&self) -> Vec<EditorNode> {
        let element = self.node().get().inner_element();
        let child_nodes = element.child_nodes();
        let mut nodes = vec![];
        for i in 0..child_nodes.length() {
            nodes.push(child_nodes.get(i).unwrap());
        }
        process_nodes(self, nodes)
    }

    fn signal(&self) -> LocalBoxSignal<Vec<EditorNode>> {
        self.notify
            .signal()
            .map(move |_| self.export())
            .boxed_local()
    }
}
