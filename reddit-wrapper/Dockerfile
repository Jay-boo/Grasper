FROM rust:1.79
WORKDIR /usr/src/myapp

ENV KAFKA_HOST="localhost:9092"

COPY . .
RUN cargo install --path .
CMD ["reddit-wrapper"]

