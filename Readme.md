# Rust Command Line Application for Uploading Files into Azure Key Vault

This Rust command-line application allows you to securely upload files into Azure Key Vault. It uses the Rest API Call for Rust to interact with Azure Key Vault.

## Prerequisites

Before using this application, ensure you have the following:

- Rust programming language installed. You can install Rust from [rustup.rs](https://rustup.rs).
- Docker installed. You can download Docker from [here](https://www.docker.com/get-started).
- Docker Compose installed. You can download Docker Compose from [here](https://docs.docker.com/compose/install/).
- Azure account with access to Azure Key Vault. If you don't have an Azure account, you can create a free account [here](https://azure.microsoft.com/en-us/free/).
- Access to an Azure Key Vault and appropriate permissions to upload secrets.

## Installation

Clone this repository to your local machine:

```bash
git clone https://github.com/yourusername/your-repo.git
```

Navigate into the project directory:

```bash
cd azure-vault-file-upload
```

# Configuration

Before running the application, you need create `.env` file and to provide the following configurations:

```dotenv
VAULT_URL=
TENANT_ID=
CLIENT_ID=
CLIENT_SECRET=
API_VERSION=
```

## Usage

### Build Locally

To build the Rust application locally:

```bash
cargo build --release
```

### Run Locally

To run the application locally:

```bash
RUST_LOG=info cargo run -- -s <your-secret-name> -f <data-file-name>
```

**OR**

```bash
RUST_LOG=info ./azure-vault-file-upload -s <your-secret-name> -f <data-file-name>
```

`Note:` The application will upload the specified file into Azure Key Vault securely.

## Docker

### Build Image

You can also run this application using Docker. To build the Docker image

```bash
docker build -t azure_vault_file_upload .
```

### Create Data Directory

Create `data` folder in `.env` located path and upload your files

```bash
mkdir data
```

### Run Container with shell script

```bash
chmod + run_app.sh
./run_app.sh <your-secret-name> <data-file-name>
```

### Run in Docker Compose

1. Update the `docker-compose.yml` file if needed.
2. Run the following command:

```bash
docker-compose up
```

## Contribution

Contributions are welcome! Here's how you can contribute to this project:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Commit your changes (`git commit -am 'Add new feature'`)
5. Push to the branch (`git push origin feature/your-feature`)
6. Create a pull request

# License

This project is licensed under the Apache License 2.0.
