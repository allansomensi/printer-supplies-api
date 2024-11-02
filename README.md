# Printer Supplies API 🖨️

A **REST API** using **Axum** for managing **printer supplies**, such as toners and drums. The project includes CRUD functionalities for various resources and inventory movement tracking.

## Technologies Used

- **Axum:** Asynchronous web framework for Rust.
- **SQLx:** For interaction with the database and migrations.
- **Just:** For utility scripts.
- **Docker:** For database container.
- **Swagger:** API documentation.

## Features

- CRUD operations for movements, printers, brands, drums, and toners.
- Inventory management for toners and drums.
- Docker Compose setup for easy deployment.
- Database migrations included, executable via CLI or endpoint.
- Error handling with thiserror for descriptive error responses.
- Includes development, production, and test environments.
- Includes unit and integration tests.
- API documentation available with Swagger UI.

---

# Getting Started 🎯
## Prerequisites:

- **Rust** *(latest stable version)*
- **Docker** and **Docker Compose**
- **Just** for scripts

## 1. Installation

``` bash
git clone https://github.com/allansomensi/printer-supplies-api.git
cd printer-supplies-api
```
For the scripts:

``` elixir
cargo install just
```

## 2. Load environment variables

By default, the selected file is `.env.development`, so there's no need to run this command in a development environment.
If you're running in a **production** environment, execute the command below to ensure the correct environment variables are loaded.

``` bash
source scripts/set_env.sh
```

## 3. Build and run the Docker container:

``` elixir
just services-up
```

## 4. Run migrations:

``` elixir
just migrate-run
```

## 5. Start server 🚀 🚀 

``` elixir
just serve
```

# Running Tests 👨‍🔬

For once:
``` elixir
just test
```

For watching mode:
``` elixir
just test-watch
```

## Endpoint tests 🧪

> Endpoint tests are located at `/tests` folder

![Test Status](https://github.com/user-attachments/assets/9f6627c5-ae2d-42d5-8362-676330031027)

## API Documentation 📚

API endpoints and usage details are documented using `Swagger UI` and `OpenAPI` with `Utoipa`.

The full documentation is available in the `openapi.json` file, which can be accessed and imported as needed. Run the application and navigate to `/swagger-ui` to view the interactive Swagger documentation.
