#!/bin/zsh
read -s "postgres_password?postgres_password:"
postgres_uri=postgres://postgres:${postgres_password}@localhost/plainlyrusty

rm -f src/*
rm -rf target/debug/deps/*

sea-orm-cli generate entity -o src/entities -u $postgres_uri --seaography
seaography-cli-rc . src/entities $postgres_uri plainly-rusty
