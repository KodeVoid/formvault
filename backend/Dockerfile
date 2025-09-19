FROM alpine:latest AS base

WORKDIR /app

# Copy the prebuilt binary
COPY formvault .

# Copy your environment file
COPY .env .

# Make the binary executable
RUN chmod +x ./formvault

# Default command
CMD ["./formvault"]
