[`should_compile!`]: #
[`should_not_compile!`]: #

This crate provides the macros
[`should_compile!`] and
[`should_not_compile!`], which convert the given function definitions to doctests.
These tests verify that the code does or does not compile, respectively.

## Usage

### Basic usage

```rust
use should_it_compile::{should_compile, should_not_compile};

should_compile! {
    fn code_that_should_compile() {
        // This cose should compile.
    }
}

should_not_compile! {
    fn code_that_should_not_compile() {
        compile_error!("This cose should compile.");
    }
}
```

### Advanced usage

```rust
use should_it_compile::should_compile;

should_compile! {}

should_compile! {
    prefix! {
        // This will be inserted before the contents of each function.
    }

    fn test1() {
        // This code should compile.
    }

    // The compilability expectation can be overridden with an attribute:
    #[should_not_compile]
    fn test2() {
        compile_error!("This code should not compile.");
    }

    // Tests can be grouped into modules:
    mod mod1 {
        prefix! {
            // This will be inserted after any enclosing prefix
        }

        fn test3() {
            // This code should compile.
        }

        suffix! {
            // This will be inserted before any enclosing suffix
        }
    }

    // The compilability expectation can be overridden for a submodule:
    #[should_not_compile]
    mod mod2 {
        fn test4() {
            compile_error!("This code should not compile.");
        }


        // The compilability expectation can be overridden inside a submodule:
        #[should_compile]
        fn test5() {
            // This code should compile
        }
    }

    suffix! {
        // This will be inserted after the contents of each function.
    }
}
```

## Running tests

The tests should be part of a standard `cargo test` run or a `cargo test --doc` run.
