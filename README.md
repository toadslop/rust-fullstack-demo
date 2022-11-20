# rust-fullstack-demo
A fullstack Rust application demo

## Items Demonstrated In this App

### General
* Compile time environment variables
* Runtime environment variables
* Sharing entities between frontend and backend
* Dockerizing Rust frontends and backends
* Running tasks with [Cargo Make](https://github.com/sagiegurari/cargo-make)

### Database
* Object Relational Mapping
* Seeding with random data
* Seeding with CSV data
* Entity Definitions

### Backend
* Routing
* Middleware
* CORS configuration
* Application State
* Database connections

### Frontend
* Asynchronous data fetching
* Functional components
* State management with hooks
* Posting data
* Building with webpack


## Running on Docker
First install Docker.
You can find instructions [here](https://docs.docker.com/get-docker/).

Next, install `docker-compose`.
This will allow us to build and run several docker containers simultaneously.
Instructions can be found [here](https://docs.docker.com/compose/install/).

Once everything is up and running, visit [http://localhost:8000] to view the app.

Note that docker-compose will start the app in production mode.

## Running Outside Docker

### Database Installation
This app requires a Postgres 14 database.

Downloads for the various operating systems can be found [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

#### Windows
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql/)

After installation, open your terminal and try running "psql".
If you find that the command isn't find, then you probably need to set some environment variables.
Follow the instructions in step 3 of this [article](https://aeadedoyin.medium.com/getting-started-with-postgresql-on-windows-201906131300-ee75f066df78) to do so.
Keep in mind that you will need to set the version to 14.

#### Mac
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-macos/)

#### Linux
Refer to this [article](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/)

### Database Setup
Once you have a Postgres server up and running, create a database and make sure your database user has read and write privileges.

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

### Running the App
[Cargo Make](https://github.com/sagiegurari/cargo-make) as a task running to simplify starting and stopping the application.
To use cargo make, run `cargo install cargo-make`.

To start the application in development mode, run `cargo make start_all`.

To start the application in production mode, run `cargo make start_all_prod`.

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

To view all the available tasks, open Makefile.toml.