mod commands;
mod editor;
mod heading;
mod image;
mod nodes;
mod paragraph;
mod state;
mod text;

pub use edita_core as core;

pub use crate::commands::bold::MakeBold;
pub use crate::editor::EditorExt;
pub use crate::heading::HeaderBlock;
pub use crate::image::Image;
pub use crate::image::ImageBlock;
pub use crate::nodes::Node;
pub use crate::paragraph::Paragraph;
pub use crate::paragraph::ParagraphBlock;
pub use crate::state::EditorState;
pub use crate::text::{BoldBlock, InlineCodeBlock, ItalicBlock, TextNodeBlock};

// # Blocks
//
// BulletList x
// CodeBlock
// DetailsContent
// DetailsSummary
// Emoji
// Hardbreak
// HR
// Image x
// Heading x
// ListItem x
// OrderedList x
// Paragraph x
// Table
// TableRow
// TableCell
// TableHeader
// TaskList x
// TaskItem x
// Text x
