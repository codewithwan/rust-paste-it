# PasteIt

PasteIt is a simple web application that allows users to create and share text or code snippets easily. This project includes a web interface and an API for creating and retrieving pastes.

## Features

- Create new pastes with a unique short link
- Retrieve pastes using the short link
- Responsive web interface
- API documentation for programmatic access

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Docker (optional, for running the database)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/codewithwan/rust-paste-it.git
    cd paste-it
    ```

2. Set up the environment variables:

    Create a `.env` file in the root directory with the following content:

    ```env
    DATABASE_URL=postgres://user:password@localhost/pasteit
    ```

3. Install the dependencies:

    ```sh
    cargo build
    ```

4. Run the database migrations:

    ```sh
    cargo install sea-orm-cli
    sea-orm-cli migrate up
    ```

5. Start the application:

    ```sh
    cargo run
    ```

<!-- ### Running with Docker

1. Build the Docker image:

    ```sh
    docker build -t paste-it .
    ```

2. Run the Docker container:

    ```sh
    docker run -d -p 8080:8080 --env-file .env paste-it
    ``` -->

## API Documentation

The API documentation is available at `/api-docs`. It provides detailed information on how to use the API endpoints for creating and retrieving pastes.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
