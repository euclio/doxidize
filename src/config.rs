use analysis;
use handlebars::{self, Handlebars};
use serde::Deserializer;
use toml_edit;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

/// A structure that contains various fields that hold data in order to generate doc output.
#[derive(Debug, Deserialize, Configure)]
pub struct Config {
    /// Path to the `Cargo.toml` file for the crate being analyzed
    manifest_path: PathBuf,

    /// Path to place rustdoc output
    output_path: Option<PathBuf>,

    /// Contains the Cargo analysis output for the crate being documented
    #[serde(deserialize_with = "deserialize_host")]
    host: analysis::AnalysisHost,

    base_url: String,

    #[serde(deserialize_with = "deserialize_handlebars")]
    handlebars: Handlebars,
}

fn deserialize_host<'de, D>(_: D) -> ::std::result::Result<analysis::AnalysisHost, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(analysis::AnalysisHost::new(analysis::Target::Debug))
}

fn deserialize_handlebars<'de, D>(_: D) -> ::std::result::Result<Handlebars, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(default_handlebars())
}

fn default_handlebars() -> Handlebars {
    let mut handlebars = Handlebars::new();

    macro_rules! template {
        ($handlebars:ident, $name:expr, $path:expr) => {
            $handlebars
                .register_template_file($name, concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
                .expect(concat!("Failed to register template ", $name, " (", $path, ")"));
        };
    }

    template!(handlebars, "example", "templates/markdown/example.hbs");
    template!(handlebars, "page", "templates/html/page.hbs");
    template!(handlebars, "api", "templates/markdown/api.hbs");
    template!(handlebars, "mod", "templates/markdown/mod.hbs");
    template!(handlebars, "struct", "templates/markdown/struct.hbs");
    template!(handlebars, "enum", "templates/markdown/enum.hbs");
    template!(handlebars, "trait", "templates/markdown/trait.hbs");
    template!(handlebars, "function", "templates/markdown/function.hbs");
    template!(handlebars, "type", "templates/markdown/type.hbs");
    template!(handlebars, "static", "templates/markdown/static.hbs");
    template!(handlebars, "const", "templates/markdown/const.hbs");

    handlebars.register_helper(
        "up-dir",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             rc: &mut handlebars::RenderContext|
             -> handlebars::HelperResult {
                let count = h.param(0).map(|v| v.value().as_u64().unwrap()).unwrap();

                for _ in 0..count {
                    rc.writer.write_all(b"../")?;
                }

                Ok(())
            },
        ),
    );

    handlebars
}

impl Config {
    pub fn new(manifest_path: PathBuf) -> Self {
        let host = analysis::AnalysisHost::new(analysis::Target::Debug);

        let config_path = PathBuf::from("Doxidize.toml");
        let mut contents = String::new();

        let base_url = (|| {
            let mut toml_file = File::open(config_path)?;
            toml_file.read_to_string(&mut contents)?;
            let doc = contents.parse::<toml_edit::Document>()?;

            Ok((|| {
                let value = doc["docs"]["base-url"].as_value()?;
                let value = value.as_str()?;
                Some(value.to_string())
            })()
                .ok_or("")?)
        })()
            .unwrap_or_else(|_: Box<::std::error::Error>| String::from(""));

        let handlebars = default_handlebars();

        Config {
            manifest_path,
            host,
            output_path: None,
            base_url,
            handlebars,
        }
    }

    /// Returns the directory containing the `Cargo.toml` of the crate being documented.
    pub fn root_path(&self) -> &Path {
        // unwrap() is okay, as manifest_path will point to a file
        self.manifest_path.parent().unwrap()
    }

    /// Returns the directory where output files should be placed
    pub fn output_path(&self) -> PathBuf {
        match self.output_path {
            Some(ref path) => path.clone(),
            None => self.root_path().join("target").join("docs"),
        }
    }

    pub fn config_path(&self) -> PathBuf {
        self.root_path().join("Doxidize.toml")
    }

    pub fn menu_path(&self) -> PathBuf {
        self.markdown_path().join("Menu.toml")
    }

    /// Set the directory where output files should be placed
    pub fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = Some(output_path)
    }

    pub fn public_path(&self) -> PathBuf {
        self.output_path().join("public")
    }

    pub fn readme_path(&self) -> PathBuf {
        self.markdown_path().join("README.md")
    }

    pub fn rls_target_path(&self) -> PathBuf {
        self.root_path().join("target").join("rls")
    }

    pub fn examples_path(&self) -> PathBuf {
        self.root_path().join("examples")
    }

    pub fn examples_markdown_path(&self) -> PathBuf {
        self.markdown_path().join("examples")
    }

    pub fn markdown_path(&self) -> PathBuf {
        self.root_path().join("docs")
    }

    pub fn api_markdown_path(&self) -> PathBuf {
        self.markdown_path().join("api")
    }

    pub fn api_readme_path(&self) -> PathBuf {
        self.api_markdown_path().join("README.md")
    }

    pub fn api_module_overview_path(&self) -> PathBuf {
        self.api_markdown_path().join("module-overview.md")
    }

    pub fn api_struct_overview_path(&self) -> PathBuf {
        self.api_markdown_path().join("struct-overview.md")
    }

    pub fn api_trait_overview_path(&self) -> PathBuf {
        self.api_markdown_path().join("trait-overview.md")
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn set_manifest_path(&mut self, path: PathBuf) {
        self.manifest_path = path;
    }

    pub fn host(&self) -> &analysis::AnalysisHost {
        &self.host
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn handlebars(&self) -> &Handlebars {
        &self.handlebars
    }
}
