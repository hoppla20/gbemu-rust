# Code

## Running locally

```bash
# we want to execute the app
cd gbemu_rust_app

# native
cargo run

# web
trunk serve
# if you are having issues with SRI (integrity check)
TRUNK_BUILD_NO_SRI=true trunk serve
# after that you can visit https://localhost:8080/index.html#dev
# the '#dev' disables caching and will always load the latest build
```
