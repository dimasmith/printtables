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