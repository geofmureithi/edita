use edita_core::Command;
use hirola::{
    dom::node_ref::NodeRef,
    prelude::{Callback, GenericNode},
    signal::Mutable,
};

use crate::nodes::Node;

// Editor state, representing the current state of the editor
#[derive(Clone)]
pub struct EditorState {
    node: NodeRef,
    // current_selection: Mutable<Option<Selection>>,
    // toolbar: HashMap<String, Box<dyn Command<Self>>>,
    // shortcuts: HashMap<String, String>,
    pub(crate) notify: Mutable<()>,
}

impl EditorState {
    pub fn new(node: NodeRef) -> Self {
        EditorState {
            node,
            // current_selection: Mutable::new(None),
            // toolbar: HashMap::new(),
            // shortcuts: HashMap::new(),
            notify: Mutable::new(()),
        }
    }
    pub fn add_node<N: Node>(&self, node: N) {
        self.node.get().append_render(node.render());
    }

    pub fn node(&self) -> &NodeRef {
        &self.node
    }

    pub fn execute<C: Command<Self>>(&self, cmd: C) {
        cmd.execute(&mut self.clone())
    }

    pub fn notify(&self) {
        self.notify.replace(())
    }

}

impl Callback<web_sys::Event> for EditorState {}
