use crate::subr;
use crate::uuid;
use crate::Opt;
use crate::Result;
use std::io::Read;
use std::io::Seek;

const UNIT_SIZE: usize = 512;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct GptHdr {
    pub(crate) hdr_sig: [u8; 8],
    pub(crate) hdr_revision: u32,
    pub(crate) hdr_size: u32,
    pub(crate) hdr_crc_self: u32,
    pub(crate) reserved: u32,
    pub(crate) hdr_lba_self: u64,
    pub(crate) hdr_lba_alt: u64,
    pub(crate) hdr_lba_start: u64,
    pub(crate) hdr_lba_end: u64,
    pub(crate) hdr_uuid: uuid::Uuid,
    pub(crate) hdr_lba_table: u64,
    pub(crate) hdr_entries: u32,
    pub(crate) hdr_entsz: u32,
    pub(crate) hdr_crc_table: u32,
    pub(crate) padding: u32,
}

impl GptHdr {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub(crate) struct GptEnt {
    pub(crate) ent_type: uuid::Uuid,
    pub(crate) ent_uuid: uuid::Uuid,
    pub(crate) ent_lba_start: u64,
    pub(crate) ent_lba_end: u64,
    pub(crate) ent_attr: u64,
    pub(crate) ent_name: [u16; 36],
}

impl Default for GptEnt {
    fn default() -> Self {
        Self {
            ent_type: uuid::Uuid::new(),
            ent_uuid: uuid::Uuid::new(),
            ent_lba_start: 0,
            ent_lba_end: 0,
            ent_attr: 0,
            ent_name: [0; 36],
        }
    }
}

impl GptEnt {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

fn try_known_uuid_to_str(uuid: &uuid::Uuid, opt: &Opt) -> String {
    if opt.symbol {
        let s = subr::known_uuid_to_str(uuid);
        if !s.is_empty() {
            return s.to_string();
        }
    }
    subr::uuid_to_str(uuid)
}

fn alloc_buffer() -> Vec<u8> {
    let buf = vec![0; UNIT_SIZE];
    assert!(buf.len() == UNIT_SIZE);
    assert!(buf.len() % 512 == 0);
    buf
}

fn dump_header(fp: &mut std::fs::File, hdr_lba: u64, opt: &Opt) -> Result<GptHdr> {
    let mut buf = alloc_buffer();
    let hdr_offset = hdr_lba * u64::try_from(buf.len()).unwrap();
    fp.seek(std::io::SeekFrom::Start(hdr_offset))?;
    fp.read_exact(&mut buf)?;

    let ret = unsafe { buf.align_to::<GptHdr>() };
    assert!(ret.0.is_empty());
    let hdr = ret.1[0];

    let mut hdr_sig = [' '; 8];
    for i in 0..hdr.hdr_sig.len() {
        hdr_sig[i] = hdr.hdr_sig[i].into();
    }
    println!(
        "sig      = \"{}{}{}{}{}{}{}{}\"",
        hdr_sig[0],
        hdr_sig[1],
        hdr_sig[2],
        hdr_sig[3],
        hdr_sig[4],
        hdr_sig[5],
        hdr_sig[6],
        hdr_sig[7],
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

    println!("uuid     = {}", try_known_uuid_to_str(&hdr.hdr_uuid, opt));

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

fn dump_entries(fp: &mut std::fs::File, hdr: &GptHdr, opt: &Opt) -> Result<()> {
    let lba_table_size =
        usize::try_from(hdr.hdr_entsz).unwrap() * usize::try_from(hdr.hdr_entries).unwrap();
    let lba_table_sectors = lba_table_size / UNIT_SIZE;
    let mut total = 0;

    println!(
        "{:<3} {:<36} {:<36} {:<16} {:<16} {:<16} name",
        "#", "type", "uniq", "lba_start", "lba_end", "attr"
    );

    for i in 0..lba_table_sectors {
        let mut buf = alloc_buffer();
        let offset =
            (hdr.hdr_lba_table + u64::try_from(i).unwrap()) * u64::try_from(buf.len()).unwrap();
        fp.seek(std::io::SeekFrom::Start(offset))?;
        fp.read_exact(&mut buf)?;

        let sector_entries = buf.len() / usize::try_from(hdr.hdr_entsz).unwrap();

        for j in 0..sector_entries {
            let p = &buf[std::mem::size_of::<GptEnt>() * j..];
            let ret = unsafe { p.align_to::<GptEnt>() };
            assert!(ret.0.is_empty());
            let p = &ret.1[0];

            if !opt.verbose && *p == GptEnt::new() {
                total += 1;
                continue;
            }

            let mut name = [0u8; 36];
            let mut nlen = 0;
            assert!(p.ent_name.len() == name.len());
            for (k, v) in name.iter_mut().enumerate() {
                *v = (p.ent_name[k] & 0xFF).try_into().unwrap(); // XXX ascii
                if *v == 0 {
                    nlen = k;
                    break;
                }
            }

            println!(
                "{:<3} {:<36} {:<36} {:<016x} {:<016x} {:<016x} {}",
                i * sector_entries + j,
                try_known_uuid_to_str(&p.ent_type, opt),
                try_known_uuid_to_str(&p.ent_uuid, opt),
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

pub(crate) fn dump_gpt(fp: &mut std::fs::File, opt: &Opt) -> Result<()> {
    let mut hdr2 = GptHdr::new();

    // primary header
    println!("primary header");
    let hdr1 = dump_header(fp, 1, opt)?;

    // secondary header
    if !opt.noalt {
        println!();
        println!("secondary header");
        hdr2 = dump_header(fp, hdr1.hdr_lba_alt, opt)?;
    }

    // primary entries
    println!();
    println!("primary entries");
    dump_entries(fp, &hdr1, opt)?;

    // secondary entries
    if !opt.noalt {
        println!();
        println!("secondary entries");
        dump_entries(fp, &hdr2, opt)?;
    }
    Ok(())
}
