use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Вказуємо Cargo перекомпілювати код, якщо файл валідатора змінився
    println!("cargo:rerun-if-changed=src/validator.zig");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("libvalidator.a");

    // Компілюємо Zig-код у статичну C-сумісну бібліотеку (.a) з максимальною оптимізацією
    let status = Command::new("zig")
        .args(&[
            "build-lib",
            "src/validator.zig",
            "-static",
            "-O",
            "ReleaseFast",
            "-lc",
            &format!("-femit-bin={}", dest_path.display()),
        ])
        .status()
        .expect("Невдалося запустити компілятор Zig! Перевірте, чи встановлено 'zig' у вашій системі.");

    if !status.success() {
        panic!("Компіляція Zig-коду завершилася з помилкою!");
    }

    // Передаємо параметри лінкування до Cargo
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=validator");
}
