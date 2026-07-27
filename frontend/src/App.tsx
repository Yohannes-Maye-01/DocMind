import React from 'react'

/**
 * DocMind root application component.
 *
 * Pages (to be implemented in Phase 3):
 *  - /            → Dashboard / document list
 *  - /upload      → Document upload
 *  - /search      → Search results
 *  - /chat        → RAG Q&A interface
 */
export default function App(): React.ReactElement {
  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem' }}>
      <h1>🧠 DocMind</h1>
      <p>AI-powered document intelligence platform.</p>
      <p style={{ color: '#666' }}>
        Frontend scaffold ready. Implement pages in <code>src/pages/</code>.
      </p>
    </div>
  )
}
