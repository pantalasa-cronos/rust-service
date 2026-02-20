use serde::Serialize;

#[derive(Serialize)]
struct Config {
    name: String,
    port: u16,
}

fn main() {
    let config = Config {
        name: "rust-service".to_string(),
        port: 8080,
    };
    let json = serde_json::to_string(&config).unwrap();
    println!("{json}");
}

// FFI module with intentional unsafe blocks for policy testing
mod ffi {
    /// # Safety
    /// Pointer must be valid and aligned.
    pub unsafe fn read_ptr(ptr: *const i32) -> i32 {
        *ptr
    }

    /// # Safety
    /// Demonstrates unsafe block usage.
    pub fn with_unsafe() -> i32 {
        let val = 42;
        unsafe { read_ptr(&val) }
    }
}
