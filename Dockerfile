# Use the official Rust image as a build environment
FROM rust:bookworm as build

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build your Rust application
RUN cargo build --release

# Use a smaller base image for the runtime environment
FROM debian:bookworm-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the build environment to the runtime environment
COPY --from=build /app/target/release/azure-vault-file-upload /usr/src/app


# Copy the script file from local to the working directory inside the container
COPY bin/azure_upload.sh /usr/src/app/

# Make your script executable
RUN chmod +x azure_upload.sh

# Command to run your application
CMD ["sh", "azure_upload.sh"]