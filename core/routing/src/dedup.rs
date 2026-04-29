//! Packet deduplication cache

use std::collections::HashSet;
use uuid::Uuid;

/// DedupCache tracks seen packet IDs to prevent forwarding duplicates
/// This is a simple set-based approach; production could use LRU with TTL
pub struct DedupCache {
    seen: HashSet<Uuid>,
}

impl DedupCache {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    /// Check if packet ID has been seen
    pub fn contains(&self, id: &Uuid) -> bool {
        self.seen.contains(id)
    }

    /// Mark a packet as seen
    pub fn insert(&mut self, id: Uuid) {
        self.seen.insert(id);
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.seen.len()
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.seen.clear();
    }
}

impl Default for DedupCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_cache() {
        let mut cache = DedupCache::new();
        let id = Uuid::new_v4();

        assert!(!cache.contains(&id));
        cache.insert(id);
        assert!(cache.contains(&id));
    }

    #[test]
    fn test_cache_size() {
        let mut cache = DedupCache::new();
        assert_eq!(cache.size(), 0);

        cache.insert(Uuid::new_v4());
        assert_eq!(cache.size(), 1);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }
}
