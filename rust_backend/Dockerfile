# Start from the official Rust image
FROM rust:1.80.1

# Set the working directory inside the Docker container
WORKDIR /usr/src/app

# Copy the entire project into the container
COPY . .

# Build the project in release mode
RUN cargo build --release

# Expose the port that your Actix Web server will run on
EXPOSE 8081

# Command to run the application
CMD ["./target/release/rust_backend"]
