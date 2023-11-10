use crate::subr;
use crate::uuid;
use crate::Result;
use crate::UserData;
use serde::Deserialize;
use std::io::Read;
use std::io::Seek;

const UNIT_SIZE: usize = 512;

#[repr(C)] // should be packed
#[derive(Debug, Default, Deserialize)]
pub struct GptHdr {
    pub hdr_sig: [u8; 8],
    pub hdr_revision: u32,
    pub hdr_size: u32,
    pub hdr_crc_self: u32,
    pub reserved: u32,
    pub hdr_lba_self: u64,
    pub hdr_lba_alt: u64,
    pub hdr_lba_start: u64,
    pub hdr_lba_end: u64,
    pub hdr_uuid: uuid::Uuid,
    pub hdr_lba_table: u64,
    pub hdr_entries: u32,
    pub hdr_entsz: u32,
    pub hdr_crc_table: u32,
    pub padding: u32,
}

#[repr(C)] // should be packed
#[derive(PartialEq, Debug, Default, Deserialize)]
pub struct GptEnt {
    pub ent_type: uuid::Uuid,
    pub ent_uuid: uuid::Uuid,
    pub ent_lba_start: u64,
    pub ent_lba_end: u64,
    pub ent_attr: u64,

    // XXX Arrays of sizes from 0 to 32 (inclusive) implement the Default trait.
    pub ent_name1: [u16; 32],
    pub ent_name2: [u16; 4],
}

fn try_known_uuid_to_str(uuid: &uuid::Uuid, dat: &UserData) -> String {
    if dat.opt.symbol {
        let s = subr::known_uuid_to_str(uuid);
        if !s.is_empty() {
            return s.to_string();
        }
    }

    subr::uuid_to_str(uuid)
}

fn dump_header(fp: &mut std::fs::File, hdr_lba: u64, dat: &UserData) -> Result<GptHdr> {
    let hdr_offset = hdr_lba * UNIT_SIZE as u64;
    fp.seek(std::io::SeekFrom::Start(hdr_offset))?;

    let mut buf = vec![0; std::mem::size_of::<GptHdr>()];
    fp.read_exact(&mut buf)?;

    // XXX Explicitly set little endian.
    // https://users.rust-lang.org/t/change-endianness-in-bincode-serde/23063
    let hdr: GptHdr = bincode::deserialize(&buf)?;

    println!(
        "sig      = \"{}{}{}{}{}{}{}{}\"",
        hdr.hdr_sig[0] as char,
        hdr.hdr_sig[1] as char,
        hdr.hdr_sig[2] as char,
        hdr.hdr_sig[3] as char,
        hdr.hdr_sig[4] as char,
        hdr.hdr_sig[5] as char,
        hdr.hdr_sig[6] as char,
        hdr.hdr_sig[7] as char,
    );

    let p = hdr.hdr_revision.to_le_bytes();
    println!(
        "revision = {:02x} {:02x} {:02x} {:02x}",
        p[0], p[1], p[2], p[3]
    );

    println!("size     = {}", hdr.hdr_size);
    println!("crc_self = 0x{:x}", hdr.hdr_crc_self);
    println!("lba_self = 0x{:016x}", hdr.hdr_lba_self);
    println!("lba_alt  = 0x{:016x}", hdr.hdr_lba_alt);
    println!("lba_start= 0x{:016x}", hdr.hdr_lba_start);
    println!("lba_end  = 0x{:016x}", hdr.hdr_lba_end);

    println!("uuid     = {}", try_known_uuid_to_str(&hdr.hdr_uuid, dat));

    println!("lba_table= 0x{:016x}", hdr.hdr_lba_table);
    println!("entries  = {}", hdr.hdr_entries);
    println!("entsz    = {}", hdr.hdr_entsz);
    println!("crc_table= 0x{:x}", hdr.hdr_crc_table);

    // XXX
    if hdr.hdr_entries > 512 {
        return Err(Box::new(std::io::Error::from(
            std::io::ErrorKind::InvalidData,
        )));
    }

    Ok(hdr)
}

fn dump_entries(fp: &mut std::fs::File, hdr: &GptHdr, dat: &UserData) -> Result<()> {
    let lba_table_size = hdr.hdr_entsz * hdr.hdr_entries;
    let lba_table_sectors = lba_table_size / UNIT_SIZE as u32;
    let mut total = 0;

    println!(
        "{:<3} {:<36} {:<36} {:<16} {:<16} {:<16} name",
        "#", "type", "uniq", "lba_start", "lba_end", "attr"
    );

    for i in 0..lba_table_sectors {
        let offset = (hdr.hdr_lba_table + i as u64) * UNIT_SIZE as u64;
        let sector_entries = UNIT_SIZE as u32 / hdr.hdr_entsz;
        let mut entry_offset = 0;

        for j in 0..sector_entries {
            fp.seek(std::io::SeekFrom::Start(offset + entry_offset))?;

            let mut buf = vec![0; std::mem::size_of::<GptEnt>()];
            fp.read_exact(&mut buf)?;

            // XXX Explicitly set little endian.
            // https://users.rust-lang.org/t/change-endianness-in-bincode-serde/23063
            let p: GptEnt = bincode::deserialize(&buf)?;

            entry_offset += std::mem::size_of::<GptEnt>() as u64;
            let empty = GptEnt {
                ..Default::default()
            };
            if !dat.opt.verbose && p == empty {
                total += 1;
                continue;
            }

            let mut name = [0u8; 36];
            let mut nlen = 0;
            assert!(name.len() == 36);
            assert!(p.ent_name1.len() + p.ent_name2.len() == name.len());
            for (k, v) in name.iter_mut().enumerate() {
                // XXX ascii
                *v = if k < p.ent_name1.len() {
                    p.ent_name1[k] & 0xFF
                } else {
                    p.ent_name2[k - p.ent_name1.len()] & 0xFF
                } as u8;
                if *v == 0 {
                    nlen = k;
                    break;
                }
            }

            println!(
                "{:<3} {:<36} {:<36} {:<016x} {:<016x} {:<016x} {}",
                i * sector_entries + j,
                try_known_uuid_to_str(&p.ent_type, dat),
                try_known_uuid_to_str(&p.ent_uuid, dat),
                p.ent_lba_start,
                p.ent_lba_end,
                p.ent_attr,
                std::str::from_utf8(&name[..nlen])?
            );
            total += 1;
        }
    }
    assert!(total == hdr.hdr_entries);

    Ok(())
}

pub fn dump_gpt(fp: &mut std::fs::File, dat: &UserData) -> Result<()> {
    let mut hdr2 = GptHdr {
        ..Default::default()
    };

    // primary header
    println!("primary header");
    let hdr1: GptHdr = dump_header(fp, 1, dat)?;

    // secondary header
    if !dat.opt.noalt {
        println!();
        println!("secondary header");
        hdr2 = dump_header(fp, hdr1.hdr_lba_alt, dat)?;
    }

    // primary entries
    println!();
    println!("primary entries");
    dump_entries(fp, &hdr1, dat)?;

    // secondary entries
    if !dat.opt.noalt {
        println!();
        println!("secondary entries");
        dump_entries(fp, &hdr2, dat)?;
    }

    Ok(())
}
