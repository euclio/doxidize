extern crate doxidize;

#[macro_use]
extern crate slog;
extern crate tempdir;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use self::tempdir::TempDir;

use self::doxidize::Config;

pub struct ProjectBuilder {
    initialized: bool,
    dir: TempDir,
    files: HashMap<PathBuf, String>,
}

impl ProjectBuilder {
    /// The project will be built with a file at the given relative path.
    pub fn with_file<P: Into<PathBuf>>(mut self, path: P, contents: &str) -> Self
    where
        P: Into<PathBuf>,
    {
        self.files.insert(path.into(), String::from(contents));
        self
    }

    /// If true, the project will be initialized with `doxidize::ops::init` on build.
    pub fn initialized(mut self, initialized: bool) -> Self {
        self.initialized = initialized;
        self
    }

    pub fn build(self) -> Project {
        cargo_init(self.dir.path()).expect("could not initialize cargo project");

        let config = Config::with_manifest_path(self.dir.path().join("Cargo.toml"));

        let log = make_logger();

        if self.initialized {
            doxidize::ops::init(&config, &log).expect("problem in initialization");
        }

        for (path, contents) in self.files {
            let path = self.dir.path().join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut file = File::create(path).expect("could not create file");
            file.write_all(contents.as_bytes())
                .expect("could not write contents");
        }

        Project {
            dir: self.dir,
            config,
            log,
        }
    }
}

pub struct Project {
    dir: TempDir,
    pub config: Config,
    pub log: slog::Logger,
}

impl Project {
    pub fn dir(&self) -> &Path {
        self.dir.path()
    }
}

pub fn project() -> ProjectBuilder {
    let dir = TempDir::new("doxidize-test").expect("could not create tempdir");

    ProjectBuilder {
        dir,
        initialized: false,
        files: HashMap::new(),
    }
}

fn cargo_init(path: &Path) -> Result<(), Box<Error>> {
    let output = Command::new("cargo")
        .args(&["init", "--name", "example"])
        .current_dir(path)
        .output()
        .expect("failed to execute cargo init");

    if !output.status.success() {
        return Err(format!(
            "couldn't cargo init:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    Ok(())
}

/// by default we suppress all logging output
pub fn make_logger() -> slog::Logger {
    // use this if you want to enable it
    /*
    use slog::Drain;
    use slog_term;
    use slog_async;
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
    */

    slog::Logger::root(slog::Discard, o!())
}
