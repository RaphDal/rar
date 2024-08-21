# RAR - Rust Async Runtime

RAR (Rust Async Runtime) is a small, experimental asynchronous runtime built in Rust. 

This project is an experiment that explore Rust's async/await capabilities and async runtimes intrinsics. **RAR is not intended for production use.**

## Features

- **Custom Executor:** Implements a basic task executor for running and scheduling asynchronous tasks.
- **Configurable Multithreading:** RAR allows you to configure the number of threads it uses for running tasks, enabling simple experimentation with multithreaded async execution.
- **No External Dependencies:** Built entirely from scratch, RAR doesn't rely on any external crates, making it a pure exploration of Rust's async capabilities.
- **Simple and Lightweight:** Focused on being a minimal and understandable implementation, RAR is great for those looking to learn more about how async runtimes work.

## Installation

RAR is not published on crates.io and is not suitable for production. If you want to experiment with it, clone the repository:

```sh
git clone https://github.com/raphdal/rar
```

## Usage

RAR allows you to configure the number of threads it uses to run asynchronous tasks. Below is an example of how to use RAR in a project for experimentation:
```rust
use rar::builder::Builder;
use rar::context;

fn main() {
    // Create a scheduler with 4 threads
    let scheduler = Builder::new().threads(4).build();

    // Use the scheduler to block on an async task
    scheduler.block_on(async {
        for _ in 0..10 {
            // Spawn multiple async tasks
            context::spawn(async {
                // Example task
                println!("Running an async task!");
            });
        }
    });
}
```

In this example:

- We create a scheduler with a configurable number of threads.
- We use block_on to run an async block until completion.
- Inside the async block, multiple tasks are spawned to demonstrate how RAR handles concurrent task execution.

# Project Structure
- builder.rs: Configure and build the scheduler.
- context.rs: Manage the task execution.
- scheduler.rs: Forwards the future and blocks until completion.
- shared_context.rs: Track the tasks that should be handled and the working threads.
- task.rs: Build the waker with the necessary structs.
- waker.rs: Implement the waker for async runtime.
  
# Disclaimer
RAR is a highly experimental project designed for learning purposes only. It is not optimized, not thoroughly tested, and should not be used in any production environment. Expect incomplete features and potential bugs.