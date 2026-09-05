use std::{io, fs};
use colored::Colorize;
use std::path::Path;

fn main() -> io::Result<()> {
    //Get directories
    let entries: fs::ReadDir = fs::read_dir(".")?;
    
    //Loop through each entry
    for entry in entries {
        
        //Error handling for each entry
        let entry = entry?;

        println!("{:>8} {}", format_size(get_size(&entry.path())?), get_color(&entry)?);
    }
    Ok(())
}

//Function to get the color of the entry
fn get_color(entry: &fs::DirEntry) -> io::Result<colored::ColoredString> {
    if entry.file_type()?.is_dir() {
        Ok(entry.file_name().to_string_lossy().into_owned().red())
    } else {
        Ok(entry.file_name().to_string_lossy().into_owned().white())
    }
}

fn get_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    // Si es un directorio, leemos lo que hay dentro con read_dir
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                let entry = entry?;
                // ¡RECURSIÓN! Llamamos a get_size con la ruta del hijo
                total_size += get_size(&entry.path())?;
            }
        }
    } else {
        // Si es un archivo normal, sumamos su tamaño
        total_size = fs::metadata(path)?.len();
    }
    Ok(total_size)
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        return format!("{:.2} B", size as f64);
    }else if size < 1024 * 1024 {
        return format!("{:.2} KB", size as f64 / 1024.0);
    }else if size < 1024 * 1024 * 1024 {
        return format!("{:.2} MB", size as f64 / (1024.0 * 1024.0));
    }else if size < 1024 * 1024 * 1024 * 1024 {
        return format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0));
    }else {
        return format!("{:.2} TB", size as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0));
    }
}

/*La meva
fn get_size(entry: &fs::DirEntry) -> io::Result<u64> {
    let mut size = entry.metadata()?.len();
        if entry.is_dir() {
            for entry in entry.iter() {
                let entry = entry?;
                size += get_size(&entry)?;
            }
        }else{
            size = entry.metadata()?.len();
        }   
    Ok(size.to_string())
}
    */