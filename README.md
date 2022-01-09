# michess
wip - an attempt to build an async online chess server using rust and web assembly.
Web server is [Warp](https://github.com/seanmonstar/warp). Runtime is [Tokio](https://github.com/tokio-rs/tokio). DB is PostgreSQL and caching of game state is done with Redis. Server uses GraphQL with the [async-graphql](https://github.com/async-graphql/async-graphql) implementation.
UI uses Rust WASM on the [Seed](https://github.com/seed-rs/seed) framework with Tailwind CSS and is built using [Trunk](https://github.com/thedodd/trunk).

#### setup & requirements
- install [Rust v1.53 or above](https://www.rust-lang.org/tools/install)
- the ui is built using a Rust WASM build tool called Trunk. Follow these [instructions](https://trunkrs.dev/#install) to get Trunk installed
- database is PostgreSQL. We use [SQLx](https://github.com/launchbadge/sqlx) to interact with the db. [Installing the CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) will be useful to run the migrations

### todo
##### accounts / authentication
- [ ] account creation
- [ ] login ( password salting, hashing )
- [ ] guest account
- [ ] profile page with win / losss / draw info
##### matchmaking
- [ ] a player can search for another player to challenge
- [ ] prevent challenges to offline players
- [ ] ability to face a random online player
- [ ] can open multiple games with multiple players
- [ ] close connection if no activity within time limit
##### chess
- [ ] draw board and pieces
- [ ] allow piece movement
- [ ] implement piece capturing
- [ ] implement castling
- [ ] implement en passant
- [ ] disallow illegal moves
- [ ] implement elo system
- [ ] implement different time controls
- [ ] implement pre-moving
- [ ] implement move history (see what the previous moves were)
