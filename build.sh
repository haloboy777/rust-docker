
# Build
docker buildx build --platform linux/amd64 -t testactix .

# Run
# docker run --platform linux/amd64 -p 80:8080 --rm testactix