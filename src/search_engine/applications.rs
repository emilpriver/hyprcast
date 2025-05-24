use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::app;

fn list_applications() -> Result<Vec<PathBuf>> {
    let path_env = env::var("PATH")?;
    let path_dirs: Vec<&str> = path_env.split(':').collect();

    let mut applications: Vec<PathBuf> = Vec::new();

    for dir in path_dirs {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                let file_type = entry.file_type();

                if file_type.is_file() {
                    #[cfg(unix)]
                    {
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 != 0 {
                            applications.push(entry.path());
                        }
                    }

                    #[cfg(windows)]
                    {
                        // On Windows, check for common executable extensions
                        // This is a simplification; a more robust solution might
                        // involve checking file headers or relying on `CreateProcess`
                        // if you need to be absolutely sure something is executable.
                        let file_name = entry.file_name().to_string_lossy().to_lowercase();
                        if file_name.ends_with(".exe")
                            || file_name.ends_with(".bat")
                            || file_name.ends_with(".cmd")
                            || file_name.ends_with(".com")
                        {
                            applications.push(entry.path());
                        }
                    }
                }
            }
        }
    }

    Ok(applications)
}
