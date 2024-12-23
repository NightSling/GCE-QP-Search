# Uses a Builder & a Runner
# Builder: Builds the app to 

FROM rust:slim AS builder

WORKDIR /usr/app
COPY . .
RUN cargo build --release

# Runner: Runs the app
FROM rust:slim

WORKDIR /usr/app
COPY --from=builder /usr/app/target/release/gce-hoster ./app
COPY --from=builder /usr/app/static ./static

ENV ROCKET_ENV=production
ENV STATIC_FOLDER=/usr/app/pdfs
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

VOLUME /usr/app/pdfs

CMD [ "./app" ]