use crate::{Stream, codec::Conversation};

#[derive(Debug, Clone)]
pub struct McpServer<T: Stream> {
    conversation: Conversation<T>,
}
