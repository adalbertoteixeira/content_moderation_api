FROM rust:1.82.0

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .
RUN sqlx migrate run
CMD ["content_moderation_api"]

