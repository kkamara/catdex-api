# catdex-api

This repository follows chapter 5 of Shing Lyu's book Practical Rust Projects Second Edition.

The book configures settings for a PostgreSQL database. I went with a MYSQL database instead. This means that some configuration code and SQL snippets differ from the book.

## Installation

```bash
# Install Diesel CLI
cargo install diesel\_cli --no-default-features --features mysql
# Set MYSQL Database URL
export DATABASE_URL=mysql://user:password@localhost/catdex
# Setup database
diesel migration run
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
cargo run
```

## Creating Database Cats

```bash
cd image
curl -F "name=Persian2" \ -F "image=@persian.jpg" \ localhost:8080/api/add_cat
```

## Useful Database Commands

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