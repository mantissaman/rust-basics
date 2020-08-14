cargo install diesel_cli --no-default-features --features "postgres sqlite"

echo DATABASE_URL=postgres://mystore:pa55word@localhost/mystore > .env

diesel migration run
