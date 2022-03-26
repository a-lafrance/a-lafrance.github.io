# My Personal Site
About me, briefly

Site is implemented in Rust w/ [Yew](yew.rs); there was a previous version implemented in Vue, but at some point I decided to switch because Rust/Yew seemed more fun that maintaining a JS web project.

If, for whatever reason, you want to build the site locally, use `trunk`. Assuming you have Rust installed, install `trunk` with `cargo install trunk`, and run the following commands to build/run the site:
* `trunk serve`: like `npm run serve` in the old Vue project, build a local instance of the site.
* `trunk build --release`: like `npm run build` before, build a release version of the site for distribution/deployment.
