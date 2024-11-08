set dotenv-required
set dotenv-filename := x'environments/.env.${ENVIRONMENT:-development}'

default:
    @just serve

[group: 'database']
migrate-add MIGRATION_NAME:
    @cargo sqlx migrate add {{ MIGRATION_NAME }}

[group: 'database']
migrate-run:
    @cargo sqlx migrate run

[group: 'database']
migrate-down:
    @cargo sqlx migrate revert

[group: 'misc']
serve:
    @just run-watch

[group: 'misc']
run:
    @cargo run

[group: 'misc']
run-watch:
    @cargo watch -q -c -x run -i logs/

[group: 'test']
test:
    @cargo test

[group: 'test']
filter PATTERN:
    @cargo test {{PATTERN}}

[group: 'test']
test-watch:
    @cargo watch -q -c -x test

[group: 'check']
clippy:
    @cargo clippy --all --all-targets --all-features -- --deny warnings

[group: 'check']
lint:
    @cargo fmt --all -- --check
    @cargo clippy --all --all-targets -- --deny warnings

[group: 'check']
lint-fix:
    @cargo fmt --all
    @cargo clippy

[group: 'docs']
docs CRATE:
    @open "https://docs.rs/{{CRATE}}"

[group: 'misc']
clean:
    @cargo clean