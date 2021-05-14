ts := `/bin/date "+%Y-%m-%d_%H-%M-%S"`

### Database
initdb:
	set -e
	createuser -SDr rsapi
	createdb -O rsapi rsapi
	migrant setup
	migrant apply --all

dropdb:
	dropdb --if-exists rsapi
	dropuser --if-exists rsapi

updatedb:
	migrant apply --all

### Test
test:
	cargo test

### Run
run_debug:
	cargo build
	cp .env target/debug/
	cp log4rs.yml target/debug/
	bash -c "cd target/debug && rm -rf log && ./api"

run:
	cargo build --release
	cp .env_prod target/release/.env
	cp log4rs_prod.yml target/release/log4rs.yml
	bash -c "cd target/release && mkdir -p archived_logs && mkdir -p log && zip -r archived_logs/$(ts)_archived_logs.zip log && rm -f log/*.*  && ./api"
