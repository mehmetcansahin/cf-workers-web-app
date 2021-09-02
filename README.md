# Cloudflare Workers Web App

Simple web app example for Cloudflare workers.

Built with [waserv](https://github.com/mehmetcansahin/waserv)

Cloudflare Workers + WASM + Rust = :heart:

### API Endpoints

- GET /
  {"message":"Hello, World!"}
- GET /hello/mehmetcan
  {"message":"Hello, mehmetcan!"}

### Install

1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. Install [Wrangler](https://developers.cloudflare.com/workers/tooling/wrangler/install)
3. Install wasm-pack

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

```bash
wrangler generate cf-wasm-worker-name https://github.com/mehmetcansahin/cf-workers-web-app.git
cd cf-wasm-worker-name
```

### Development

1. wrangler dev

### Benchmark

```bash
$ ab -c 10 -n 1000 https://cloudflare_worker_url/
```

Requests: 1k
Median CPU Time: 0.7ms

# 👷‍♀️🦀🕸️ `rustwasm-worker-template`

A template for kick starting a Cloudflare worker project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting worker to Cloudflare's worker infrastructure.

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## 🚴 Usage

### 🐑 Use `wrangler generate` to Clone this Template

[Learn more about `wrangler generate` here.](https://github.com/cloudflare/wrangler)

```
wrangler generate wasm-worker  https://github.com/cloudflare/rustwasm-worker-template.git
cd wasm-worker
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
