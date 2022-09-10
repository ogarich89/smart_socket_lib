type ToggleFn = unsafe extern "C" fn(&mut SmartSocket) -> ();
type GetStatusFn = unsafe extern "C" fn(&SmartSocket) -> String;

#[derive(Clone, Copy)]
struct SmartSocket {
    is_enabled: bool,
    voltage: i32,
}

#[repr(C)]
pub struct Functions {
    size: usize,
    toggle: ToggleFn,
    get_status: GetStatusFn,
}

impl Default for Functions {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<Self>(),
            toggle,
            get_status,
        }
    }
}

#[no_mangle]
pub extern "C" fn functions() -> Functions {
    Functions::default()
}

unsafe extern "C" fn toggle(smart_socket: &mut SmartSocket) {
    smart_socket.is_enabled = !smart_socket.is_enabled;
    smart_socket.voltage = if smart_socket.is_enabled { 100 } else { 0 };
}

unsafe extern "C" fn get_status(smart_socket: &SmartSocket) -> String {
    let status = if smart_socket.is_enabled {
        "enabled"
    } else {
        "disabled"
    };

    format!("Status: {}, voltage: {}", status, smart_socket.voltage)
}
