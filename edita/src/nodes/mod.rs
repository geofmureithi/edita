mod block_quote;
mod bullet_list;
mod heading;
mod image;
mod list_item;
mod ordered_list;
mod paragraph;
mod task_item;
mod task_list;
mod text;
use hirola::{dom::Dom, prelude::*};
use serde::Serialize;

use crate::{editor::HtmlNode, nodes::block_quote::BlockQuote};

pub use self::{
    bullet_list::BulletList,
    heading::Header,
    image::Image,
    list_item::ListItem,
    ordered_list::OrderedList,
    paragraph::Paragraph,
    task_item::TaskItem,
    task_list::TaskList,
    text::{Bold, InlineCode, Italic, TextNode},
};

pub use self::{
    bullet_list::BulletListBlock,
    heading::HeaderBlock,
    image::ImageBlock,
    list_item::ListItemBlock,
    ordered_list::OrderedListBlock,
    paragraph::ParagraphBlock,
    task_item::TaskItemBlock,
    task_list::TaskListBlock,
    text::{BoldBlock, InlineCodeBlock, ItalicBlock, TextNodeBlock},
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
    Heading(Header),
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
