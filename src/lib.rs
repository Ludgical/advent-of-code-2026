use std::fmt::Display;
use std::time::Instant;

#[macro_export]
macro_rules! input {
    ($name:expr) => {{
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(file!()).parent().unwrap().join($name);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()))
    }};
}

pub fn time<T: Display>(label: &str, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("{label}: {result} ({elapsed:.2?})");
    result
}
