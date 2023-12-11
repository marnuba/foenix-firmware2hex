use std::io::{BufReader, BufRead, Read, Write, BufWriter};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct BulkEntry {
    page: u8,
    filename: String,
}

impl BulkEntry {
    pub fn from_line(line: String) -> Self {
        let p: Vec<_> = line.split(",").collect();

        if p.len() != 2 {
            panic!("could no parse bulk.csv");
        }

        let page: u8 = u8::from_str_radix(p[0], 16).expect("could no parse hex");
        let filename = String::from(p[1]);

        Self { page, filename}
    }
}

fn read_bulk_csv(basedir: &Path) -> Vec<BulkEntry> {
    let filepath = basedir.join("bulk.csv");
    let file = File::open(filepath).expect("could not open bulk.csv");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| BulkEntry::from_line(line.expect("no line")))
        .collect()
}

fn main() {

    let base_dir = Path::new(".");
    let entries = read_bulk_csv(base_dir);
    let outpath = base_dir.join("kernel_F256jr.hex");
    let mut outfile = BufWriter::new(File::create(outpath).expect("could not create output file"));

    let flash_base: u32 = 0x80000;

    let mut outbase = 0x0000;
    let mut lower;

    for entry in entries {
        if entry.filename != "lockout.bin" {

            let base: u32 = flash_base + 0x2000u32 * (entry.page as u32);

            let upper = base >> 16;
            lower = base & 0xffff;

            if upper != outbase {
                
                let hi = (upper >> 8) as u8;
                let lo = upper as u8;

                let mut chksum = !(0x02u8
                    .wrapping_add(0x04)
                    .wrapping_add(hi)
                    .wrapping_add(lo));
                chksum = chksum.wrapping_add(1);
                writeln!(outfile, ":02000004{:02X}{:02X}{:02X}", hi, lo, chksum)
                    .expect("could not write to output file");
                outbase = upper;
            }

            let binfile = File::open(base_dir.join(&entry.filename))
                .expect(format!("could not open file {}", &entry.filename).as_str());
            let mut binfilereader = BufReader::new(binfile);            
            let mut buffer =[0u8;0x20];
            
            let mut size = binfilereader.read(&mut buffer).expect("error reading");

            while size > 0 {

                let mut chksum = (size as u8).wrapping_add((lower >> 8) as u8).wrapping_add((lower & 0xff) as u8);
                write!(outfile, ":{:02X}{:04X}00", size, lower).expect("could not write to output file"); // type 00
                for i in 0..size {
                    write!(outfile, "{:02X}", buffer[i])
                    .expect("could not write to output file");
                    chksum = chksum.wrapping_add(buffer[i]);
                }
                chksum = (!chksum).wrapping_add(1);   // 2 complement
                writeln!(outfile, "{:02X}", chksum)
                .expect("could not write to output file");

                lower += size as u32;
                size = binfilereader.read(&mut buffer).expect("error reading");
            }
        }
    };
}
