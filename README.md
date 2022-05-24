![maintenance: actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# generics-parse

Provides macros for parsing generics and where clauses in `macro_rules!`.

```rust
pub trait TheTrait { }

#[doc(hidden)]
pub use generics::parse as generics_parse;
#[doc(hidden)]
pub use std::compile_error as std_compile_error;

#[macro_export]
macro_rules! impl_the_trait {
    (
        $name:ident $($token:tt)*
    ) => {
        $crate::generics_parse! {
            $crate::impl_the_trait {
                @impl $name
            }
            $($token)*
        }
    };
    (
        @impl $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*]
    ) => {
        impl $($g)* $crate::TheTrait for $name $($r)* $($w)* { }
    };
    (
        @impl $name:ident [$($g:tt)*] [$($r:tt)*] [$($w:tt)*] $($token:tt)+ 
    ) => {
        $crate::std_compile_error!(
            "invalid input, allowed input is '$name $( < $generics > $(where $where_clause)? )?'"
        );
    };
}
```
