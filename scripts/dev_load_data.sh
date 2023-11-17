#!/bin/zsh
read -s "postgres_password?postgres_password:"
postgres_uri=postgres://postgres:${postgres_password}@localhost/plainlyrusty

psql -q $postgres_uri < scripts/plainly_rusty_create.sql
