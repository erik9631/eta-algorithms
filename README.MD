## Description
Custom data structures that I use in my projects.

The whole idea of these data structures is to be as fast as possible.

I sacrifice some functionality for low overhead and speed.
The code is mainly targeted for X86_64 architecture.
There are no checks for special cases that can happen on other architectures.

I benchmarked the code on two different machines:

**BENCHMARK RESULTS TO BE UPDATED**

**Tested on:** 

*AMD Ryzen 9 7900X 12-Core Processor 4.70 GHz*

*32GB DDR4 5200MHz*

The results are as follows:

| Data structure | Push time avg | Pop time avg | Sum time avg |
|----------------|---------------|--------------|--------------|
| Custom Array   | -             | -            | 289ms        |
| Vec            | -             | -            | 309ms        |
| Queue Custom   | -             | -            | 16ms         |
| Deque          | -             | -            | 29ms         |
| Stack Custom   | -             | -            | 16ms         |
| Vec Stack      | -             | -            | 19ms         |

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
eta-algorithms = "1.0.2"
```