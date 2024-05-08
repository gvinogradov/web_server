# web_server
example rust deploy and cross compiling
- environment variables required:
  - APP_IPADDR
  - APP_PORT

# docker run
1) build --release --target x86_64-unknown-linux-musl 
2) docker build
3) docker run with 8080 port mapping

# local run
1) add environment for target .env file, example "CARGO_DOTENV_FILE=.env.docker"
   (you can use "APP_IPADDR=host.docker.internal" to run the application on this interface and have simple interconnect with other docker containers)
2) cargo build
3) cargo run --package web_server --bin web_server