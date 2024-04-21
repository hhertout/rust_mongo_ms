FROM rust:1.77.2-slim-bullseye

RUN cargo install cargo-watch

RUN apt-get -y update
RUN apt-get -y upgrade

WORKDIR /app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ]

