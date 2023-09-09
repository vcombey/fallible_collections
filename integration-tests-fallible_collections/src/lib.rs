//! This crate contains integration tests for fallible_collections.

mod boilerplate_structure;
mod testable;

pub use boilerplate_structure::{Attribute, IntTest};
pub use testable::Testable;

// use std::ffi::OsStr;

/// Test for `cargo build` command.
// #[derive(Debug)]
// pub struct CargoBuilder;

// YOU CAN IMPLEMENT THE `Testable` TRAIT TO CREATE A NEW TEST ...
// #[cfg_attr(rustfmt, rustfmt_skip)]
// impl Testable for CargoBuilder {
//     fn command(&self) -> &OsStr { "cargo".as_ref() }
//     fn name(&self) -> &str { "Build program" }
//     fn args(&self) -> Vec<&OsStr> { vec!["build".as_ref()] }
//     fn terminate(&self) -> bool { true }
//     fn show_output(&self) -> Option<bool> { Some(true) }
//     fn output(&mut self, status: i32, _out: &[u8], _err: &[u8]) -> bool { status == 0 }
// }

/// This main function will be exported.
#[no_mangle]
#[allow(non_snake_case)]
pub fn TESTS(_args: &[&str]) -> Result<Vec<Box<dyn Testable>>, Box<dyn std::error::Error>> {
    // ... OR YOU CAN USE THE `IntTest` STRUCTURE, WHICH IS A VERY HANDY BOILERPLATE.
    let attr = Attribute {
        name: "dummy name test".to_string(),
        terminate: true,
        show_output: Some(true),
        ..Default::default()
    };
    let ok_cargo =
        IntTest::builder("cargo").output(|status: i32, _out: &[u8], _err: &[u8]| status == 0);

    let mut output: Vec<Box<dyn Testable>> = Vec::new();
    output.push(Box::new(
        ok_cargo
            .clone()
            .args(["+stable", "test", "--features=default"])
            .set_attribute(attr.clone().name("dummy stable test")),
    ));
    output.push(Box::new(
        ok_cargo
            .clone()
            .args(["+stable", "build", "--features=default"])
            .set_attribute(attr.clone().name("dummy stable build")),
    ));

    let features_list = [
        "--no-default-features",
        "--features=std",
        "--features=std_io",
        // "--features=hashmap", // Default feature
        "--features=hashmap",
        "--features=hashmap,std",
        "--features=hashmap,std_io",
    ];
    output.extend(features_list.iter().map(|feature| {
        Box::new(
            ok_cargo
                .clone()
                .args(["+stable", "test", feature])
                .set_attribute(attr.clone().name(format!("stable test: {}", feature))),
        ) as Box<dyn Testable>
    }));
    output.extend(features_list.iter().map(|feature| {
        Box::new(
            ok_cargo
                .clone()
                .args(["+stable", "build", feature])
                .set_attribute(attr.clone().name(format!("stable build: {}", feature))),
        ) as Box<dyn Testable>
    }));

    output.push(Box::new(
        ok_cargo
            .clone()
            .args(["+nightly", "test", "--features=unstable"])
            .set_attribute(attr.clone().name("dummy unstable test")),
    ));
    output.push(Box::new(
        ok_cargo
            .clone()
            .args(["+nightly", "build", "--features=unstable"])
            .set_attribute(attr.clone().name("dummy unstable build")),
    ));

    let features_list = [
        "--features=unstable,std",
        "--features=unstable,std_io",
    ];
    output.extend(features_list.iter().map(|feature| {
        Box::new(
            ok_cargo
                .clone()
                .args(["+nightly", "test", feature])
                .set_attribute(attr.clone().name(format!("unstable test: {}", feature))),
        ) as Box<dyn Testable>
    }));
    output.extend(features_list.iter().map(|feature| {
        Box::new(
            ok_cargo
                .clone()
                .args(["+nightly", "build", feature])
                .set_attribute(attr.clone().name(format!("unstable build: {}", feature))),
        ) as Box<dyn Testable>
    }));

    Ok(output)
}
