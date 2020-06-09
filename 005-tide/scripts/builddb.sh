#!/bin/sh

if [[ -z "$POSTGRES_USER" ]]; then
    export POSTGRES_USER=postgres
fi
if [[ -z "$POSTGRES_PASSWORD" ]]; then
    export POSTGRES_PASSWORD='pa55word'
fi
if [[ -z "$POSTGRES_DB" ]]; then
    export POSTGRES_DB=postgres
fi

export WITTER_USER=witteruser
export WITTER_DB='witter'
export WITTER_PASSWORD='pa55word'

echo "Rebuilding local DB started..."
base="$(date +%s)"

psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" << EOSQL
	UPDATE pg_database SET datallowconn = 'false' WHERE datname = '$WITTER_DB';
	SELECT pg_terminate_backend(pid)
	FROM pg_stat_activity
	WHERE datname = '$WITTER_DB';
	DROP DATABASE IF EXISTS $WITTER_DB;
	DROP USER IF EXISTS $WITTER_USER;
EOSQL

psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" << EOSQL
	CREATE USER $WITTER_USER with password '$WITTER_PASSWORD';
    CREATE DATABASE $WITTER_DB;
    GRANT ALL PRIVILEGES ON DATABASE $WITTER_DB TO $WITTER_USER;
	ALTER USER $WITTER_USER WITH SUPERUSER;

EOSQL

after="$(date +%s)"
elasped_seconds="$(expr $after - $base)"
echo "Full Database build successfully completed in $elasped_seconds seconds."