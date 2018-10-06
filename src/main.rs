extern crate walkdir;
use walkdir::WalkDir;
use std::env;

const SIZE_NAMES: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB", "PB"];

fn file_size(fp: &std::path::Path) -> std::io::Result<u128> {
    Ok(fp.metadata()?.len() as u128)
}

fn dir_size(fp: &std::path::Path) -> u128 {
    let mut bytes: u128 = 0;
    for entry in WalkDir::new(fp).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            bytes += file_size(path).unwrap();
        }
    }
    bytes
}

fn humanize(byte_size: f64) -> String {
    if byte_size > 0_f64 {
        let p : i32 = byte_size.log(1024 as f64).floor() as i32;
        let i = 1024_f64.powi(p);
        String::from(format!("{:.2} {}", (byte_size/i), SIZE_NAMES[p as usize]))
    }
    else {
        String::from("0 B")
    }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
      let root = std::path::Path::new(args[1].as_str());
      if root.exists() {
          if root.is_file() {
              println!("{}", humanize(file_size(&root).unwrap() as f64));
          }
          else if root.is_dir() {
              println!("{}", humanize(dir_size(&root) as f64));
          }
      }
      else {
          eprintln!("Couldn't find file/dir");
      }
  }
  else {
      eprintln!("Usage: {} <file/dir>", args[0])
  }
}
