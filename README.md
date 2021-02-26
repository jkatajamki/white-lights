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
- Diesel CLI

## Setup!

- Copy `.env.example` file into `.env` and fill in the fields appropriate to your dev environment. Note that postgres wants a password. You can specify it to the DB_PASSWORD variable. Also make sure to add it to the DATABASE_URL connection string (in place of `<password>`).
- Set up dev environment with `make`. `make scratch` will set up things from scratch.
- Run database migrations using diesel: `diesel migration run`
- Run the server app at `server/white_lights` with `cargo run`
- Client: install dependencies at `client/src` with `npm i`, and run the client app with `npm run dev`
