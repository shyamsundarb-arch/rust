#![feature(raw_dylib, native_link_modifiers_verbatim)]

#[link(name = "extern_1.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern {
    fn extern_fn_1();
}

#[link(name = "extern_2", kind = "raw-dylib")]
extern {
    fn extern_fn_3();
}

pub fn library_function() {
    #[link(name = "extern_1", kind = "raw-dylib")]
    extern {
        fn extern_fn_2();
        fn print_extern_variable();
        static mut extern_variable: i32;
    }

    unsafe {
        extern_fn_1();
        extern_fn_2();
        extern_fn_3();
        extern_variable = 42;
        print_extern_variable();
        extern_variable = -42;
        print_extern_variable();
    }
}

fn main() {
    library_function();
}
