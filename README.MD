# Teo's data structures
## Description
Custom data structures that I use in my projects.

The whole idea of these data structures is to be as fast as possible.

I sacrifice some functionality for low overhead and speed.
The code is mainly targeted for X86_64 architecture.
There are no checks for special cases that can happen on other architectures.

I benchmarked the code on two different machines:

- AMD Ryzen 5 3600X (12 cores)
- AMD Ryzen 9 5950X (36 cores)

The results are as follows:

| Data structure | Push time | Pop time | Sum time |
| -------------- | --------- | -------- | -------- |
| Custom Array   | 1.5ms     | 0.5ms    | 0.7ms    |
| Vec            | 1.5ms     | 0.5ms    | 0.7ms    |

## Safety
I wrote tests for the most cases I could think of.
If you think something is missing contact me or submit a PR.

- The code itself should be safe.
- If you do something unexpected or use the structures incorrectly, they will panic
- There shouldn't be any undefined behavior

## Usage
Add this to your Cargo.toml
```toml
[dependencies]
teo-algorithms = "1.0.0"
```