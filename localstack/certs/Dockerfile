# Stage 1: Certificate Generation (Builder)
FROM alpine:latest AS builder

RUN apk add --no-cache openssl

WORKDIR /tmp/gen

# Generate CA and Service Certs
RUN openssl genrsa -out root-key.pem 4096 && \
    openssl req -x509 -new -nodes -key root-key.pem -sha256 -days 3650 \
    -out root-cert.pem -subj "/CN=MyLocalCA" && \
    for service in redis user-api mongodb; do \
      openssl genrsa -out ${service}-key.pem 2048; \
      openssl req -new -key ${service}-key.pem -out ${service}.csr -subj "/CN=${service}"; \
      openssl x509 -req -in ${service}.csr -CA root-cert.pem -CAkey root-key.pem \
      -CAcreateserial -out ${service}-cert.pem -days 825 -sha256; \
    done

# Stage 2: Final Image
FROM alpine:latest

# 1. Create the final destination directory
RUN mkdir -p /certs

# 2. COPY the files from the 'builder' stage to this stage
COPY --from=builder /tmp/gen/*.pem /certs/

RUN mkdir -p /certs_internal && cp /certs/*.pem /certs_internal/

CMD ["sh", "-c", "cp /certs_internal/*.pem /certs/ && chmod 644 /certs/*.pem && echo 'Certs copied to host!' && ls -l /certs"]