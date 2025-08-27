default_database_url := "poster.db"
database_url := env("DATABASE_URL", default_database_url)

dev-run:
    cargo run run {{ if database_url != default_database_url { "--database-url " + database_url } else { "" } }}
