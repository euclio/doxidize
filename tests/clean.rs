extern crate doxidize;
extern crate doxidize_test;

#[test]
fn clean_deletes_directory_in_target() {
    let p = doxidize_test::project().build();

    doxidize::ops::init(&p.config, &p.log).expect("init failed");
    doxidize::ops::build(&p.config, &p.log).expect("build failed");

    let target_docs_dir = p.dir().join("target").join("docs");
    assert!(
        target_docs_dir.is_dir(),
        format!("{} is not a directory", target_docs_dir.display())
    );

    doxidize::ops::clean(&p.config, &p.log).expect("clean failed");

    assert!(!target_docs_dir.is_dir());
}
