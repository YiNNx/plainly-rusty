cargo build --release --target x86_64-unknown-linux-musl

docker build -f deploy/build/Dockerfile . -t plainlyrusty:${IMAGE_VERSION} 
docker compose -f deploy/docker-compose.yml up