#!/bin/zsh
read -s "postgres_password?postgres_password:"
postgres_uri=postgres://postgres:${POSTGRES_PASSWORD}@localhost/plainlyrusty

sudo POSTGRES_PASSWORD=${POSTGRES_PASSWORD} docker-compose -f deploy/docker-compose.yml up