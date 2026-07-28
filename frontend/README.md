# DocMind Frontend

The user interface for **DocMind**, an AI-powered document intelligence platform. It provides a clean, responsive web interface for uploading documents, performing semantic searches, and interacting with the RAG (Retrieval-Augmented Generation) chat assistant.

---

## Tech Stack

* **Language:** TypeScript
* **Framework:** React
* **Styling / UI:** Modern CSS / Component library (customizable)
* **API Communication:** Fetch API / Axios for REST integration with the Go backend

---

## Project Structure

```text
frontend/
├── public/              # Static assets and icons
├── src/
│   ├── components/      # Reusable UI components (buttons, modals, cards)
│   ├── features/        # Feature-based modules (upload, search, chat)
│   ├── services/        # API client configurations and endpoints
│   ├── types/           # TypeScript interfaces and shared types
│   ├── App.tsx          # Root component and layout routing
│   └── main.tsx         # Application entry point
├── package.json
├── tsconfig.json
└── vite.config.ts       # Build and development configuration
