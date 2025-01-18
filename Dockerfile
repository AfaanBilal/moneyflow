FROM rust:1.82-slim-bullseye AS build

WORKDIR /moneyflow

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /moneyflow

COPY --from=build /moneyflow/target/release/moneyflow .

EXPOSE 80

CMD ["./moneyflow"]
