# white-lights
Powerlifting app

- Client app is built on Next.js with TypeScript
- Server app is built on Actix Web framework with Rust
- The server app is connected to a PostgreSQL database

## Requirements:

- Git
- Docker
- NodeJS and npm
- Rust and cargo

## Setup!

1. Set up and run Postgres DB in docker with `./set-up-db.sh`
2. Run the server app at `server/white_lights` with `cargo run`
3. Run the client app at `client/src` with `npm run dev`
