# Software Architecture Case Study with Rust

# Technical (Product) Documentation

[From Google Docs](https://docs.google.com/document/d/1XhoqWgHti_WArzu7oZJjJcr9yvebNP1C7r42s5Y0vyM/edit?usp=sharing)

This includes the Product Requirements documentation as well.


# Architecture Diagram

[From Draw.io](https://drive.google.com/file/d/1WBCnk8QiiBw8mf0VkOgjS3oYCYN40CXI/view?usp=drive_link)

# Persistence Schematic Diagram

[From Draw.io](https://drive.google.com/file/d/175X2peGS7EG9vrILSeNTbOD_UySyv49K/view?usp=drive_link)


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

# Roadmap

- Database Schemas
- Persistency Library
- Gateway Service
- Account Service
- Post Service
- Comment Service
- Event Bus

# Key Objectives

1. Language Objectives
2. Technology Objectives
3. Software Architecture Objectives

## Language Objectives

1. Rust for Web
2. Rust for Distributed and Scalable Systems
3. Async Rust
4. Monolithic Rust
5. Clean Rust (Locality of Behavior)

## Skill Objectives

1. Containerization with Docker 
2. Orchestration with Kubernetes
3. Deployment and Management with AWS / GCP
4. Instrumentation with Grafana / Prometheus
5. Event Driven with Kafka
6. CI/CD with Github Actions
7. Database Management with Postgres and SQLite

## Software Architecture Objectives

1. Scalable (Decoupled)
2. Event-Driven Architecture
3. Hexagonal / Ports and Adapters Architecture (App Layer)
5. Onion Architecture (Presentation, Infra, Application, Domain)


