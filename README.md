# Compute@Edge Static Site

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

This project demonstrates using Fastly Compute@Edge to host a static site. It is based on the Compute@Edge default starter kit for Rust.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly Developer Hub](https://developer.fastly.com/solutions/starters/)**.

## Understanding the code

- `include_dir` is used to embed static files in the WebAssembly binary
- `mime_guess` is used to provide the correct content type for files

The starter doesn't require the use of any backends.
