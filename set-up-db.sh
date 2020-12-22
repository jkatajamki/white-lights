#!/usr/bin/env bash

set -e

# Load env variables from local .env
set -o allexport; source .env; set +o allexport

docker-compose up --build -d postgres

WAIT_FOR_PG_ISREADY="while ! pg_isready --quiet; do sleep 1; done"
docker-compose exec postgres bash -c "$WAIT_FOR_PG_ISREADY"

docker-compose exec postgres \
  su - postgres -c "psql $DB_NAME -c '' || createdb $DB_NAME"

docker-compose exec postgres \
  su - postgres -c "psql $DB_TEST_NAME -c '' || createdb $DB_TEST_NAME"
