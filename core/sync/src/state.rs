//! State management for synchronized documents

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a shared document state in the mesh
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SharedDocument {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub content: Vec<u8>,
    pub version: u64,
}

impl SharedDocument {
    pub fn new(owner: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            owner,
            content: Vec::new(),
            version: 0,
        }
    }

    /// Update the content and increment version
    pub fn update(&mut self, new_content: Vec<u8>) {
        self.content = new_content;
        self.version += 1;
    }

    /// Get the size of the document
    pub fn size(&self) -> usize {
        self.content.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = SharedDocument::new("owner".to_string(), "test".to_string());
        assert_eq!(doc.owner, "owner");
        assert_eq!(doc.name, "test");
        assert_eq!(doc.version, 0);
    }

    #[test]
    fn test_document_update() {
        let mut doc = SharedDocument::new("owner".to_string(), "test".to_string());
        doc.update(vec![1, 2, 3]);

        assert_eq!(doc.version, 1);
        assert_eq!(doc.size(), 3);
    }
}
