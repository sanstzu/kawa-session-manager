FROM rust:alpine

WORKDIR /usr/src/app

COPY . .

RUN ["cargo", "build", "--release"]

EXPOSE 8888

EXPOSE 50052

CMD ["cargo", "run", "--release"]
