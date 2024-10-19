# Utility tree

| Attribute | Refinement | Scenario | Business impact | Technical complexity |
| --- |------------| --- | --- | --- |
| Performance | Throughput | | Low | Medium |
| Maintainability | Routine changes | A maintainer identifies a bug in application logic and fixes it. It takes no more than 4 working hours to fix 90% of issues. | High | Medium |
| Maintainability | Data schema migration | A new version of application introduces changes to a database schema. The migration happens automatically without data coruption | High | Medium |
| Observability | Logging | A maintainer sees incorrect application behaviour. The application log in debug level contains sufficient amount of data to identify the root cause | Medium | Medium |
| Deployability | Deployability | A user can deploy new or updated system to a target platform using minimum amount of configuration. | Medium | Medium |
| Accuracy | Data integrity | A user creates and ties together project and inventory items. The data in the storage is unambigious | High | Low |
| | Backups | A user copies database and configuration files to an archive on a daily basis. Those artifacts are enough to recover the system after the failure | High | Medium |

## Context

Currently, the printtables aims at individual users running the application in a home network.
Neither performance, nor security is critical in this scenario.

Later versions of the application will pay more attention to security, even if the only reason will be following good practices.

It must be easy to cross-compile and deploy the service to a small home server.
Currently, the app targets Raspberry Pi as a main platform.
Easy backups and recovery are must-have in this scenario.

