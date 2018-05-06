extern crate doxidize;
extern crate doxidize_test;

#[test]
fn creates_docs_dir() {
    let p = doxidize_test::project().build();

    doxidize::ops::init(&p.config, &p.log).expect("init failed");

    assert!(p.dir().join("docs").is_dir());
}

#[test]
fn creates_root_readme() {
    let p = doxidize_test::project().build();

    doxidize::ops::init(&p.config, &p.log).expect("init failed");

    let docs_dir = p.dir().join("docs");
    let readme_path = docs_dir.join("README.md");

    assert!(readme_path.is_file());
}

#[test]
fn creates_doxidize_config() {
    let p = doxidize_test::project().build();

    doxidize::ops::init(&p.config, &p.log).expect("init failed");

    let config_path = p.dir().join("Doxidize.toml");

    assert!(config_path.is_file());
}

// TODO: make this test pass
//
// #[test]
// fn double_initialize() {
//     let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");
//     let log = util::make_logger();
//
//     let dir_path = dir.path();
//
//     util::cargo_init(dir_path).expect("Could not create sample crate");
//
//     let mut config = Config::default();
//     config.set_manifest_path(dir_path.join("Cargo.toml"));
//
//     doxidize::ops::init(&config, &log).expect("init failed");
//
//     doxidize::ops::init(&config, &log).expect("init failed when run a second time");
// }

#[test]
fn creates_menu_toml() {
    let p = doxidize_test::project().build();

    doxidize::ops::init(&p.config, &p.log).expect("init failed");

    let docs_dir = p.dir().join("docs");
    let readme_path = docs_dir.join("Menu.toml");

    assert!(readme_path.is_file());
}
