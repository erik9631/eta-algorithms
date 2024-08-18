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

#License
## License
Custom MIT-Based License

Copyright (c) 2024 Helecz Erik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

1. The above copyright notice and this permission notice shall be included in all
   copies or substantial portions of the Software.

2. The Software may be used for any purpose, including commercial applications.

3. **Users are prohibited from forking or creating copies of the Software's repository
   or codebase for the purpose of creating a separate distribution.**

4. Any modifications, improvements, or additions to the Software must be submitted
   as pull requests to the original repository. The copyright holder reserves the
   right to approve, modify, or reject these pull requests.

5. This license explicitly disallows the redistribution of the Software or any
   modified versions under any other license terms.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.