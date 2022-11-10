# askama-tera-hot-reload

This is a demo for a static site generator that uses Askama _and_ Tera for their different strengths.

There are two modes, _hot template_ using Tera, and _fixed template_, using Askama. Hot template mode is useful when you are still making changes to your templates. Fixed template mode is useful when the templates are finalized and you want faster renders.

| Mode           | Updates on CSS change | Updates on template change |
| -------------- | --------------------- | -------------------------- |
| hot template   | Yes                   | Yes                        |
| fixed template | Yes                   | No                         |

## Usage

The `hot-template` feature is enabled by default. To run in hot-template mode:

```
cargo run
```

To run in fixed-template-mode:

```
cargo run --features fixed-template --no-default-features
```

You could also have a mix of fixed templates and hot templates. To run in a mixed mode:

```
cargo run --features fixed-template
```

## Limitations
Askama and Tera templates are similar Jinja-based templates, but they are not exactly the same.
For example, Tera requires HTML comments to be enclosed in curly braces, while Askama does not.

## Future considerations

Perhaps one day, it will be possible to recast the [Askama AST to the Tera AST](https://github.com/djc/askama/issues/425).

There is also [LiveReload.js](https://github.com/livereload/livereload-js), which allows for showing updates without refreshing the page manually. This is what Zola uses. [tower-livereload](https://github.com/leotaku/tower-livereload) may be useful here.
