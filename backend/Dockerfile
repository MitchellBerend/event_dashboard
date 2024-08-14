# Use the official Golang image to build the app
FROM golang:1.22.6 AS builder

WORKDIR /app

# Copy the Go modules files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy the source code
COPY . .

# Build the Go app
RUN go build -o main .

# Use a smaller base image for the final build
FROM gcr.io/distroless/base-debian12

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/main .

# Command to run the application
CMD ["./main"]
