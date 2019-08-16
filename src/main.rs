#![feature(core_intrinsics)]
#![feature(type_ascription)]
#![feature(const_fn)]

//extern crate flame;
//#[macro_use] extern crate flamer;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

extern crate a3mo_lib;
use a3mo_lib::repository::{new, run};
use a3mo_lib::repository::build;
use std::fs;
use a3mo_lib::repository::clone;
use std::fs::{File, remove_dir_all, remove_file};

fn main() {
    if !is_elevated::is_elevated(){
        println!("Restart as admin !");
        return;
    }

    let repo_path = "C:\\Users\\Scarjit\\Arma3Mods\\TEST_IN";
    let data_path = "C:\\Users\\Scarjit\\Arma3Mods\\TEST_OUT";
    let raw_repo_url = "http://127.0.0.1:8000/";
    let repo_url = "http://127.0.0.1:8000/.a3mo/";
    let repo_name = "TestModPack";
    let arma_path = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Arma 3\\arma3_x64.exe";
    let opt_args: Option<Vec<String>> = Some(vec!["-malloc=jemalloc_bi_x64".to_owned(), "-showScriptErrors".to_owned(), "-enableHT".to_owned(), "-hugepages".to_owned(), "-nosplash".to_owned(), "-world=empty".to_owned()]);
    let tmp_folder = "C:\\Users\\Scarjit\\AppData\\Roaming\\A3MOTMP";

    //remove_file("a3mm.sqlite3");

/*
    match new::new(repo_name, repo_path, raw_repo_url) {
        Ok(v) => println!("ok {:?}", v),
        Err(e) => println!("err {:?}", e),
    };

    match build::build(repo_name, true, true) {
        Ok(v) => {
            println!("ok {:?}", v);
        }
        Err(e) => println!("err {:?}", e),
    }

    match clone::clone(data_path, repo_url, repo_name) {
        Ok(v) => {
            println!("ok {:?}", v);
        }
        Err(e) => println!("err {:?}", e),
    }

*/

    match run::run(repo_name, arma_path, tmp_folder, opt_args) {
        Ok(v) => {
            println!("ok {:?}", v);
        }
        Err(e) => println!("err {:?}", e),
    }


}
