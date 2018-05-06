extern crate doxidize;
extern crate doxidize_test;

use std::fs;

#[test]
fn build_renders_readme() {
    let p = doxidize_test::project().initialized(true).build();

    doxidize::ops::build(&p.config, &p.log).expect("build failed");

    let output_dir = p.dir().join("target").join("docs");
    let rendered_readme_contents = fs::read_to_string(output_dir.join("index.html")).unwrap();

    assert!(rendered_readme_contents.contains("<h1>Overview</h1>"));
}

#[test]
fn build_renders_additional_markdown_files() {
    let p = doxidize_test::project()
        .initialized(true)
        .with_file(
            "docs/guide.md",
            r#"---
id = "guide"
title = "Testing"
---
# Testing

testing"#,
        )
        .build();

    doxidize::ops::build(&p.config, &p.log).expect("generate failed");

    let output_dir = p.dir().join("target").join("docs");
    let guide_contents = fs::read_to_string(output_dir.join("guide.html")).unwrap();

    assert!(guide_contents.contains(
        "<h1>Testing</h1>
<p>testing</p>"
    ));
}

#[test]
fn build_renders_nested_directories() {
    let p = doxidize_test::project()
        .initialized(true)
        .with_file(
            "docs/nested/guide.md",
            r#"---
id = "guide"
title = "Testing"
---
# Testing

testing"#,
        )
        .build();

    doxidize::ops::build(&p.config, &p.log).expect("build failed");

    let output_dir = p.dir().join("target").join("docs");

    let rendered_guide_contents =
        fs::read_to_string(output_dir.join("nested").join("guide.html")).unwrap();

    assert!(rendered_guide_contents.contains(
        "<h1>Testing</h1>
<p>testing</p>"
    ));
}
