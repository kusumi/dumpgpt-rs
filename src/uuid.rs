pub const UUID_NODE_LEN: usize = 6;

#[repr(C)] // should be packed
#[derive(PartialEq, Debug, Default)]
pub struct Uuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_hi_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: [u8; UUID_NODE_LEN],
}

pub fn read_uuid_le(fp: &mut std::fs::File) -> Result<Uuid, std::io::Error> {
    let mut uuid = Uuid {
        ..Default::default()
    };

    uuid.time_low = crate::subr::read_le32(fp)?;
    uuid.time_mid = crate::subr::read_le16(fp)?;
    uuid.time_hi_and_version = crate::subr::read_le16(fp)?;
    uuid.clock_seq_hi_and_reserved = crate::subr::read_u8(fp)?;
    uuid.clock_seq_low = crate::subr::read_u8(fp)?;
    for i in 0..UUID_NODE_LEN {
        uuid.node[i] = crate::subr::read_u8(fp)?;
    }

    Ok(uuid)
}

pub fn uuid_to_string(u: &Uuid) -> String {
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        u.time_low,
        u.time_mid,
        u.time_hi_and_version,
        u.clock_seq_hi_and_reserved,
        u.clock_seq_low,
        u.node[0],
        u.node[1],
        u.node[2],
        u.node[3],
        u.node[4],
        u.node[5]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_uuid_to_string() {
        let u = super::Uuid {
            ..super::Uuid::default()
        };
        assert_eq!(
            super::uuid_to_string(&u),
            "00000000-0000-0000-0000-000000000000"
        );

        let u = super::Uuid {
            time_low: 0x516e7cb4,
            time_mid: 0x6ecf,
            time_hi_and_version: 0x11d6,
            clock_seq_hi_and_reserved: 0x8f,
            clock_seq_low: 0xf8,
            node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
        };
        assert_eq!(
            super::uuid_to_string(&u),
            "516e7cb4-6ecf-11d6-8ff8-00022d09712b"
        );
    }
}
