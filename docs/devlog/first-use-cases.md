# First use cases

It's time to add a first use case to our project. 
It allows completing walking skeleton with necessary feature.

The very first use case won't make an application useful yet. 
Nevertheless, it is a step forward in development.

## Creating project

The use case we are about to implement is creating a project.
An empty project is useless.
Anyway, we need to start somewhere, so why not start at the very core?

We also need to implement View project use case to check that the creation is successful.

What do we need in empty project? A bare minimum would be a name, an ID, and a status.
The status reflects completion progress of a project.

While the initial idea was to use status `Done` immediately on creation, that might be not the best option.
Most projects will be empty in the very beginning, before maker adds any assemblies to them.
So it may be useful to assign a `New` status, marking that the project needs configuration.

Question on project states.

- What are the necessary states?
- What are the transitions between states?
- Do states belong to a project itself, or are they a part of other concept, e.g., progress?

I'll deal with statuses later. For now, let's focus on the creation itself.

### Implementing services and repositories

I want an application service to handle project use cases.
One component it needs is a repository to store projects.
The more-or-less standard approach is to have a service parametrised with the repository.

```rust
pub struct ProjectService<R> {
    repository: R,
}
```

The thing is, if I decide to pick another implementation of a repository, I'll have to change all clients of the service.
The `ProjectService<SQLiteRepository> and ProjectService<InMemoryRepository>` are different types.

So I decided to introduce yet another trait for the service itself.

```rust
pub trait ProjectService {
    fn create_project(&self, name: String) -> Result<Project, Error>;
}

pub struct DefaultProjectService<R: ProjectRepository> {
    repository: Arc<R>,
}
```

This way I can hide the actual repository type I use.
Not the most elegant solution.
I think I'll have to revisit it later.
However, it should be good enough for now.

I'm postponing dealing with `sqlx` and trying to create an in-memory repository first. And here's where the simplest approach was not the best one.

```rust
pub trait ProjectRepository {
    fn create(&mut self, project: Project) -> Result<Project, Error>;
}

pub struct InMemoryProjectRepository {
    projects: HashMap<Uuid, Project>,
}
```

Note that the `create` method requires mutable reference to the repository.

And here is a service.

```rust
pub struct DefaultProjectService<R: ProjectRepository> {
    repository: Arc<R>,
}

impl<R: ProjectRepository> ProjectService for DefaultProjectService<R> {
    fn create_project(&self, name: String) -> Result<Project, Error> {
        let project = Project {
            id: Uuid::new_v4(),
            name,
            status: ProjectStatus::New,
        };

        self.repository.create(project)
    }
}
```

The `self.repository.create(project)` call won't compile.
I cannot get a mutable reference to the repository from `Arc`.

I could use `Mutex` or `RwLock` to wrap the repository.
That, however, would require me to lock the repository every time I want to use it.
And it will severely limit the number of concurrent operations.

It's not a problem, actually. Remember, printtables is a single-user self-hosted application. 
Still, I don't like this approach.

Instead of this, I'll use interior mutability pattern in the in-memory repository.

```rust
pub trait ProjectRepository {
    fn create(&self, project: Project) -> Result<Project, Error>;
}

pub struct InMemoryProjectsRepository {
    storage: Mutex<HashMap<ProjectId, Project>>,
}

impl ProjectRepository for InMemoryProjectsRepository {
    async fn create(&self, project: Project) -> anyhow::Result<ProjectId> {
        let mut storage = self.storage.lock().await;
        let id = project.id();
        storage.insert(id, project);
        Ok(id)
    }
}
```

Now that compiles ok.

The `sqlx` uses internal mutability as well, so my repository trait should be compatible with the real implementation.

Yes, the in-memory repository is locked on every operation.
But as it won't be used in production, I don't care too much.

### Async trait crate

I must confess that I do not completely understand the magic of a `async_trait` crate.
So I refused to use it and went with #[allow(async_fn_in_trait)].
That worked well until I tried to use the service in the REST handler.

```rust
async fn register_project(State(project_service): State<Arc<dyn ProjectService>>) {}
```

Compiler stopped me saying that the traits with async methods are not object-safe. 
So this trick won't work. 
The `async_trait` crate for the rescue once again :)

### Implementing endpoints

Everything is simpler if you first read the documentation and experimenting afterwards.
That's not how I do things unfortunately. But even that did not get in a way of implementation.

Handlers are functions accepting extractors and returning responses.
You just need to pick extractors working for you. 
I have a few pieces missing in the error handling still.
One prominent issue is the 500 response when the project is not found.
It's because I decided to have a dedicated error in the service to mark this situation.
I did not implement that yet, so it should be one of the next points.
The project repository has the separation, as it returns the `Result<Option<Project>>`.
It's tempting to get things right, but I'll postpone it and will take care of real database access.
This may end up in changing repository trait a bit.

But wait. On the other hand - when was the last time I created a test? 
I now have an interface to test my first use case.
Off to testing!

## Payload validation

Do not validate - parse. It is a guiding principle I want to employ in this project. And boy oh boy it looks so reasonable when remembering all validation-related disasters from my 20 years with Java. No, it's not a Java issue - it's just not that good usage of it. 

So the idea is to replace simple types in domain objects with safe wrappers that allow only valid values.

```rust
struct Name {
  pub fn parse(value: String) -> Result<Self, ValidationError> {
    // do validation
    // return self only when everyting is OK
  }
}
```

It's safe to pass this object down. There's guarantee that the value is intact. E.g., the name is not empty and not too long.

Implementing the `TryFrom` trait informs other developers about your intention. And the `AsRef` trait allows representing inner value. It is important in infrastructure code, like database access.

Implementing validation this way is a bit verbose - I assume there are crates allowing to reduce boilerplate. However, it is really convenient.

Another component I need is some kind of validation error. I come out with the struct like that for now.

```rust
struct ValidationError {
  message: String,
  attribute: String,
  code: String,
}
```

Now it's a matter of implementing the `IntoResponse` trait for the error to present validation errors as responses with code 400.

