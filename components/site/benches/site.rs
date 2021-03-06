#![feature(test)]
extern crate test;
extern crate site;
extern crate pagination;
extern crate tempfile;

use std::env;

use tempfile::tempdir;
use site::Site;
use pagination::Paginator;


fn setup_site(name: &str) -> Site {
    let mut path = env::current_dir().unwrap().to_path_buf();
    path.push("benches");
    path.push(name);
    let mut site = Site::new(&path, "config.toml").unwrap();
    site.load().unwrap();
    site
}

#[bench]
fn bench_render_aliases(b: &mut test::Bencher) {
    let mut site = setup_site("small-blog");
    let tmp_dir = tempdir().expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    b.iter(|| site.render_aliases().unwrap());
}

#[bench]
fn bench_render_sitemap(b: &mut test::Bencher) {
    let mut site = setup_site("small-blog");
    let tmp_dir = tempdir().expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    b.iter(|| site.render_sitemap().unwrap());
}

#[bench]
fn bench_render_rss_feed(b: &mut test::Bencher) {
    let mut site = setup_site("small-blog");
    let tmp_dir = tempdir().expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    b.iter(|| site.render_rss_feed(None, None).unwrap());
}

#[bench]
fn bench_render_taxonomies(b: &mut test::Bencher) {
    let mut site = setup_site("small-blog");
    let tmp_dir = tempdir().expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    b.iter(|| site.render_taxonomies().unwrap());
}

#[bench]
fn bench_render_paginated(b: &mut test::Bencher) {
    let mut site = setup_site("small-blog");
    let tmp_dir = tempdir().expect("create temp dir");
    let public = &tmp_dir.path().join("public");
    site.set_output_path(&public);
    let section = site.sections.values().collect::<Vec<_>>()[0];
    let paginator = Paginator::from_section(&section.pages, section);

    b.iter(|| site.render_paginated(public, &paginator));
}
