FROM rust:1.63.0

WORKDIR /usr/src/rust-cloud-run
COPY . .

RUN cargo install --path .

CMD ["mongodb-todo-api"]
