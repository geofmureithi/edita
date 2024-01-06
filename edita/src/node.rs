use hirola::dom::Dom;
use serde::Serialize;

use crate::{
    editor::HtmlNode,
    heading::Heading,
    image::Image,
    paragraph::Paragraph,
    text::{Bold, InlineCode, Italic, TextNode},
};

pub trait Node {
    fn render(&self) -> Dom;
}

#[derive(Serialize, Clone, Debug)]
pub enum EditorNode {
    Html(HtmlNode),
    Paragraph(Paragraph),
    Text(TextNode),
    Bold(Bold),
    Heading(Heading),
    Italic(Italic),
    InlineCode(InlineCode),
    Image(Image),
}

impl Node for EditorNode {
    fn render(&self) -> Dom {
        match self {
            EditorNode::Html(html_node) => html_node.render(),
            EditorNode::Paragraph(paragraph) => paragraph.render(),
            EditorNode::Text(text_node) => text_node.render(),
            EditorNode::Bold(bold) => bold.render(),
            EditorNode::Heading(heading) => heading.render(),
            EditorNode::Italic(italic) => italic.render(),
            EditorNode::InlineCode(inline_code) => inline_code.render(),
            EditorNode::Image(image) => image.render(),
        }
    }
}
