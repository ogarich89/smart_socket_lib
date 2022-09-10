mod bindings;

use libloading::Library;

pub struct SmartSocket {
    pub is_enabled: bool,
    pub voltage: i32,
}

type ToggleFn = unsafe extern "C" fn(&mut SmartSocket) -> ();
type GetStatusFn = unsafe extern "C" fn(&SmartSocket) -> String;

#[repr(C)]
pub struct Functions {
    pub size: usize,
    pub toggle: ToggleFn,
    pub get_status: GetStatusFn,
}

pub type Function = unsafe extern "C" fn() -> Functions;

pub struct Factory;

impl Factory {
    pub fn new() -> Result<Lib, anyhow::Error> {
        let lib = unsafe {
            let lib = Library::new(bindings::lib_path())?;
            Lib::new(lib)?
        };

        Ok(lib)
    }
}

impl SmartSocket {
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            voltage: 0,
        }
    }
}

pub struct Lib {
    functions: Functions,
}

impl Lib {
    pub unsafe fn new(lib: Library) -> Result<Self, anyhow::Error> {
        let load_fn: libloading::Symbol<Function> = lib.get(b"functions")?;
        let functions = load_fn();

        if functions.size != std::mem::size_of::<Functions>() {
            return Err(anyhow::Error::msg(
                "Lib Functions size != app Functions size",
            ));
        }

        Ok(Self { functions })
    }
    pub fn toggle(&self, smart_socket: &mut SmartSocket) {
        unsafe { (self.functions.toggle)(smart_socket) }
    }
    pub fn get_status(&self, smart_socket: &SmartSocket) -> String {
        unsafe { (self.functions.get_status)(smart_socket) }
    }
}
