use std::path::Path;

use interoptopus::{Error, Interop};

fn generate_python(dir: &Path) -> Result<(), Error> {
    use interoptopus_backend_cpython::{Config, Generator};

    let library = fang::ffi_inventory();
    Generator::new(Config::default(), library).write_file(dir.join("fang.py"))?;

    Ok(())
}

fn generate_c(dir: &Path) -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator};

    let library = fang::ffi_inventory();
    Generator::new(Config::default(), library).write_file(dir.join("fang.h"))?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let dir = Path::new("bindings");
    if !dir.exists() {
        println!("Creating bindings directory..");
        std::fs::create_dir(dir)?;
    }

    println!("Generating python bindings..");
    generate_python(dir)?;

    println!("Generating c bindings..");
    generate_c(dir)?;

    Ok(())
}
