pub mod block_quote;
mod bullet_list;
pub mod list_item;
mod ordered_list;
mod task_item;
mod task_list;

use hirola::{dom::Dom, prelude::*};
use serde::Serialize;

use crate::{
    nodes::block_quote::BlockQuote,
    editor::HtmlNode,
    heading::Heading,
    image::Image,
    paragraph::Paragraph,
    text::{Bold, InlineCode, Italic, TextNode},
};

use self::{
    bullet_list::BulletList, list_item::ListItem, ordered_list::OrderedList, task_item::TaskItem,
    task_list::TaskList,
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
    BlockQuote(BlockQuote),
    ListItem(ListItem),
    BulletList(BulletList),
    OrderedList(OrderedList),
    TaskItem(TaskItem),
    TaskList(TaskList),
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
            EditorNode::BlockQuote(quote) => quote.render(),
            EditorNode::ListItem(item) => item.render(),
            EditorNode::BulletList(bullets) => bullets.render(),
            EditorNode::OrderedList(list) => list.render(),
            EditorNode::TaskItem(item) => item.render(),
            EditorNode::TaskList(list) => list.render(),
        }
    }
}

impl Render<Dom> for EditorNode {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        parent.append_render(self.render());
        Ok(())
    }
}
