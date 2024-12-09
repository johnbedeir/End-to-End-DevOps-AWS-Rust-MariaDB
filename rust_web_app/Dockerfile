# Use the official Rust image
FROM rust:1.73-slim

# Set the working directory
WORKDIR /app

# Copy the application code
COPY . .

RUN cargo update 

# Expose the application port
EXPOSE 5000

# Use cargo run to start the application
CMD ["cargo", "run"]
