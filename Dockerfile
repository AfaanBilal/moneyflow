#
# 💵 MoneyFlow
#
# @author Afaan Bilal
# @link   https://afaan.dev
# @link   https://github.com/AfaanBilal/moneyflow
#

#
# Stage 1 (Build)
#

FROM rust:1.82-slim-bullseye AS build

WORKDIR /moneyflow

COPY . .

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM gcr.io/distroless/cc

WORKDIR /moneyflow

COPY --from=build /moneyflow/target/release/moneyflow .

EXPOSE 80

CMD ["./moneyflow"]
