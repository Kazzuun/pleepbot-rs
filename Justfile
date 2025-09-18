set dotenv-required
set dotenv-load

database_url_base := "postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432"

default:
    @just --list

@db-init:
    for crate in `ls ./apps`; do \
        sqlx database create -D {{database_url_base}}/$crate \
            && echo "Created database for $crate"; \
    done

@db-setup:
    for crate in `ls ./apps`; do \
        sqlx database setup --source ./apps/$crate/migrations -D {{database_url_base}}/$crate \
            && echo "Setup database for $crate"; \
    done

[confirm]
@db-drop:
    for crate in `ls ./apps`; do \
        sqlx database drop -D {{database_url_base}}/$crate \
            && echo "Dropped database $crate"; \
    done

@db-schema-dump:
    for crate in `ls ./apps`; do \
        PGPASSWORD="${POSTGRES_PASSWORD}" pg_dump --schema-only \
            -h localhost \
            -p 5432 \
            -U ${POSTGRES_USER} \
            -d $crate \
            --no-owner --no-privileges \
            > ./apps/$crate/docs/schema.sql \
        && echo "Dumped database schema for $crate"; \
    done

migrate-create crate description:
    sqlx migrate add -s -r --source ./apps/{{crate}}/migrations {{description}}

alias migrate-up := migrate-run
migrate-run crate *args:
    sqlx migrate run --source ./apps/{{crate}}/migrations -D {{database_url_base}}/{{crate}} {{ args }}

alias migrate-down := migrate-revert
migrate-revert crate *args:
    sqlx migrate revert --source ./apps/{{crate}}/migrations -D {{database_url_base}}/{{crate}} {{ args }}

@sqlx-prepare:
    for crate in `ls ./apps`; do \
        cargo sqlx prepare --database-url {{database_url_base}}/$crate --workspace -- --bin $crate \
            && echo "Prapered sqlx queries for $crate"; \
    done
