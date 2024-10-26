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

