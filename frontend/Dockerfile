FROM node:hydrogen-alpine3.15 AS node

FROM rust:1.65.0-alpine AS build
# ARG BACKEND_URL
# RUN export BACKEND_URL=$BACKEND_URL
RUN apk add build-base
RUN rustup target add wasm32-unknown-unknown
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin
RUN npm install -g pnpm
WORKDIR /app
COPY ./frontend ./frontend
RUN cd frontend && pnpm install && cd /app
COPY ./database ./database
COPY ./shared ./shared
COPY docker.env ./frontend/.env
RUN cd frontend && pnpm run build

FROM busybox:1.35
WORKDIR /app
COPY --from=build /app/frontend/dist .
COPY ./frontend/httpd.conf /etc/httpd.conf
EXPOSE 8000
CMD ["busybox", "httpd", "-f", "-v", "-p", "8000"]