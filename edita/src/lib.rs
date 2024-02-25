mod commands;
mod editor;
mod nodes;

mod state;

pub use edita_core as core;

pub use crate::commands::bold::MakeBold;
pub use crate::editor::EditorExt;
pub use crate::nodes::*;
pub use crate::state::EditorState;

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
