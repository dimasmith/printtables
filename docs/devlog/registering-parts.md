# Registering parts

Now when project registration is ready, I can to the same with parts. And it is really the same. No new libraries, no new approaches. Even some copy-paste action. Nothing to write about.

What's more interesting is a refactoring afterwards. Module duplication makes repetitive code more apparent. It's time to deal with it (to some extent). But before I start - there was, actually, one significant difference.

The `projects` module has the `infra` package. While it makes it more self-consistent, it also makes it less portable. The inventory module follows different approach. The infrastructure code is now a part of a core `infra` module. Axum handlers lives in the `server` module. So the inventory is stripped down to domain and application service.

Why the server is not in `infra` as well? Because it is one of the project interfaces. Remember, the initial plan includes a TUI? It will live in own core module. The infra module contains infrastructure, that will be reused by all, or most of interfaces. So the database handling goes to infra. Tracing should move here. And I'll include moving infra module from projects to general infra. Now, when the idea is clear, it's time to update architecture document. Maybe even add a layering view.

What else do I need to do?

Error handling in REST endpoints is a bit convoluted with excessive calls to `into_response()` method. I will define a set of typical errors translatable to REST, like `NotFound`, `InvalidPayload`, etc. It should make errors more uniform and simplify handling.

Validation deserves a few touches here and there. I copied the `Name` value object from the projects module to inventory. Probably not the best idea. I don't yet have a situation when multiple validation errors are possible, but it's time to prepare for that. The response must have a list of errors instead of a single error in the payload root.

