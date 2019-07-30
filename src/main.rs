#![feature(core_intrinsics)]
#![feature(type_ascription)]
#![feature(const_fn)]

extern crate flame;
#[macro_use] extern crate flamer;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate a3mo_lib;
use a3mo_lib::repository::new;
use a3mo_lib::repository::build;
use std::fs;

fn main() {
/*
    match new::new("TestModPack", "D:\\___Arma3Sync\\test_mod_pack", "Url",  true) {
        Ok(v) => println!("ok {:?}", v),
        Err(e) => println!("err {:?}", e),
    };
*/

    match build::build("TestModPack") {
        Ok(v) => {
            println!("ok {:?}", v);
            fs::write("build.json", v).expect("Unable to write to file :(")
        }
        Err(e) => println!("err {:?}", e),
    }


}
