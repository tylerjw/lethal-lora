FROM rust:1.71

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["lethal-lora"]
