mod docsis;
mod tlv;
mod d4;
mod mib;
mod flows;
mod drop_classifiers;
mod vendor_specific;
mod erouter;
mod packet_classifiers;

use std::{env};
use getopts::Options;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
// use std::io;
use directories::UserDirs;
use serde::Serialize;
use serde_json::{Result, to_string};

use crate::d4::{d4_defs, DATATYPE};
// use crate::MIB::MIB;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}



fn main() {
    let filename = format!("{}/.mibs.json", UserDirs::new().unwrap().home_dir().to_str().unwrap());
    let mut miblist = mib::MIBList::new();
    if Path::new(filename.as_str()).exists() {
        println!("Loading MIBs from: {}", filename);
        miblist = mib::MIBList::from_file(filename.as_str());
    }

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "A DOCSIS, MTA, or STB config file", "Config File");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let filename = if matches.opt_present("f")
    {
        matches.opt_str("f").unwrap().clone()
    } else {
        println!("You must provide a file to parse");
        std::process::exit(1);
    };
    let mut tlv_list = tlv::TLVList { tlvs: Vec::new() };
    let d4d = d4_defs();
    println!("File: {}", filename);
    let mut f = File::open(&filename).unwrap();

    let mut buffer = Vec::new();
    let mut out_buffer :Vec<u8> = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer).unwrap();
    let mut i = 0;
    while i < buffer.len() {
        let t = buffer[i];
        let l = buffer[i + 1];
        let mut v = Vec::new();
        let mut sub :Vec<tlv::TLV> = Vec::new();
        for j in 0..l {
            v.push(buffer[i + 2 + j as usize]);
        }
        let tlv = tlv::TLV { t, l, v, sub_tlvs: sub};
        tlv_list.tlvs.push(tlv);
        i += 2 + l as usize;
    }
    for tlv in tlv_list.tlvs {
        let this_d4 = docsis::decode(tlv.clone(), d4d.clone());
        if this_d4.is_ok() {
            let mut this_d4_unwrapped = this_d4.unwrap();
            let s = to_string(&this_d4_unwrapped);
            if s.is_ok() {
                println!("{}", s.unwrap());
            }
            else {
                println!("Error serializing TLV: {}", tlv.t);
            }
            // println!("{}", this_d4_unwrapped);
            // if this_d4_unwrapped.t == 0x12 {
            //     println!("{}", this_d4_unwrapped);
            //     let old_value = this_d4_unwrapped.get_int_value().unwrap();
            //     this_d4_unwrapped.set_uint_value(old_value * 4u32).expect("TODO: panic message");
            //     println!("{}", this_d4_unwrapped);
            // }
            // out_buffer.push(this_d4_unwrapped.tlv.t);
            // out_buffer.push(this_d4_unwrapped.tlv.l);
            // for b in this_d4_unwrapped.tlv.v {
            //     out_buffer.push(b);
            // }
        }
        else {
            println!("Error decoding TLV: {}", tlv.t);
        }
    }

    // save a modified file
    // let mut out = File::create(format!("{}-new", &filename)).unwrap();
    // out.write_all(&out_buffer).unwrap();

}
