extern crate reqwest;

use std::fs::{File, create_dir_all, set_permissions, Permissions};
use std::io::copy;
use std::io::Error;

fn create_file(filename: &str) -> Result<File, Error> {
    return {
        File::create(filename)
    };
}

pub fn download_file(url: &str, filename: &str) {
    println!("{} {} {}", "\u{1F4BE}", "Requesting", url);
    let mut response = match reqwest::get(url) {
        Ok(val) => val,
        Err(err) => panic!("Cannot download {} {}", url, err),
    };

    println!("{} {} {}", "\u{1F4BE}", "Creating", filename);
    let mut dest: File = match create_file(filename) {
        Ok(val) => val,
        Err(err) => panic!("Cannot create file {} {}", filename, err),
    };

    println!("{} {} {}", "\u{1F4BE}", "Copying data to", filename);
    match copy(&mut response, &mut dest) {
        Ok(val) => val,
        Err(err) => panic!("Copy content to file {}", err),
    };
}

pub fn unzip_file(filename: &str) {
    println!("{} {} {}", "\u{1F4BE}", "Unzipping", filename);
    let file_path = std::path::Path::new(filename);
    let file = File::open(&file_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("{} File {} comment {}", "\u{1F4BE}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("{} File {} extracted to {}", "\u{1F4BE}", i, outpath.as_path().display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!("{} File {} extracted to {} ({} bytes)", "\u{1F4BE}", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
                }
            }
    }
}
