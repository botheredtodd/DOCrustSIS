mod docsis;
mod TLV;
mod d4;
mod MIB;

use std::{env, fmt};
use getopts::Options;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use directories::UserDirs;

use crate::d4::{d4_defs, DATATYPE};
// use crate::MIB::MIB;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}



fn main() {
    let filename = format!("{}/.mibs.json", UserDirs::new().unwrap().home_dir().to_str().unwrap());
    println!("Loading MIBs from: {}", filename);
    let miblist = MIB::MIBList::from_file(filename.as_str());
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
    let mut tlv_list = TLV::TLVList { tlvs: Vec::new() };
    let d4d = d4_defs();
    println!("File: {}", filename);
    let mut f = File::open(filename).unwrap();

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer).unwrap();
    let mut i = 0;
    while i < buffer.len() {
        let t = buffer[i];
        let l = buffer[i + 1];
        let mut v = Vec::new();
        let mut sub :Vec<TLV::TLV> = Vec::new();
        for j in 0..l {
            v.push(buffer[i + 2 + j as usize]);
        }
        let tlv = TLV::TLV { t, l, v, sub_tlvs: sub};
        tlv_list.tlvs.push(tlv);
        i += 2 + l as usize;
    }
    for tlv in tlv_list.tlvs {
        let this_d4 = docsis::decode(tlv, d4d.clone());
        if this_d4.is_ok() {
            let this_d4_unwrapped = this_d4.unwrap();
            match this_d4_unwrapped.dataType {
                d4::DATATYPE::UCHAR => {
                    println!("TLV: {}: {}", this_d4_unwrapped.t, this_d4_unwrapped.description);
                    println!("Decoded: {}", this_d4_unwrapped.get_int_value().unwrap());
                },
                d4::DATATYPE::UINT => {
                    println!("TLV: {}: {}", this_d4_unwrapped.t, this_d4_unwrapped.description);
                    println!("Decoded: {}", this_d4_unwrapped.get_int_value().unwrap());
                },
                d4::DATATYPE::STRING => {
                    println!("TLV: {}: {}", this_d4_unwrapped.t, this_d4_unwrapped.description);
                    println!("Decoded: {}", this_d4_unwrapped.get_string_value().unwrap());
                }
                d4::DATATYPE::MIB => {
                    println!("TLV: {}: {}", this_d4_unwrapped.t, this_d4_unwrapped.description);
                    // println!("Decoded: {}", this_d4_unwrapped.get_mib_value().unwrap());
                    let mb = MIB::MIB::from_bytes(this_d4_unwrapped.tlv.v.clone());
                    let mbname = miblist.get_mib(mb.oid.as_str());
                    if mbname.is_some() {
                        println!("MIB: {} ({}): {:?}", mb.oid, mbname.unwrap().name, mb.value);
                    }
                    else {
                        println!("MIB: {} ({}): {:?}", mb.oid, "Unknown MIB", mb.value);

                    }
                }
                d4::DATATYPE::AGGREGATE => {
                    println!("TLV: {}: {}", this_d4_unwrapped.t, this_d4_unwrapped.description);
                    for (tag, stlv) in this_d4_unwrapped.sub_tlvs {
                    if stlv.tlv.v != vec![] {
                        println!("\tSub-TLV: {}: {} ({:?})", stlv.t, stlv.description, stlv.tlv.v);
                        match stlv.dataType {
                            DATATYPE::UCHAR => {
                                println!("\tDecoded: {}", stlv.get_int_value().unwrap());
                            },
                            DATATYPE::UINT => {
                                println!("\tDecoded: {}", stlv.get_int_value().unwrap());
                            },
                            DATATYPE::USHORT => {
                                println!("\tDecoded: {}", stlv.get_int_value().unwrap());
                            },
                            DATATYPE::STRING => {
                                println!("\tDecoded: {}", stlv.get_string_value().unwrap());
                            }
                            DATATYPE::STRINGZERO => {
                                println!("\tDecoded: {}", stlv.get_string_value().unwrap());
                            }
                            //_ => {}
                            DATATYPE::AGGREGATE => {
                                println!("Aggregate");
                            }
                            DATATYPE::MIB => {
                                println!("MIB");
                            }
                        }
                    }

                    }
                }
                _ => {}
            }
        }
    }
}
