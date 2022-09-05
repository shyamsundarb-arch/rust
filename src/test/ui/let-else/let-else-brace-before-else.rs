// run-rustfix

#![feature(let_else)]

fn main() {
    let Some(1) = { Some(1) } else {
        //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
        return;
    };
    let Some(1) = loop { break Some(1) } else {
        //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
        return;
    };
    let 2 = 1 + match 1 { n => n } else {
        //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
        return;
    };
    let Some(1) = unsafe { unsafe_fn() } else {
        //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
        return;
    };
}

unsafe fn unsafe_fn<T>() -> T {
    unimplemented!();
}
