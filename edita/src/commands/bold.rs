use edita_core::Command;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Element};

use crate::state::EditorState;

pub struct MakeBold;

impl Command<EditorState> for MakeBold {
    fn execute(&self, _state: &mut EditorState) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        if let Some(selection) = window.get_selection().unwrap() {
            if selection.range_count() == 0 {
                return;
            }

            let range = selection.get_range_at(0).unwrap();

            let bold = document.create_element("b").unwrap();
            let bold = bold.dyn_into::<HtmlElement>().unwrap();

            let fragment = range.extract_contents().unwrap();
            bold.append_child(&fragment).unwrap();
            range.insert_node(&bold.into()).unwrap();

            selection.remove_all_ranges().unwrap();
            selection.add_range(&range).unwrap();
        }
    }
}

pub struct RemoveBold;

impl Command<EditorState> for RemoveBold {
    fn execute(&self, _state: &mut EditorState) {
        // let window = window().unwrap();
        // let document = window.document().unwrap();
        // Helper function to find the bold ancestor of a node
        fn find_bold_ancestor(node: &web_sys::Node) -> Option<HtmlElement> {
            let mut current = Some(node.clone());
            while let Some(ref current_node) = current {
                if let Some(element) = current_node.dyn_ref::<Element>() {
                    if element.tag_name() == "B" || element.tag_name() == "STRONG" {
                        return Some(element.clone().dyn_into::<HtmlElement>().unwrap());
                    }
                }
                current = current_node.parent_node();
            }
            None
        }

        // Helper function to unwrap an element from its children
        fn unwrap_element(element: &HtmlElement) {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let fragment = document.create_document_fragment();
            while let Some(child) = element.first_child() {
                fragment.append_child(&child).unwrap();
            }
            element.replace_with_with_node_1(&fragment).unwrap();
        }
        if let Some(selection) = window().unwrap().get_selection().unwrap() {
            if selection.range_count() == 0 {
                return;
            }

            let range = selection.get_range_at(0).unwrap();

            // Walk up the DOM from the start container to find a bold element
            if let Some(start_bold_element) = find_bold_ancestor(&range.start_container().unwrap())
            {
                unwrap_element(&start_bold_element);
            }

            // Do the same for the end container if it's different
            if range.start_container() != range.end_container() {
                if let Some(end_bold_element) = find_bold_ancestor(&range.end_container().unwrap())
                {
                    unwrap_element(&end_bold_element);
                }
            }

            // Update the range to reflect the change
            selection.remove_all_ranges().unwrap();
            selection.add_range(&range).unwrap();
        }
    }
}
