use std::fmt;
use std::ops::{Deref, DerefMut};

/// The main editor structure that manages the state and blocks.
pub struct Editor<Node, State, Input> {
    /// The internal state of the editor.
    state: State,
    /// A collection of blocks that process input nodes.
    blocks: Vec<Box<dyn Block<Node = Node, State = State, Input = Input>>>,
    /// An optional fallback block used when no other block accepts input nodes.
    fallback_block: Option<Box<dyn Block<Node = Node, State = State, Input = Input>>>,
}
impl<Node, State, Input> Editor<Node, State, Input> {
    /// Creates a new `Editor` with the given initial state.
    pub fn new(state: State) -> Self {
        Editor {
            state,
            blocks: Vec::new(),
            fallback_block: None,
        }
    }
    /// Executes a command on the editor's state.
    pub fn command<C: Command<State>>(&mut self, cmd: C) {
        cmd.execute(&mut self.state)
    }
    /// Adds a block to the editor's list of blocks.
    pub fn add_block<B: Block<Node = Node, State = State, Input = Input> + 'static>(
        &mut self,
        block: B,
    ) {
        self.blocks.push(Box::new(block))
    }

    /// Sets the fallback block for the editor.
    pub fn set_fallback_block(
        &mut self,
        block: Box<dyn Block<Node = Node, State = State, Input = Input>>,
    ) {
        self.fallback_block = Some(block);
    }
}

impl<Node, State, Input> fmt::Debug for Editor<Node, State, Input>
where
    State: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Editor")
            .field("state", &self.state)
            .field("blocks", &self.blocks.len())
            .finish()
    }
}

impl<Node, State, Input> Deref for Editor<Node, State, Input> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<Node, State, Input> DerefMut for Editor<Node, State, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
/// Trait for defining blocks that process input nodes
#[allow(unused_variables)]
pub trait Block {
    type Input;
    type Node;
    type State;
    /// Called when a block is hooked into an editor.
    fn hook(&self, editor: &mut Editor<Self::Node, Self::State, Self::Input>) {}

    /// Determines whether the block accepts a specific input node.
    fn accepts(&self, input: &Self::Input) -> bool {
        false
    }
    /// Parses an input node and produces a node of a different type.
    fn parse(
        &self,
        editor: &Editor<Self::Node, Self::State, Self::Input>,
        input: &Self::Input,
    ) -> Self::Node;
}


/// Trait for defining commands that can modify the editor's state.
pub trait Command<State> {
    fn execute(&self, state: &mut State);
}


/// Processes input nodes using the editor's blocks and returns parsed nodes.
///
/// This function takes an `Editor`, which contains a collection of blocks, and a sequence
/// of input nodes. It processes each input node using the editor's blocks, and if no block
/// accepts an input node, it falls back to the editor's fallback block (if provided).
///
/// The parsed nodes are collected and returned as a `Vec`.
pub fn process_nodes<N, S, I, Iter: IntoIterator<Item = I>>(
    editor: &Editor<N, S, I>,
    children: Iter,
) -> Vec<N> {
    let blocks = &editor.blocks;
    let mut parsed_nodes = Vec::new();
    let children: Vec<I> = children.into_iter().collect();
    for i in 0..children.len() {
        if let Some(node) = children.get(i) {
            let mut accepted = false;

            for block in blocks {
                if block.accepts(node) {
                    parsed_nodes.push(block.parse(editor, node));
                    accepted = true;
                    break;
                }
            }
            // Fallback for nodes not accepted by any block
            if !accepted && editor.fallback_block.is_some() {
                parsed_nodes.push(editor.fallback_block.as_ref().unwrap().parse(editor, node));
            }
        }
    }

    parsed_nodes
}


#[cfg(test)]
mod tests {
    use super::*;

    // Define a simple state struct for testing
    #[derive(Debug, PartialEq, Eq)]
    struct TestState {
        value: i32,
    }

    // Define a simple block for testing
    struct TestBlock;

    impl Block for TestBlock {
        type Input = i32;
        type Node = i32;
        type State = TestState;

        fn accepts(&self, input: &i32) -> bool {
            *input % 2 == 0
        }

        fn parse(&self, _editor: &Editor<Self::Node, Self::State, Self::Input>, input: &i32) -> i32 {
            *input * 2
        }
    }

    // Define a simple command for testing
    struct TestCommand;

    impl Command<TestState> for TestCommand {
        fn execute(&self, state: &mut TestState) {
            state.value += 1;
        }
    }

    #[test]
    fn test_editor_new() {
        let editor: Editor<i32, TestState, i32> = Editor::new(TestState { value: 42 });
        assert_eq!(editor.state.value, 42);
    }

    #[test]
    fn test_editor_command() {
        let mut editor: Editor<i32, TestState, i32> = Editor::new(TestState { value: 0 });
        let command = TestCommand;
        editor.command(command);
        assert_eq!(editor.state.value, 1);
    }

    #[test]
    fn test_editor_add_block() {
        let mut editor: Editor<i32, TestState, i32> = Editor::new(TestState { value: 0 });
        let block = TestBlock;
        editor.add_block(block);
        let input = 4;
        let parsed_nodes = process_nodes(&editor, vec![input]);
        assert_eq!(parsed_nodes, vec![8]); // 4 * 2 = 8
    }

    #[test]
    fn test_editor_set_fallback_block() {
        let mut editor: Editor<i32, TestState, i32> = Editor::new(TestState { value: 0 });
        let fallback_block = Box::new(TestBlock);
        editor.set_fallback_block(fallback_block);
        let input = 3; // Fallback block should handle odd input
        let parsed_nodes = process_nodes(&editor, vec![input]);
        assert_eq!(parsed_nodes, vec![6]); // 3 * 2 = 6 (handled by the fallback block)
    }

    #[test]
    fn test_process_nodes_no_blocks() {
        let editor: Editor<i32, TestState, i32> = Editor::new(TestState { value: 0 });
        let input = 5;
        let parsed_nodes: Vec<_> = process_nodes(&editor, vec![input]);
        assert_eq!(parsed_nodes, vec![]); // No blocks to handle the input
    }
}
