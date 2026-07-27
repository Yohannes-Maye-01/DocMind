/**
 * Axios API client pre-configured for the DocMind Go backend.
 * All service calls should go through this client.
 */
import axios from 'axios'

const BASE_URL = import.meta.env.VITE_API_BASE_URL ?? '/api'

export const apiClient = axios.create({
  baseURL: BASE_URL,
  headers: { 'Content-Type': 'application/json' },
  timeout: 30_000,
})

// ── Document endpoints ────────────────────────────────────────────────────────

export const listDocuments = () =>
  apiClient.get('/documents').then((r) => r.data)

export const uploadDocument = (file: File) => {
  const form = new FormData()
  form.append('file', file)
  return apiClient.post('/documents/upload', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
  }).then((r) => r.data)
}

export const deleteDocument = (id: string) =>
  apiClient.delete(`/documents/${id}`).then((r) => r.data)

// ── Search ───────────────────────────────────────────────────────────────────

export const searchDocuments = (query: string, page = 1) =>
  apiClient.get('/search', { params: { q: query, page } }).then((r) => r.data)

// ── RAG Q&A ──────────────────────────────────────────────────────────────────

export const ragQuery = (question: string) =>
  apiClient.post('/query', { question }).then((r) => r.data)
