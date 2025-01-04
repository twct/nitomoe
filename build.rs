use grass::{from_string, OutputStyle};
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let sass_base_path = Path::new("assets/scss");
    let css_base_path = Path::new("static/css");

    compile_scss_files(sass_base_path, css_base_path)?;

    Ok(())
}

fn compile_scss_files(sass_base_path: &Path, css_base_path: &Path) -> io::Result<()> {
    for entry in WalkDir::new(sass_base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "scss"))
    {
        let sass_path = entry.path();

        if sass_path
            .file_name()
            .and_then(|f| f.to_str())
            .map_or(false, |name| name.starts_with('_'))
        {
            continue;
        }

        let relative_path = sass_path.strip_prefix(sass_base_path).map_err(|e| {
            eprintln!("Failed to strip prefix: {:?}", e);
            io::Error::new(io::ErrorKind::Other, "Failed to strip prefix")
        })?;

        let css_path = css_base_path.join(relative_path).with_extension("css");

        compile_sass(sass_path, &css_path)?;
    }

    Ok(())
}

fn compile_sass(sass_path: &Path, css_path: &Path) -> io::Result<()> {
    let mut sass_content = String::new();
    let mut file = fs::File::open(sass_path)?;
    file.read_to_string(&mut sass_content)?;

    let options = grass::Options::default().style(OutputStyle::Compressed);

    match from_string(&sass_content, &options) {
        Ok(css) => {
            if let Some(parent) = css_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(css_path, css)?;
            println!("Compiled {} to {}", sass_path.display(), css_path.display());
        }
        Err(e) => {
            eprintln!(
                "Sass compilation failed for {}: {:?}",
                sass_path.display(),
                e
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
