#!/bin/sh

if [[ -z "$POSTGRES_USER" ]]; then
    export POSTGRES_USER=postgres
fi
if [[ -z "$POSTGRES_PASSWORD" ]]; then
    export POSTGRES_PASSWORD='postgres'
fi
if [[ -z "$POSTGRES_DB" ]]; then
    export POSTGRES_DB=postgres
fi

export MYSTORE_USER=mystoreuser
export MYSTORE_DB='mystore'
export MYSTORE_PASSWORD='pa55word'

echo "Rebuilding local DB started..."
base="$(date +%s)"

psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" << EOSQL
	UPDATE pg_database SET datallowconn = 'false' WHERE datname = '$MYSTORE_DB';
	SELECT pg_terminate_backend(pid)
	FROM pg_stat_activity
	WHERE datname = '$MYSTORE_DB';
	DROP DATABASE IF EXISTS $MYSTORE_DB;
	DROP USER IF EXISTS $MYSTORE_USER;
EOSQL

psql --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" << EOSQL
	CREATE USER $MYSTORE_USER with password '$MYSTORE_PASSWORD';
    CREATE DATABASE $MYSTORE_DB;
    GRANT ALL PRIVILEGES ON DATABASE $MYSTORE_DB TO $MYSTORE_USER;
	ALTER USER $MYSTORE_USER WITH SUPERUSER;

EOSQL

after="$(date +%s)"
elasped_seconds="$(expr $after - $base)"
echo "Full Database build successfully completed in $elasped_seconds seconds."
