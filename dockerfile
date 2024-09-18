FROM rust:1.79 AS builder
ARG DATABASE_URL
WORKDIR /app
COPY Cargo.toml ./
COPY src/ src/
RUN echo "DATABASE_URL=$DATABASE_URL" > .env
RUN cargo build --release 

FROM debian:bookworm-slim AS runner
ARG APP=/usr/local/bin
EXPOSE 8080
COPY --from=builder /app/target/release ${APP}/
WORKDIR ${APP}
CMD ["train_ops_api"]