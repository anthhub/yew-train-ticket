FROM anthhub/rust-wasm:latest AS builder

ENV NODE_ENV production
WORKDIR /code
ADD . /code
RUN npm config set -g production false
RUN npm install
RUN ls
RUN npm run build

FROM nginx:alpine
COPY --from=builder /code/dist /usr/share/nginx/html
