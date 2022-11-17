# rust-fullstack-demo
A fullstack Rust application demo

## Running on Docker
First install Docker.
You can find instructions [here](https://docs.docker.com/get-docker/).

Next, install `docker-compose`.
This will allow us to build and run several docker containers simultaneously.
Instructions can be found [here](https://docs.docker.com/compose/install/).

To run the docker container, you'll need file called `docker.env` to hold your environment variables.
It's contents should look like this:

```
PGDATA=./database/data/

FRONTEND_HOST=localhost
FRONTEND_PORT=8000
FRONTEND_PROTOCOL=http

DATABASE_PROTOCOL=postgres
DATABASE_PORT=5432
POSTGRES_USER=ratebeer_app
POSTGRES_PASSWORD=passw0rd
POSTGRES_DB=ratebeer_clone
POSTGRES_HOST=database

BACKEND_HOST=backend
BACKEND_PORT=8080
BACKEND_PROTOCOL=http
```

Once everything is up and running, visit [http://localhost:8000] to view the app.

## Running Outside Docker

### Database Installation
This app requires a Postgres 14 database.
Follow the steps in the article below which matches your operating system.

Downloads for the various operating systems can be found [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

#### Windows
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql/)

After installation, open your terminal and try running "psql".
If you find that the command isn't find, then you probably need to set some environment variables.

If you're running windows, you can follow the instructions in step 3 of this [article](https://aeadedoyin.medium.com/getting-started-with-postgresql-on-windows-201906131300-ee75f066df78).

#### Mac
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-macos/)

#### Linux
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/)

### Database Setup
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

### ENV FILE
You'll need a file to hold necessary environment variables.
If you're running docker, it should be named `docker.env` and contain the following:

```
PGDATA=./database/data/

FRONTEND_HOST=localhost
FRONTEND_PORT=8000
FRONTEND_PROTOCOL=http

DATABASE_PROTOCOL=postgres
DATABASE_PORT=5432
POSTGRES_USER=ratebeer_app
POSTGRES_PASSWORD=passw0rd
POSTGRES_DB=ratebeer_clone
POSTGRES_HOST=database

BACKEND_HOST=localhost
BACKEND_HOST_INTERNAL=0.0.0.0
BACKEND_PORT=8080
BACKEND_PROTOCOL=http
```

If you're running it outside of docker, the file should be called `.env` and should contain the following:

```
FRONTEND_HOST=localhost
FRONTEND_PORT=8000
FRONTEND_PROTOCOL=http

DATABASE_PROTOCOL=postgres
DATABASE_PORT=5432
POSTGRES_USER=ratebeer_app
POSTGRES_PASSWORD=password
POSTGRES_DB=ratebeer_clone
POSTGRES_HOST=localhost

BACKEND_HOST=localhost
BACKEND_PORT=8080
BACKEND_PROTOCOL=http
```


### Running Database Migrations

Next we need to run the migrations to populate the database with tables and sample data.

From the root directory run the following:

```bash
cargo run --manifest-path ./database/migration/Cargo.toml
```

### Running the App
The app consists of a frontend and a backend component.
To run the backend, navigate to the backend folder and run `cargo run`

Any pending migrations will be executed on app startup.

The frontend is built using Webpack, so you'll need to install [Node](https://nodejs.org/en/download/) to run it.

Before running the app, you'll have to install the NPM packages.
Navigate to the frontend folder and run `npm install`.
You'll only have to do this once.

From the frontend folder run `npm run dev` to start the dev server.

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