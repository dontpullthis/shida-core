use std::collections::LinkedList;
use std::fs;

use libloading::{Library, Symbol};

type CanHandleFunc = fn(connection_type: *const libc::c_char) -> bool;

pub struct Module {
    pub can_handle: CanHandleFunc,
}

pub struct LibModule {
    lib: Library,
    pub module: Module,
}

impl LibModule {
    fn new(lib: Library, module: Module) -> LibModule {
        LibModule {
            lib,
            module
        }
    }
}

fn call_dynamic(path: String) -> Result<LibModule, Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new(path)?;
        let load_func: Symbol<unsafe extern fn() -> Module> = lib.get(b"load")?;
        let loaded_module = load_func();
        Ok(LibModule::new(lib, loaded_module))
    }
}

pub fn load_modules() -> LinkedList<LibModule> {
    let mut result: LinkedList<LibModule> = LinkedList::new();

    let paths = fs::read_dir("./.dev/modules").unwrap(); // TODO: replace with configurable path
    for path_item in paths {
        let path_dir_entry = match path_item {
            Ok(p) => p,
            Err(_) => continue,
        };
        let lib_path = path_dir_entry.path().into_os_string().into_string().unwrap();
        match call_dynamic(lib_path) {
            Ok(lm) => {
                result.push_back(lm);
            },
            Err(_) => println!("Error"),
        };
    }

    result
}