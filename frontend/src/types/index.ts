/** Shared TypeScript types for DocMind frontend. */

export interface Document {
  id: string
  title: string
  file_type: 'pdf' | 'txt' | 'md'
  file_size: number   // bytes
  status: 'pending' | 'indexed' | 'failed'
  created_at: string  // ISO 8601
}

export interface SearchResult {
  document_id: string
  chunk_index: number
  snippet: string
  score: number
}

export interface SearchResponse {
  results: SearchResult[]
  total: number
  page: number
}

export interface RagResponse {
  answer: string
  sources: string[]
}
