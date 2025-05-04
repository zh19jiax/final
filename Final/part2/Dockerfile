FROM rust:1.70

# Install system dependencies
RUN apt-get update && \
    apt-get install -y libopenblas-dev liblapack-dev && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Build the project
RUN cargo build

# Run the project
CMD ["cargo", "run"] 