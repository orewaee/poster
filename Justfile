database_url := env("DATABASE_URL", "poster.db")

dev-run:
    DATABASE_URL={{database_url}} cargo run run
