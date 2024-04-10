# Software Architecture Case Study with Rust

# Architecture Diagram
[From Draw.io](https://drive.google.com/file/d/1WBCnk8QiiBw8mf0VkOgjS3oYCYN40CXI/view?usp=drive_link)

# Workspace Structure

## Phase 1: 1 Binary

- Each Rust library is a separate service with a connecting `.rs.` file at `/root` which will serve as the eventual `main.rs` or `entrypoint` for containerizing in phase 2.

```
/root
    /client(binary to deploy for initial phase of coupled code)
        main.rs
    /frontend
        lib.rs
    /gateway
        lib.rs
    /event
        lib.rs
    /account.rs
        lib.rs
    /post.rs
        lib.rs
    /comment.rs
        lib.rs
    /persistence
        lib.rs
```

## Phase 2: Separated into containerized services

Utilizing the [[bin]] manifest option to compile each library into a service

```
/root
    [CONTAINER]
    /frontend
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /gateway
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /event
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /account.rs
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /post.rs
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /comment.rs
        /bin
            main.rs
        lib.rs

    [CONTAINER]
    /persistence
        /bin
            main.rs
        lib.rs
```

# Skill Objectives
1. Fullstack
2. Docker
3. Kubernetes
4. Grafana / Prometheus

# Software Architecture Objectives
1. Scalable (Decoupled)
2. Event-Driven Architecture
3. Hexagonal / Ports and Adapters Architecture (App Layer)
5. Onion Architecture (Presentation, Infra, Application, Domain)


