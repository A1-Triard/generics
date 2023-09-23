#![feature(start)]

#![deny(warnings)]

#![no_std]

use core::panic::PanicInfo;
#[cfg(not(windows))]
use libc::exit;
use libc_alloc::LibcAlloc;
#[cfg(windows)]
use winapi::shared::minwindef::UINT;
#[cfg(windows)]
use winapi::um::processthreadsapi::ExitProcess;

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[cfg(windows)]
unsafe fn exit(code: UINT) -> ! {
    ExitProcess(code);
    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(99) }
}

trait TheTrait { }

use generics::parse as generics_parse;
#[allow(unused_imports)]
use core::compile_error as std_compile_error;

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

struct Test<T: Clone>(T);

impl_the_trait!(Test<T: Clone>);

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _ = Test(0i32);
    0
}
