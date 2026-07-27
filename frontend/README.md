# DocMind – Frontend (TypeScript / React)

React SPA built with Vite. Communicates exclusively with the Go backend API.

## Stack
- **Language:** TypeScript 5 (strict mode)
- **Framework:** React 18
- **Bundler:** Vite 5
- **HTTP client:** Axios

## Structure
```
frontend/
├── src/
│   ├── api/
│   │   └── client.ts       # Axios client + all API calls
│   ├── components/         # Reusable UI components (Phase 3)
│   ├── hooks/              # Custom React hooks (Phase 3)
│   ├── pages/              # Route-level page components (Phase 3)
│   ├── types/
│   │   └── index.ts        # Shared TypeScript interfaces
│   ├── App.tsx             # Root component
│   └── main.tsx            # Entry point
├── index.html
├── vite.config.ts          # Dev proxy → Go :8080
├── tsconfig.json
├── package.json
├── nginx.conf              # Production nginx config
└── Dockerfile
```

## Running locally
```bash
npm install
npm run dev
# → http://localhost:3000
```

## Code Style
- ESLint + TypeScript strict mode (`noImplicitAny`, etc.)
- No `any` types allowed
- 60% component test coverage target (Phase 6)
