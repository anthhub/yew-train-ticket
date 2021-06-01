FROM anthhub/rust-wasm:latest AS builder

WORKDIR /code
ADD . /code

RUN npm config set -g production false
RUN npm install

ENV NODE_ENV production
RUN npm run build

FROM nginx:alpine
COPY --from=builder /code/dist /usr/share/nginx/html
