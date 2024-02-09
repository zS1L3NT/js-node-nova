if [ ! -f .env ]; then
  echo "Clone the .env file before deploying"
  exit 1
fi
dotenv -- cargo build --release
mv target/release/rs-cli-nova ~/.bin/nova