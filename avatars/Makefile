
build:
	cargo build --release

migrate:
	cross-dotenv migrate -database 'cockroachdb://$$PG_USER:$$PG_PASSWORD@$$PG_HOST:$$PG_PORT/$$PG_DBNAME?ssl_disable=true' -path "./sql/" up
