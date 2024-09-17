FROM ekidd/rust-musl-builder:1.57.0 AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ src/
RUN echo "DATABASE_URL=$DATABASE_URL" > .env
RUN rustup update
RUN cargo build --release 

# FROM alpine:3.20.3 AS runner
# ARG APP=/usr/src/app
# EXPOSE 8080
# COPY --from=builder /app/target/release ${APP}/
# RUN chmod 777 ${APP}/train_ops_api
# WORKDIR ${APP}

# CMD ["./train_ops_api"]