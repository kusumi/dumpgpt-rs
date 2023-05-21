use crate::subr;
use crate::uuid;
use crate::UserData;
use std::io::Seek;

const UNIT_SIZE: usize = 512;

#[repr(C)] // should be packed
#[derive(Debug, Default)]
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
#[derive(PartialEq, Debug, Default)]
pub struct GptEnt {
    pub ent_type: uuid::Uuid,
    pub ent_uuid: uuid::Uuid,
    pub ent_lba_start: u64,
    pub ent_lba_end: u64,
    pub ent_attr: u64,

    // XXX "error[E0277]: the trait bound `[u16; 36]: Default` is not satisfied" why ?
    //pub ent_name: [u16; 36],
    pub ent_name1: [u16; 18],
    pub ent_name2: [u16; 18],
}

fn try_known_uuid_to_str(uuid: &uuid::Uuid, dat: &UserData) -> String {
    if dat.opt.symbol {
        let s = subr::known_uuid_to_str(uuid);
        if !s.is_empty() {
            return s;
        }
    }

    subr::uuid_to_str(uuid)
}

fn dump_header(fp: &mut std::fs::File, hdr_lba: u64, dat: &UserData) -> std::io::Result<GptHdr> {
    let hdr_offset = hdr_lba * UNIT_SIZE as u64;
    fp.seek(std::io::SeekFrom::Start(hdr_offset))?;

    let mut hdr = GptHdr {
        ..Default::default()
    };
    // why can't Rust easily read(2) + handle blob ?
    for i in 0..8 {
        hdr.hdr_sig[i] = subr::read_u8(fp)?;
    }
    hdr.hdr_revision = subr::read_le32(fp)?;
    hdr.hdr_size = subr::read_le32(fp)?;
    hdr.hdr_crc_self = subr::read_le32(fp)?;
    hdr.reserved = subr::read_le32(fp)?;
    hdr.hdr_lba_self = subr::read_le64(fp)?;
    hdr.hdr_lba_alt = subr::read_le64(fp)?;
    hdr.hdr_lba_start = subr::read_le64(fp)?;
    hdr.hdr_lba_end = subr::read_le64(fp)?;
    hdr.hdr_uuid = uuid::read_uuid_le(fp)?;
    hdr.hdr_lba_table = subr::read_le64(fp)?;
    hdr.hdr_entries = subr::read_le32(fp)?;
    hdr.hdr_entsz = subr::read_le32(fp)?;
    hdr.hdr_crc_table = subr::read_le32(fp)?;
    hdr.padding = subr::read_le32(fp)?;

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
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    Ok(hdr)
}

fn dump_entries(fp: &mut std::fs::File, hdr: &GptHdr, dat: &UserData) -> std::io::Result<()> {
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

            let mut p = GptEnt {
                ..Default::default()
            };
            // why can't Rust easily read(2) + handle blob ?
            p.ent_type = uuid::read_uuid_le(fp)?;
            p.ent_uuid = uuid::read_uuid_le(fp)?;
            p.ent_lba_start = subr::read_le64(fp)?;
            p.ent_lba_end = subr::read_le64(fp)?;
            p.ent_attr = subr::read_le64(fp)?;
            for k in 0..p.ent_name1.len() {
                p.ent_name1[k] = subr::read_le16(fp)?;
            }
            for k in 0..p.ent_name2.len() {
                p.ent_name2[k] = subr::read_le16(fp)?;
            }

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
            for k in 0..name.len() {
                // XXX ascii
                name[k] = if k < name.len() / 2 {
                    p.ent_name1[k] & 0xFF
                } else {
                    p.ent_name2[k - name.len() / 2] & 0xFF
                } as u8;
                if name[k] == 0 {
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
                std::str::from_utf8(&name[..nlen]).unwrap()
            );
            total += 1;
        }
    }
    assert!(total == hdr.hdr_entries);

    Ok(())
}

pub fn dump_gpt(fp: &mut std::fs::File, dat: &UserData) -> std::io::Result<()> {
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
