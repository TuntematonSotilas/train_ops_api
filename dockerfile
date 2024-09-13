FROM rust:1.81
WORKDIR /usr/src/trainopsapi
COPY . .
RUN echo "DATABASE_URL=$DATABASE_URL" > .env
RUN cargo install --path .
CMD ["trainopsapi"]