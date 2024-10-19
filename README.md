# PrintTables

PrintTables is a tracker of multipart 3D-prints where you need to print a batch of products.

## Core concepts

Keeping track of items in a printing project can be complicated, especially if you are using multiple printers.
The `printtables` helps to organise print jobs.

You goal with the printtables is to complete a *project*.
The project is a number of *assemblies* you need to build.
And each *assembly* consists of one or more parts.

Let's say you decided to organize your electronic components.
You need 5 bins to place small items, and 9 drawers to store other things.
The drawer has 2 parts: the enclosure, the insert and 2 stoppers.
It makes it a 3-part assembly:

```
+ Drawer
+-- Enclosure:  1
+-- Insert:     1
+-- Stopper:    2
```

The bin is much simpler. It's an assembly with only 1 part: the bin itself.
Let's combine assemblies in a project.

```
Organizers
+-- Drawer: 9
    +-- Enclosure:  1
    +-- Insert:     1
    +-- Stopper:    2
+-- Bin: 5
```

To complete the project you need to print all parts.
Printtables uses *progress* to keep track of printed parts.
You report a number of printed parts and see how many assemblies are ready, how close you are to completion and what else you need to print.

In project like this printing parts one-by-one is not optimal.
Usually it is better to place multiple identical parts on a single table.
In printtables you register *tables* - groups of parts you print together.
You can combine multiple tables into *schedule* to create a plan for your project.
Setting printing time on table helps predict time you need.

Of course, every table you create is specific to a model of the *printer* you use.
Printing speed differs across printer models.

Printing is, unfortunately, not ideal. 
When finishing a table you may report that some parts failed.
*Failures* allows to accurately keep track of remaining parts and to calculate the print success rate.

The opposite situation is possible.
You can end up having a surplus of printed parts.
Move those parts to *storage* if you know they might be used in future projects.

Assemblies, parts, and tables aren't enclosed in projects.
You may use the same assembly in different project.
If you decide to make yet another organization project with bins from one of your previous project, just add the bins from the *inventory*.

Finally, mark your project as ready and archive it.
You can review it later. 
Or you can create a copy if you need to print it again, or even modify a bit.

## Limitations

Printtables currently is a single-user project intended for local installation.
The collaboration between users is not the goal.
However, it might be added later.

## Implementation

Printtables is written in Rust and serves as a programming exercise.

It provides the REST API for front-end part.
Limited CLI/TUI ui is also in the works.
The backend uses Axum framework.

Data is stored in SQLite database.
The backend uses sqlx to access DB.

## Version goals

### `0.1.0` - A walking skeleton

This version should have major building blocks in place.
It should support creating projects.
This, in turn, includes creating parts and assemblies in the inventory.
Also project viewing and basic reporting on parts.

The main target platform for `0.1.0` is Raspberry PI. 
Any other platform support is not guaranteed.

### `0.2.0` - Table tracking

This version should support project scheduling and table management.

## Building

### Cross-compiling for Raspberry Pi

It takes a long time to compile directly on Raspberry Pi. 
Usually, it is more effective to use development machine for cross-compilation.

#### Prerequisites

You need to have Docker and [Cross](https://github.com/cross-rs/cross) installed on your machine.

> [!TIP]
> Install cross with `cargo install cross`

 #### Compiling

Now you can build the project using cross.

```shell
cross build --target aarch64-unknown-linux-gnu --release
```

The `target/aarch64-unknown-linux-gnu` now contains `printables-ui` and `printables-server` binaries.
Copy them to your Raspberry Pi and you can now run printtables.
