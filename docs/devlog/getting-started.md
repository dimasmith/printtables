# Getting Started

## `2024-10-19`

I decided to try and follow modular monolith pattern.

Two bounded contexts will live in their own top-level modules.
A general module structure is like this.

```
<module-root>
+-- domain          # contains domain items
+-- services        # auxiliary services, containing operations that can't be mapped to a single domain item
+-- app             # application-level services and functions. Implement use-cases
+-- infra           # infrastructure items implementing technical aspects for domains
```

Using Axum and sqlx mandates `async`. 
I'll need to specify abstractions implemented via the `infra` layer. 
So looks like `async-trait` crate is a must.

### Adding health check endpoint

#### Integration test

The very first task to get things up and running is to implement the health check endpoint.

The implementation with Axum is simple. It was the integration testing where things got slightly complicated.
I use awesome [Zero to Production in Rust](https://www.zero2prod.com/index.html) as the inspiration.
This book uses `actix` instead of `axum`. So I stuck a bit when writing a first integration test.

Actix has the `HttpServer` struct which I can pass to `tokio::spawn` to run a server in parallel to the test code.
In axum it's slightly different. I supposed to use `axum::serve` function returning a `Serve` struct. 
It does not implement `Future`, so I can't spawn it.

The trick that worked for me was to call `into_future` method on the struct.

```rust
pub async fn start_test_server() -> anyhow::Result<TestServer> {
    // Create a listener on a random open port
    let test_listener = TcpListener::bind("0.0.0.0:0").await?;
    // Store the local port for later use.
    let port = &test_listener.local_addr()?.port();
    // Initialize application router
    let app_router = router();
    // Instantiate the test server.
    // Don't call .await on it. We don't want it to start and block further processing.
    let test_server = axum::serve(test_listener, app_router);
    // Spawn the server in background.
    // the `into_future` converts it into the form `tokio::spawn` accepts.
    tokio::spawn(test_server.into_future());
    Ok(TestServer { port: *port })
}
```

According to the [example](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs) it is recommended to test routers without starting the server.
I am yet to explore this approach, but it seems to be great for unit testing.
I wanted to create a first integration test checking real server startup.

#### Code organisation

My initial idea was to keep at least part of server-related code in `bin/server`.
However, the router and a startup code should be shared with integration tests.
That's why I was forced to move it into the `lib` section.
Now the `server` module is responsible for initializing and controlling API server.
Individual domain routes are still planned for the respective modules.

```
+ server
    +-- routes
        +-- health
+ inventory
  +-- infra
      +-- rest
          +-- routes
```

### Cross-compiling

The main target platform for printtables is Raspberry Pi.
In a few previous projects I used [Cross](https://github.com/cross-rs/cross) to cross-compile binaries on my development machine.
Printtables is not an exception - I'll use the same tooling here.

The `build` directory has barebones dockerfile for aarch64 architecture.
It's possible to use any image with Cross, however their images seem to have almost no issues with dependencies.
I used debian:bookworm before, but run into notorious issues with OpenSSL.

Another part of the cross-compiling recipe is a Cross configuration.
Check the `Cross.toml` in the project root.
My configuration points to the dockerfile. 
Cross will build an image during the compilation.
Another way, which might be more performant, is to build the image from the file and refer to it.
However, using the dockerfile is so convenient, that I decided to postpone optimisations.

Cross works like a replacement for Cargo for some build tasks.
For example, building the project for Raspberry Pi can be done with the following command.

```shell
cross build --target aarch64-unknown-linux-gnu --release
```