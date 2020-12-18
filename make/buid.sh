cd ../
cross build --target x86_64-unknown-linux-musl --release
cp -f target/x86_64-unknown-linux-musl/release/telegram-github-acttion docker/telegram-github-acttion