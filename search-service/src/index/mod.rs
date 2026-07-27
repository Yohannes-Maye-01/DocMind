use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, OnceLock};

/// A single indexed chunk.
#[derive(Clone, Debug)]
pub struct Chunk {
    pub document_id: String,
    pub chunk_index: usize,
    pub content: String,
}

/// Global in-memory index backed by a BTreeMap<token → Vec<chunk_id>>.
/// Designed to stay within the 300MB RAM budget (see NFR-01.1).
pub struct InMemoryIndex {
    /// token → list of (document_id, chunk_index)
    inverted: BTreeMap<String, Vec<(String, usize)>>,
    /// chunk storage
    chunks: HashMap<String, Vec<Chunk>>, // keyed by document_id
}

static GLOBAL: OnceLock<Arc<Mutex<InMemoryIndex>>> = OnceLock::new();

impl InMemoryIndex {
    fn new() -> Self {
        Self {
            inverted: BTreeMap::new(),
            chunks: HashMap::new(),
        }
    }

    /// Return a handle to the process-wide singleton index.
    pub fn global() -> std::sync::MutexGuard<'static, InMemoryIndex> {
        GLOBAL
            .get_or_init(|| Arc::new(Mutex::new(InMemoryIndex::new())))
            .lock()
            .expect("InMemoryIndex mutex poisoned")
    }

    /// Index a set of chunks for a document.
    /// Returns the number of chunks indexed.
    pub fn insert(&mut self, document_id: &str, chunks: &[String]) -> usize {
        for (i, content) in chunks.iter().enumerate() {
            let chunk = Chunk {
                document_id: document_id.to_string(),
                chunk_index: i,
                content: content.clone(),
            };

            // Tokenize and populate inverted index
            for token in tokenize(content) {
                self.inverted
                    .entry(token)
                    .or_default()
                    .push((document_id.to_string(), i));
            }

            self.chunks
                .entry(document_id.to_string())
                .or_default()
                .push(chunk);
        }
        chunks.len()
    }

    /// Remove all chunks for a document.
    pub fn remove(&mut self, document_id: &str) {
        self.chunks.remove(document_id);
        // Purge from inverted index
        for postings in self.inverted.values_mut() {
            postings.retain(|(doc_id, _)| doc_id != document_id);
        }
        self.inverted.retain(|_, v| !v.is_empty());
    }

    /// Look up postings for a token.
    pub fn lookup(&self, token: &str) -> &[(String, usize)] {
        self.inverted.get(token).map_or(&[], Vec::as_slice)
    }

    /// Retrieve a chunk by document_id and chunk_index.
    pub fn get_chunk(&self, document_id: &str, chunk_index: usize) -> Option<&Chunk> {
        self.chunks.get(document_id)?.get(chunk_index)
    }

    pub fn total_chunks(&self) -> usize {
        self.chunks.values().map(|v| v.len()).sum()
    }
}

/// Simple whitespace + punctuation tokenizer producing lowercase tokens.
pub fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() > 1)
        .map(|s| s.to_lowercase())
}
