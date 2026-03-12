# catdex-api

This repository follows chapter 5 of Shing Lyu's book Practical Rust Projects Second Edition.

The book configures settings for a PostgreSQL database. I went with a MYSQL database instead. This means that some configuration code and SQL snippets differ from the book.

## Installation

```bash
# Set MYSQL Database URL
# `catdex` is the name of the database, you can change this.
export DATABASE_URL=mysql://user:password@localhost/catdex
# Install Diesel CLI
cargo install diesel\_cli --no-default-features --features mysql
# Create your `catdex` database
# and run your migrations.
diesel setup
```

## MYSQL Scripts

```sql
INSERT INTO cats
	(name, image_path)
values
	('Ragdoll', '/image/ragdoll.jpg'),
	('Persian', '/image/persian.jpg'),
	('British Short Hair', '/image/british-short-hair.jpg');
```

## Usage

```bash
# Serves a web app at http://localhost:8080
# With debug logs enabled.
RUST_LOG=debug cargo run
RUST_LOG=error cargo run
RUST_LOG=warn cargo run
RUST_LOG=info cargo run
RUST_LOG=trace cargo run
```

## Creating Database Cats via API

```bash
curl -F "name=Persian2" -F "image=@image/persian.jpg" http://localhost:8080/api/add_cat
```

## Validation

```bash
# Range error
curl -v localhost:8080/api/cat/9999
curl -v localhost:8080/api/cat/-1
```

## Useful Database Commands

• diesel migration run

• diesel migration revert: runs the down.sql of the most
recent migration

• diesel migration redo: runs the down.sql followed by up.
sql of the most recent migration. after running it your database
schema should be unchanged (but you might lose some data if
down.sql drops a table!). this is useful for verifying that your
down.sql works as intended.

## Testing

```bash
cargo test
```
