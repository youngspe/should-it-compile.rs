//! [`should_compile!`]:        should_compile
//! [`should_not_compile!`]:    should_not_compile
//!
#![doc = include_str!("../README.md")]

/// Contains functions that are expected to compile.
///
/// ```
/// use should_it_compile::should_compile;
///
/// should_compile! {
///     prefix! {
///         let a = false;
///     }
///
///     fn good() {
///         let b: bool = a;
///     }
/// }
/// ```
///
/// You can also wrap the macro calls in `()` to allow rustfmt to format the tests:
///
/// ```
/// # use should_it_compile::should_compile;
/// should_compile!({
///     prefix!({
///         let a = false;
///     });
///
///     fn good() {
///         let b: bool = a;
///     }
/// });
/// ```
#[macro_export]
macro_rules! should_compile {
    // If the entire invocation is wrapped in braces, unwrap them.
    // The braces allow rustfmt to format the contents properly.
    { { $( $x:tt )* } } => {
        $crate::should_compile! { $($x)* }
    };

    {
        $( prefix! $prefix:tt $(;)? )*
        $(
            $(#$attr:tt)*
            $(fn $name:ident())?
            $(mod $mod:ident)?
            { $($body:tt)* }
        )*
        $(suffix! $suffix:tt $(;)? )*
    } => {
        $crate::__should_it_compile! {
            { mode = should_compile }
            [$(prefix! $prefix)*]
            [$({
                $(#$attr)*
                $(fn $name())?
                $(mod $mod)?
                { $($body)* }
            })*]
            [$(suffix! $suffix)*]
        }
    };
}

/// Contains functions that are expected not to compile.
///
/// ```
/// use should_it_compile::should_not_compile;
///
/// should_not_compile! {
///     prefix! {
///         let a = false;
///     }
///
///     fn bad() {
///         let b: i32 = a;
///     }
/// }
/// ```
///
/// You can also wrap the macro calls in `()` to allow rustfmt to format the tests:
///
/// ```
/// # use should_it_compile::should_not_compile;
/// should_not_compile!({
///     prefix!({
///         let a = false;
///     });
///
///     fn good() {
///         let b: i32 = a;
///     }
/// });
/// ```
#[macro_export]
macro_rules! should_not_compile {
    // If the entire invocation is wrapped in braces, unwrap them.
    // The braces allow rustfmt to format the contents properly.
    { { $( $x:tt )* } } => {
        $crate::should_not_compile! { $($x)* }
    };

    {
        $( prefix! $prefix:tt $(;)? )*
        $(
            $(#$attr:tt)*
            $(fn $name:ident())?
            $(mod $mod:ident)?
            { $($body:tt)* }
        )*
        $(suffix! $suffix:tt $(;)? )*
    } => {
        $crate::__should_it_compile! {
            { mode = should_not_compile }
            [$(prefix! $prefix)*]
            [$({
                $(#$attr)*
                $(fn $name())?
                $(mod $mod)?
                { $($body)* }
            })*]
            [$(suffix! $suffix)*]
        }
    };
}

/// Simply declares a module with `#[cfg(doc)] #[doc(hidden)]`
///
/// ## Example: External module
///
/// ### `lib.rs`
/// ```ignore
/// extern crate should_it_compile;
///
/// should_it_compile::compile_test_mod!(compile_test);
/// ```
///
/// ### `compile_test.rs`
/// ```ignore
/// use should_it_compile::should_compile;
///
/// should_compile! { /* ... */ }
/// ```
/// ## Example: Inline module
///
/// ### `lib.rs`
/// ```ignore
/// extern crate should_it_compile;
///
/// should_it_compile::compile_test_mod!(mod compile_test {
///     should_compile! { /* ... */ }
/// });
/// ```
#[macro_export]
macro_rules! compile_test_mod {
    (mod $($rest:tt)*) => { $crate::compile_test_mod! { $($rest)* } };
    ($mod:ident) => {
        #[cfg(doc)]
        #[doc(hidden)]
        mod $mod;
    };
    ($mod:ident { $($body:tt)* }) => {
        #[cfg(doc)]
        #[doc(hidden)]
        mod $mod {
            use $crate::macros::*;
            $($body)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __should_it_compile {
    {
        // `{ mode = should_compile }` or `{ mode = should_not_compile}`
        $mode:tt
        // e.g.
        // [prefix! { ... } prefix! ({ ... })]
        $prefix:tt

        // A list of `fn`s and `mod`s
        [$($test:tt)*]

        // [suffix! { ... } suffix! ({ ... })]
        $suffix:tt
    } => { $(
        $crate::__each_item! {
            $mode
            $prefix
            $test
            $suffix
        }
    )* };
}

// What to place after the backticks that begin the doctest
#[doc(hidden)]
#[macro_export]
macro_rules! __code_block_tag {
    ({ mode = should_compile }) => {
        ""
    };
    // Indicates that the doctest is expected not to compile
    ({ mode = should_not_compile }) => {
        "compile_fail"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __each_item {
    // When the item has a #[should_compile] attr, remove it and update the mode to should_compile
    {
        $mode:tt
        $prefix:tt
        { #[should_compile] $($item:tt)* }
        $suffix:tt
    } => {
        $crate::__each_item! {
            { mode = should_compile }
            $prefix
            { $($item)* }
            $suffix
        }
    };

    // When the item has a #[should_not_compile] attr, remove it and update the mode to
    // should_not_compile
    {
        $mode:tt
        $prefix:tt
        { #[should_not_compile] $($item:tt)* }
        $suffix:tt
    } => {
        $crate::__each_item! {
            { mode = should_not_compile }
            $prefix
            { $($item)* }
            $suffix
        }
    };

    // This item is a function
    {
        $mode:tt
        [$(prefix!
            // Allow prefix wrapped in { ... }
            $( { $($prefix1:tt)* } )?
            // Allow prefix wrapped in ({ ... }) to allow rustfmt
            $( ({ $($prefix2:tt)* }) )?
        )*]
        { fn $name:ident() { $($body:tt)* } }
        [$(suffix!
            // Allow suffix wrapped in { ... }
            $( { $($suffix1:tt)* } )?
            // Allow suffix wrapped in ({ ... }) to allow rustfmt
            $( ({ $($suffix2:tt)* }) )?
        )*]
    } => {
        #[cfg(doc)]
        #[doc(hidden)]
        // Output in the form of a doctest
        #[doc = $crate::_m::concat!(
            "````",
            // Use the mode to determine whether `compile_fail` is added:
            $crate::__code_block_tag!($mode),

            // Wrap the test in a function so when the mode is `should_compile`, the code is not
            // actually run, just compiled.
            "\n fn ",
            $crate::_m::stringify!($name),
            "() {\n",

            // Add prefixes before test body
            $crate::_m::stringify!(
                $(
                    $( $( $prefix1 )* )?
                    $( $( $prefix2 )* )?
                )*

                $($body)*

                // Add suffixes after test body
                $(
                    $( $( $suffix1 )* )?
                    $( $( $suffix2 )* )?
                )*
            ),

            "\n}\n````\n"
        )]
        #[allow(unused)]
        pub fn $name() {}
    };

    // This item is a module
    {
        { mode = $mode:ident }
        [$(prefix! $prefix:tt)*]
        { mod $mod:ident { $($body:tt)* } }
        [$(suffix! $suffix:tt)*]
    } => {
        #[cfg(doc)]
        #[doc(hidden)]
        #[allow(unused)]
        pub mod $mod {
            // Wrap the contents in a `should_compile` or `should_not_compile` invocation, depending
            // on the mode.
            $crate::macros::$mode! {
                $(prefix! $prefix;)*
                $($body)*
                $(suffix! $suffix;)*
            }
        }
    };
}

#[doc(hidden)]
pub mod macros {
    #[doc(no_inline)]
    pub use crate::{compile_test_mod, should_compile, should_not_compile};
}

#[doc(hidden)]
pub mod _m {
    pub use core::{concat, stringify};
}

#[cfg(doc)]
#[doc(hidden)]
compile_test_mod!(
    mod test {
        const _: () = {
            should_not_compile!({
                fn foo() {
                    let x: bool = 0;
                }
                #[should_compile]
                fn bar() {
                    let x: bool = true;
                }
            });
        };

        mod mod1 {
            should_compile!({
                prefix!({
                    let a = true;
                });
                mod inner1 {
                    fn foo() {
                        let a: bool = a;
                    }
                }
            });
        }

        mod mod2 {
            should_compile!({
                prefix!({});
            });
        }

        mod mod3 {
            should_not_compile!({
                fn foo() {
                    let x: bool = 0;
                }
                #[should_compile]
                fn bar() {
                    let x: bool = true;
                }
            });
        }

        mod mod4 {
            should_not_compile! {
                prefix! {
                    let a = 0;
                }

                fn foo() {
                    let b: bool = a;
                }
                #[should_compile]
                fn bar() {
                    let b: bool = a == 0;
                }
                fn baz() {
                    let b: i32 = a;
                }

                suffix! {
                    let c: bool = b;
                }
            }
        }

        mod mod5 {
            should_not_compile!({
                prefix!({
                    let a = 0;
                });
                fn foo() {
                    let b: bool = a;
                }
                mod inner1 {
                    prefix!({
                        let a = false;
                    });
                    #[should_compile]
                    fn foo() {
                        let b: bool = a;
                    }
                }
                suffix!({
                    let c: bool = b;
                });
            });
        }

        mod mod6 {
            should_not_compile!({
                prefix!({
                    let a = 0;
                });
                fn foo() {
                    let b: bool = a;
                }
                #[should_compile]
                mod inner1 {
                    prefix!({
                        let a = false;
                    });
                    fn foo() {
                        let b: bool = a;
                    }
                    #[should_not_compile]
                    fn bar() {
                        let b: i32 = a;
                    }
                }
                suffix!({
                    let c: bool = b;
                });
            });
        }
    }
);
