# rust-fullstack-demo
A fullstack Rust application demo

## Database Installation
This app requires a Postgres 14 database.
Follow the steps in the article below which matches your operating system.
Be sure to select Postgres 14 rather than whatever version is mentioned in the article.

Downloads for the various operating systems can be found [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

### Windows
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql/)

After installation, open your terminal and try running "psql".
If you find that the command isn't find, then you probably need to set some environment variables.

If you're running windows, you can follow the instructions in step 3 of this [article](https://aeadedoyin.medium.com/getting-started-with-postgresql-on-windows-201906131300-ee75f066df78).

### Mac
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-macos/)

### Linux
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/)

## Database Setup
Once you have a Postgres server up and running, create a database and make sure your database user as read and write privileges.

Example:

```bash
sudo -u postgres psql
create database rustfullstack;
create user rustuser with encrypted password 'password';
grant all privileges on database rustfullstack to rustuser;
exit
```

NOTE: If you get an error message saying "connection refused", your postgres server may have installed on port 5433 rather than the default 5432. To resolve this, try using the -p flag as follows:

```bash
sudo -u postgres psql -p 5433
```

Next create a .env file in the root of this project and put the connection url inside.
It should look like this:

```
DATABASE_URL=postgres://{postgres_user}:{postgres_password}@localhost:5432/{database_name}
```

If you used the exact settings from above then it should look like this:

```
DATABASE_URL=postgres://rustuser:password@localhost:5432/rustfullstack
```

NOTE: If you needed to use port 5433 as above, then change the port accordingly here:

```
DATABASE_URL=postgres://rustuser:password@localhost:5433/rustfullstack
```

Next we need to run the migrations to populate the database with tables and sample data.

From the root directory run the following:

```bash
cargo run --manifest-path ./database/migration/Cargo.toml
```

If the above command fails, you may need to install the SeaOrm CLI. Install it using the following command:

```bash
cargo install sea-orm-cli
```

## Running the App
The app consists of a frontend and a backend component.
To run the backend, navigate to the backend folder and run `cargo run`

Any pending migrations will be executed on app startup.

The frontend is built using Webpack, so you'll need to install [Node](https://nodejs.org/en/download/) to run it.

Before running the app, you'll have to install the NPM packages.
You'll only have to do this once.

```bash
npm install
```

Once you've installed Node, navigate to the fontend folder and run `npm run dev` to start the dev server.

Note: On Windows, you might see the following error:

```powershell
<e> [webpack-dev-middleware] Error: spawn npm ENOENT
<e>     at ChildProcess._handle.onexit (node:internal/child_process:285:19)
<e>     at onErrorNT (node:internal/child_process:483:16)
<e>     at process.processTicksAndRejections (node:internal/process/task_queues:82:21) {
<e>   errno: -4058,
<e>   code: 'ENOENT',
<e>   syscall: 'spawn npm',
<e>   path: 'npm',
<e>   spawnargs: [ 'install', '-g', 'wasm-pack' ]
<e> }
```

If you get this error, run the following command and then try again:

```
npm install -g wasm-pack
```