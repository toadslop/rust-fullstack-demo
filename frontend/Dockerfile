FROM node:hydrogen-alpine3.16 AS node

FROM rust:1.65.0-alpine3.16 AS build
WORKDIR /app

# Get node from node alpine
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin

# Install system requirements
RUN apk upgrade --update-cache --available && \
    apk add openssl build-base pkgconfig openssl-dev
ARG OPENSSL_DIR=/usr
RUN cargo install simple-http-server --root /app
RUN rustup target add wasm32-unknown-unknown
RUN npm install -g pnpm

# Actually build the app
COPY ./frontend ./frontend
RUN cd frontend && pnpm install && cd /app
COPY ./database ./database
COPY ./shared ./shared
COPY docker.env ./frontend/.env
RUN cd frontend && pnpm run build


FROM alpine:3.16
WORKDIR /app
COPY --from=build /app/frontend/dist ./static
COPY --from=build /app/bin/simple-http-server .
RUN chmod +x ./simple-http-server
EXPOSE 8000
CMD ["./simple-http-server", "./static", "-i", "-p", "8000", "--nocache", "--try-file", "./static/index.html"]