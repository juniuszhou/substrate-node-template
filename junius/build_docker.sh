cd ..
cargo build --release
mkdir docker-tmp
cp target/release/node-template docker-tmp/
docker build docker-tmp -f junius/Dockerfile -t junius/node-template
rm -rf docker-tmp
