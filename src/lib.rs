//! Printtables is a project to keep track of multipart 3D-printing projects.
//!
//! The structure loosely follow the hexagonal architecture principles and DDD.
//! The application contains two subdomains: inventory and projects.
//!
//! Inventory keeps reusable parts that can be printed.
//!
//! Projects is concerned with organising those parts into printable plans and keeping track of
//! plans completion.
pub mod inventory;
pub mod projects;
pub mod server;
