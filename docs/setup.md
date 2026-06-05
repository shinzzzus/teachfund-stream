# Setup

1. Copy `.env.example` to `.env` where needed.
2. Install dependencies with `npm install`.
3. Generate Prisma client with `npm run db:generate`.
4. Create the local SQLite database with `npm --workspace backend run db:migrate`.
5. Seed demo records with `npm run db:seed`.
6. Start both apps with `npm run dev`.

The frontend runs on Vite. The backend listens on `BACKEND_PORT`, default `8787`.

Project: Teacher Crowdfunded Professional Development
