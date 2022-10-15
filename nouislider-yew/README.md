# nouislider-yew

An example that shows how to use library to use [noUiSlider](https://refreshless.com/nouislider/) within a [yew.rs](https://yew.rs) component.

## How to run the example

```sh
# Install required target files
rustup target add wasm32-unknown-unknown

cargo install trunk

# In current directory, so: src/examples/yew-component
trunk serve
```

Then open <http://localhost:8080>

## Github-pages deployment

Go to *Github -> Settings -> Pages*.

**Source**: Deploy from a branch

**Select branch**: `docs`

**Select folder**: `/docs`

**Save** and run from local repository

```sh
# Starting from root folder
git checkout -b docs
mkdir docs
trunk build --release --public-url nouislider-rs/ -d docs nouislider-yew/index.html
git add docs
git commit -m"Updated github-pages"
git push
```

## Demo

[https://jsjolund.github.io/nouislider-rs/](https://jsjolund.github.io/nouislider-rs/)

## License

nouislider-rs is licensed MIT.

It can be used for free and without any attribution, in any personal or commercial project.
