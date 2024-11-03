# Registering parts

Now when project registration is ready, I can to the same with parts. And it is really the same. No new libraries, no new approaches. Even some copy-paste action. Nothing to write about.

What's more interesting is a refactoring afterwards. Module duplication makes repetitive code more apparent. It's time to deal with it (to some extent). But before I start - there was, actually, one significant difference.

The `projects` module has the `infra` package. While it makes it more self-consistent, it also makes it less portable. The inventory module follows different approach. The infrastructure code is now a part of a core `infra` module. Axum handlers lives in the `server` module. So the inventory is stripped down to domain and application service.

Why the server is not in `infra` as well? Because it is one of the project interfaces. Remember, the initial plan includes a TUI? It will live in own core module. The infra module contains infrastructure, that will be reused by all, or most of interfaces. So the database handling goes to infra. Tracing should move here. And I'll include moving infra module from projects to general infra. Now, when the idea is clear, it's time to update architecture document. Maybe even add a layering view.

What else do I need to do?

Error handling in REST endpoints is a bit convoluted with excessive calls to `into_response()` method. I will define a set of typical errors translatable to REST, like `NotFound`, `InvalidPayload`, etc. It should make errors more uniform and simplify handling.

Validation deserves a few touches here and there. I copied the `Name` value object from the projects module to inventory. Probably not the best idea. I don't yet have a situation when multiple validation errors are possible, but it's time to prepare for that. The response must have a list of errors instead of a single error in the payload root.

## Common REST errors

I decide to start with common errors and their REST representation. The design is straightforward: simple enum with most common errors. I added the following entries:

- Not found error.
- Validation error.
- Internal error.

The internal error does not carry any additional information. While it may be surprising at first, I decided not to provide any information on error to the user. If something goes wrong - check the log. This, in turn, forces to think about a reliable logging policy. I'll write it into a todo list.

The validation error is more interesting. It accepts a list of validation messages and includes them in a payload. I'd like to build some convenience tools to validate multiple fields, but there are only names to check. Not much. Still, I decided to extract payload parsing to a separate function. Now it allows me to use the `?`operator: `parse_project(command)?`. Later it will be useful to process more fields.

## Collecting validator

I need something to collect validation errors from all payload fields. Despite my 2 entities have only one validated value - name - it's better to be ready. How to collect errors? I can create a list of `ValidationError` in the parse method and add any error that appears during field parsing. This approach is a bit verbose. So I decided to create a `CollectingValidator` to make things marginally easier.

The core of the validator is a method accepting any implementation of `TryFrom` using `ValidationError` as the error type. Method parsed the supplied payload and returns parsing result - literally what you'd do with a direct call to `try_from` on a value object. The added benefit is that once error happens, validator adds it to the internal errors list.

```rust
fn parse<V, P>(&mut self, payload: P) -> Result<V, ValidationError> where V: TryFrom<P, Error = ValidationError> {
  let result = V::try_from(payload);
  match result {
    Ok(value) => Ok(value),
    Err(invalid_payload) => {
      self.errors.push(invalid_payload.clone());
      Err(invalid_payload)
    }
  }
}
```

I'm still looking for a more elegant solution, but this should do for now.

On a side note: I love Rust's type system. The `V::try_from` line makes my old Java-fied brain happy.

