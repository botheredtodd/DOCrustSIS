use std::{env, fmt};
use getopts::Options;
use std::fs::File;
use std::io::prelude::*;
use std::io;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

struct TLV {
    t: u8,
    l: u8,
    v: Vec<u8>,
}

impl fmt::Display for TLV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type: {}, Length: {}, Value: {:?}", self.t, self.l, self.v)
    }
}
struct TLVList {
    tlvs: Vec<TLV>,
}

fn main() {
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
    let mut tlv_list = TLVList { tlvs: Vec::new() };
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
        for j in 0..l {
            v.push(buffer[i + 2 + j as usize]);
        }
        let tlv = TLV { t, l, v };
        tlv_list.tlvs.push(tlv);
        i += 2 + l as usize;
    }
    for tlv in tlv_list.tlvs {
        println!("{}", tlv);
    }
}
