FROM rust:alpine

WORKDIR /usr/src/app

RUN ["apk", "add" , "build-base", "protobuf", "protobuf-dev", "protoc"]

COPY . .

RUN ["cargo", "build", "--release"]

EXPOSE 8888

EXPOSE 50052

CMD ["cargo", "run", "--release"]
