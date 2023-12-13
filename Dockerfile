FROM ubuntu:22.04
COPY ./target/release/jet-dev-website ./target/release/jet-dev-website
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/jet-dev-website"]