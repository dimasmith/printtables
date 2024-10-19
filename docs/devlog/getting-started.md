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