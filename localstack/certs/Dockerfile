# Stage 1: Certificate Generation (Builder)
FROM alpine:latest AS builder

RUN apk add --no-cache openssl

WORKDIR /tmp/gen

# Generate CA and Service Certs
RUN openssl genrsa -out root-key.pem 4096 && \
    openssl req -x509 -new -nodes -key root-key.pem -sha256 -days 3650 \
    -out root-cert.pem -subj "/CN=MyLocalCA" && \
    for service in redis user-api mongodb; do \
      # Create SAN config for each service
      echo "subjectAltName = DNS:${service}, DNS:localhost, IP:127.0.0.1" > ext.cnf; \
      # Generate Private Key
      openssl genrsa -out ${service}-key.pem 2048; \
      # Generate CSR
      openssl req -new -key ${service}-key.pem -out ${service}.csr -subj "/CN=${service}"; \
      # Sign with CA + SAN extension
      openssl x509 -req -in ${service}.csr -CA root-cert.pem -CAkey root-key.pem \
      -CAcreateserial -out ${service}-cert.pem -days 825 -sha256 \
      -extfile ext.cnf; \
    done && \
    # Create the combined PEM MongoDB specifically requires
    cat mongodb-cert.pem mongodb-key.pem > mongodb-combined.pem && \
    # Clean up signing artifacts
    rm *.csr *.srl ext.cnf

# Stage 2: Final Image
FROM alpine:latest

# 1. Create the directories
# /certs: This will be our mount point for the volume
# /certs_internal: This holds the permanent copies baked into the image
RUN mkdir -p /certs /certs_internal

# 2. COPY the files from the builder
COPY --from=builder /tmp/gen/*.pem /certs_internal/

# 3. Runtime Script
# This copies files FROM internal storage TO the mounted volume, 
# then sets permissions so non-root users (like Redis/Mongo) can read them.
CMD ["sh", "-c", "cp /certs_internal/*.pem /certs/ && chmod 644 /certs/*.pem && echo 'SSL Certificates initialized in volume.' && ls -l /certs"]