## Create a pg-container 
- podman run --rm -it --name=postgres-c -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres   postgres:13-alpine

- Run `cargo run`, migrations will fail with version 0.4 of sqlx

- Stop the container above and re run the same command to recreate the container
    - podman run --rm -it --name=postgres-c -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres   postgres:13-alpine

- Change the sqlx to use the git version of dependency in Cargo.toml
- Again run `cargo run`