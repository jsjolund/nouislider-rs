# nouislider-rs in yew.rs example

An example that shows how to use library to use [noUiSlider](https://refreshless.com/nouislider/) within a [yew.rs](https://yew.rs) component.

## How to run the example

```sh
cargo install trunk
# in current directory, so: src/examples/yew-component
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
git checkout -b docs
trunk build --release --public-url nouislider-rs/ -d docs examples/yew-component/index.html
git add docs
git commit -m"Updated github-pages"
git push
```
