mod commands;
mod editor;
mod heading;
mod image;
mod node;
mod paragraph;
mod state;
mod text;

use edita_core::Command;
use edita_core::Editor;
use hirola::dom::node_ref::NodeRef;
use hirola::dom::*;
use hirola::prelude::*;
use hirola::signal::SignalExt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::commands::bold::MakeBold;
use crate::editor::EditorExt;
use crate::heading::HeaderBlock;
use crate::image::Image;
use crate::image::ImageBlock;
use crate::node::Node;
use crate::paragraph::Paragraph;
use crate::paragraph::ParagraphBlock;
use crate::state::EditorState;
use crate::text::{BoldBlock, InlineCodeBlock, ItalicBlock, TextNodeBlock};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    hirola::dom::mount(app()).unwrap();
}

fn app() -> Dom {
    let node = NodeRef::new();
    let state = EditorState::new(node.clone());

    let notifier = state.clone();

    let mut editor = edita_core::Editor::new(state.clone());
    editor.add_block(HeaderBlock);
    editor.add_block(ParagraphBlock);
    editor.add_block(TextNodeBlock);
    editor.add_block(InlineCodeBlock);
    editor.add_block(BoldBlock);
    editor.add_block(ItalicBlock);
    editor.add_block(ImageBlock);

    let parent = node.clone();

    let fut = async move {
        let callback: Closure<dyn FnMut(_)> =
            Closure::new(move |_: js_sys::Array| notifier.notify());
        let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).unwrap();
        let mut options = web_sys::MutationObserverInit::new();
        options.attributes(true);
        options.subtree(true);
        options.character_data(true);
        observer
            .observe_with_options(&parent.get().inner_element(), &options)
            .unwrap();
        callback.forget();
    };
    let sig = async move {
        editor
            .signal()
            .map(|nodes| {
                log::info!("{}", serde_json::to_string(&nodes).unwrap());
            })
            .to_future()
            .await;
    };

    html! {
        <div id="holder">
            <div id="menubar">
                <button on:click=state.callback_with(|state, _| HeaderBlock.execute(&mut state.clone()))>
                    "H"
                </button>
                <button on:click=state.callback_with(|state, _| ParagraphBlock.execute(&mut state.clone()))>
                    "P"
                </button>
                // <button on:click=state.callback_with(|state, _| RemoveBold.execute(&mut state.clone()))>
                //     "RB"
                // </button>
                <button on:click=state.callback_with(|state, _| state.add_node(Box::new(Image{
                    src: "https://placehold.co/600x400".to_owned(),
                    alt: "Placeholder".to_owned()
                })))>
                "I"
            </button>
            <div/>
            <button on:click=state.callback_with(|state, _| MakeBold.execute(&mut state.clone()))>
                    "B"
                </button>
            </div>
            <div bind:ref=node use:future=fut use:future=sig contenteditable="true" class="prose">
                <h1>"A simple Editor"</h1>
                {Paragraph::default().render()}
            </div>

        </div>
    }
}
