# train_ops_api

API for Train Ops

## Install / check required tools

Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)

## Configure

Configure environment variables :

Copy the file `.env.example` to a new file named `.env` 
And set your variables in this file

## Run

Run : `cargo run`

Test : http://localhost:8080/login

## Lint

    cargo clippy

## Docker

1. Build : `docker build -t trainopsapi .` 
1. Run : `docker run -it —rm —name trainopsapi-instance trainopsapi`
1. Test : http://localhost:8080/login