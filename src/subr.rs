use crate::uuid::Uuid;
use byteorder::ReadBytesExt;

pub fn uuid_to_str(uuid: &crate::uuid::Uuid) -> String {
    crate::uuid::uuid_to_string(uuid)
}

#[derive(PartialEq, Debug, Default)]
struct KnownUuid {
    uuid: crate::uuid::Uuid,
    name: String,
}

pub fn known_uuid_to_str(uuid: &crate::uuid::Uuid) -> String {
    let known_uuid: [KnownUuid; 64] = [
        KnownUuid {
            uuid: Uuid {
                time_low: 0x00000000,
                time_mid: 0x0000,
                time_hi_and_version: 0x0000,
                clock_seq_hi_and_reserved: 0x00,
                clock_seq_low: 0x00,
                node: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            },
            name: String::from("UNUSED"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xc12a7328,
                time_mid: 0xf81f,
                time_hi_and_version: 0x11d2,
                clock_seq_hi_and_reserved: 0xba,
                clock_seq_low: 0x4b,
                node: [0x00, 0xa0, 0xc9, 0x3e, 0xc9, 0x3b],
            },
            name: String::from("EFI"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x024dee41,
                time_mid: 0x33e7,
                time_hi_and_version: 0x11d3,
                clock_seq_hi_and_reserved: 0x9d,
                clock_seq_low: 0x69,
                node: [0x00, 0x08, 0xc7, 0x81, 0xf3, 0x9f],
            },
            name: String::from("MBR"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x516e7cb4,
                time_mid: 0x6ecf,
                time_hi_and_version: 0x11d6,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0xf8,
                node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
            },
            name: String::from("FREEBSD"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x83bd6b9d,
                time_mid: 0x7f41,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xbe,
                clock_seq_low: 0x0b,
                node: [0x00, 0x15, 0x60, 0xb8, 0x4f, 0x0f],
            },
            name: String::from("FREEBSD_BOOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x74ba7dd9,
                time_mid: 0xa689,
                time_hi_and_version: 0x11e1,
                clock_seq_hi_and_reserved: 0xbd,
                clock_seq_low: 0x04,
                node: [0x00, 0xe0, 0x81, 0x28, 0x6a, 0xcf],
            },
            name: String::from("FREEBSD_NANDFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x516e7cb5,
                time_mid: 0x6ecf,
                time_hi_and_version: 0x11d6,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0xf8,
                node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
            },
            name: String::from("FREEBSD_SWAP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x516e7cb6,
                time_mid: 0x6ecf,
                time_hi_and_version: 0x11d6,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0xf8,
                node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
            },
            name: String::from("FREEBSD_UFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x516e7cb8,
                time_mid: 0x6ecf,
                time_hi_and_version: 0x11d6,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0xf8,
                node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
            },
            name: String::from("FREEBSD_VINUM"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x516e7cba,
                time_mid: 0x6ecf,
                time_hi_and_version: 0x11d6,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0xf8,
                node: [0x00, 0x02, 0x2d, 0x09, 0x71, 0x2b],
            },
            name: String::from("FREEBSD_ZFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9e1a2d38,
                time_mid: 0xc612,
                time_hi_and_version: 0x4316,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x26,
                node: [0x8b, 0x49, 0x52, 0x1e, 0x5a, 0x8b],
            },
            name: String::from("PREP_BOOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xebd0a0a2,
                time_mid: 0xb9e5,
                time_hi_and_version: 0x4433,
                clock_seq_hi_and_reserved: 0x87,
                clock_seq_low: 0xc0,
                node: [0x68, 0xb6, 0xb7, 0x26, 0x99, 0xc7],
            },
            name: String::from("MS_BASIC_DATA"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xaf9b60a0,
                time_mid: 0x1431,
                time_hi_and_version: 0x4f62,
                clock_seq_hi_and_reserved: 0xbc,
                clock_seq_low: 0x68,
                node: [0x33, 0x11, 0x71, 0x4a, 0x69, 0xad],
            },
            name: String::from("MS_LDM_DATA"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x5808c8aa,
                time_mid: 0x7e8f,
                time_hi_and_version: 0x42e0,
                clock_seq_hi_and_reserved: 0x85,
                clock_seq_low: 0xd2,
                node: [0xe1, 0xe9, 0x04, 0x34, 0xcf, 0xb3],
            },
            name: String::from("MS_LDM_METADATA"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xde94bba4,
                time_mid: 0x06d1,
                time_hi_and_version: 0x4d40,
                clock_seq_hi_and_reserved: 0xa1,
                clock_seq_low: 0x6a,
                node: [0xbf, 0xd5, 0x01, 0x79, 0xd6, 0xac],
            },
            name: String::from("MS_RECOVERY"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xe3c9e316,
                time_mid: 0x0b5c,
                time_hi_and_version: 0x4db8,
                clock_seq_hi_and_reserved: 0x81,
                clock_seq_low: 0x7d,
                node: [0xf9, 0x2d, 0xf0, 0x02, 0x15, 0xae],
            },
            name: String::from("MS_RESERVED"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xe75caf8f,
                time_mid: 0xf680,
                time_hi_and_version: 0x4cee,
                clock_seq_hi_and_reserved: 0xaf,
                clock_seq_low: 0xa3,
                node: [0xb0, 0x01, 0xe5, 0x6e, 0xfc, 0x2d],
            },
            name: String::from("MS_SPACES"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x0fc63daf,
                time_mid: 0x8483,
                time_hi_and_version: 0x4772,
                clock_seq_hi_and_reserved: 0x8e,
                clock_seq_low: 0x79,
                node: [0x3d, 0x69, 0xd8, 0x47, 0x7d, 0xe4],
            },
            name: String::from("LINUX_DATA"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xa19d880f,
                time_mid: 0x05fc,
                time_hi_and_version: 0x4d3b,
                clock_seq_hi_and_reserved: 0xa0,
                clock_seq_low: 0x06,
                node: [0x74, 0x3f, 0x0f, 0x84, 0x91, 0x1e],
            },
            name: String::from("LINUX_RAID"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x0657fd6d,
                time_mid: 0xa4ab,
                time_hi_and_version: 0x43c4,
                clock_seq_hi_and_reserved: 0x84,
                clock_seq_low: 0xe5,
                node: [0x09, 0x33, 0xc8, 0x4b, 0x4f, 0x4f],
            },
            name: String::from("LINUX_SWAP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xe6d6d379,
                time_mid: 0xf507,
                time_hi_and_version: 0x44c2,
                clock_seq_hi_and_reserved: 0xa2,
                clock_seq_low: 0x3c,
                node: [0x23, 0x8f, 0x2a, 0x3d, 0xf9, 0x28],
            },
            name: String::from("LINUX_LVM"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xaa31e02a,
                time_mid: 0x400f,
                time_hi_and_version: 0x11db,
                clock_seq_hi_and_reserved: 0x95,
                clock_seq_low: 0x90,
                node: [0x00, 0x0c, 0x29, 0x11, 0xd1, 0xb8],
            },
            name: String::from("VMFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9d275380,
                time_mid: 0x40ad,
                time_hi_and_version: 0x11db,
                clock_seq_hi_and_reserved: 0xbf,
                clock_seq_low: 0x97,
                node: [0x00, 0x0c, 0x29, 0x11, 0xd1, 0xb8],
            },
            name: String::from("VMKDIAG"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9198effc,
                time_mid: 0x31c0,
                time_hi_and_version: 0x11db,
                clock_seq_hi_and_reserved: 0x8f,
                clock_seq_low: 0x78,
                node: [0x00, 0x0c, 0x29, 0x11, 0xd1, 0xb8],
            },
            name: String::from("VMRESERVED"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x381cfccc,
                time_mid: 0x7288,
                time_hi_and_version: 0x11e0,
                clock_seq_hi_and_reserved: 0x92,
                clock_seq_low: 0xee,
                node: [0x00, 0x0c, 0x29, 0x11, 0xd0, 0xb2],
            },
            name: String::from("VMVSANHDR"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x426F6F74,
                time_mid: 0x0000,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_BOOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x48465300,
                time_mid: 0x0000,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_HFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x55465300,
                time_mid: 0x0000,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_UFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a898cc3,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("APPLE_ZFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x52414944,
                time_mid: 0x0000,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x22,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_RAID"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x52414944,
                time_mid: 0x5f4f,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x22,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_RAID_OFFLINE"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x4C616265,
                time_mid: 0x6c00,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_LABEL"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x5265636f,
                time_mid: 0x7665,
                time_hi_and_version: 0x11AA,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_TV_RECOVERY"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x53746f72,
                time_mid: 0x6167,
                time_hi_and_version: 0x11AA,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_CORE_STORAGE"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x7c3457ef,
                time_mid: 0x0000,
                time_hi_and_version: 0x11aa,
                clock_seq_hi_and_reserved: 0xaa,
                clock_seq_low: 0x11,
                node: [0x00, 0x30, 0x65, 0x43, 0xec, 0xac],
            },
            name: String::from("APPLE_APFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x49f48d5a,
                time_mid: 0xb10e,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xb9,
                clock_seq_low: 0x9b,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_FFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x49f48d82,
                time_mid: 0xb10e,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xb9,
                clock_seq_low: 0x9b,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_LFS"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x49f48d32,
                time_mid: 0xb10e,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xB9,
                clock_seq_low: 0x9B,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_SWAP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x49f48daa,
                time_mid: 0xb10e,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xb9,
                clock_seq_low: 0x9b,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_RAID"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x2db519c4,
                time_mid: 0xb10f,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xb9,
                clock_seq_low: 0x9b,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_CCD"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x2db519ec,
                time_mid: 0xb10f,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xb9,
                clock_seq_low: 0x9b,
                node: [0x00, 0x19, 0xd1, 0x87, 0x96, 0x48],
            },
            name: String::from("NETBSD_CGD"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9d087404,
                time_mid: 0x1ca5,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x88,
                clock_seq_low: 0x17,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_LABEL32"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9d58fdbd,
                time_mid: 0x1ca5,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x88,
                clock_seq_low: 0x17,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_SWAP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9d94ce7c,
                time_mid: 0x1ca5,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x88,
                clock_seq_low: 0x17,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_UFS1"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x9dd4478f,
                time_mid: 0x1ca5,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x88,
                clock_seq_low: 0x17,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_VINUM"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xdbd5211b,
                time_mid: 0x1ca5,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x88,
                clock_seq_low: 0x17,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_CCD"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x3d48ce54,
                time_mid: 0x1d16,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x86,
                clock_seq_low: 0x96,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_LABEL64"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xbd215ab2,
                time_mid: 0x1d16,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x86,
                clock_seq_low: 0x96,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_LEGACY"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x61dc63ac,
                time_mid: 0x6e38,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0x85,
                clock_seq_low: 0x13,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_HAMMER"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x5cbb9ad1,
                time_mid: 0x862d,
                time_hi_and_version: 0x11dc,
                clock_seq_hi_and_reserved: 0xa9,
                clock_seq_low: 0x4d,
                node: [0x01, 0x30, 0x1b, 0xb8, 0xa9, 0xf5],
            },
            name: String::from("DRAGONFLY_HAMMER2"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xcab6e88e,
                time_mid: 0xabf3,
                time_hi_and_version: 0x4102,
                clock_seq_hi_and_reserved: 0xa0,
                clock_seq_low: 0x7a,
                node: [0xd4, 0xbb, 0x9b, 0xe3, 0xc1, 0xd3],
            },
            name: String::from("CHROMEOS_FIRMWARE"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0xfe3a2a5d,
                time_mid: 0x4f32,
                time_hi_and_version: 0x41a7,
                clock_seq_hi_and_reserved: 0xb7,
                clock_seq_low: 0x25,
                node: [0xac, 0xcc, 0x32, 0x85, 0xa3, 0x09],
            },
            name: String::from("CHROMEOS_KERNEL"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x2e0a753d,
                time_mid: 0x9e48,
                time_hi_and_version: 0x43b0,
                clock_seq_hi_and_reserved: 0x83,
                clock_seq_low: 0x37,
                node: [0xb1, 0x51, 0x92, 0xcb, 0x1b, 0x5e],
            },
            name: String::from("CHROMEOS_RESERVED"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x3cb8e202,
                time_mid: 0x3b7e,
                time_hi_and_version: 0x47dd,
                clock_seq_hi_and_reserved: 0x8a,
                clock_seq_low: 0x3c,
                node: [0x7f, 0xf2, 0xa1, 0x3c, 0xfc, 0xec],
            },
            name: String::from("CHROMEOS_ROOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x824cc7a0,
                time_mid: 0x36a8,
                time_hi_and_version: 0x11e3,
                clock_seq_hi_and_reserved: 0x89,
                clock_seq_low: 0x0a,
                node: [0x95, 0x25, 0x19, 0xad, 0x3f, 0x61],
            },
            name: String::from("OPENBSD_DATA"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a82cb45,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_BOOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a85cf4d,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_ROOT"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a87c46f,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_SWAP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a8b642b,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_BACKUP"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a8ef2e9,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_VAR"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a90ba39,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_HOME"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a9283a5,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_ALTSEC"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x6a945a3b,
                time_mid: 0x1dd2,
                time_hi_and_version: 0x11b2,
                clock_seq_hi_and_reserved: 0x99,
                clock_seq_low: 0xa6,
                node: [0x08, 0x00, 0x20, 0x73, 0x66, 0x31],
            },
            name: String::from("SOLARIS_RESERVED"),
        },
        KnownUuid {
            uuid: Uuid {
                time_low: 0x21686148,
                time_mid: 0x6449,
                time_hi_and_version: 0x6e6f,
                clock_seq_hi_and_reserved: 0x74,
                clock_seq_low: 0x4e,
                node: [0x65, 0x65, 0x64, 0x45, 0x46, 0x49],
            },
            name: String::from("BIOS_BOOT"),
        },
    ];

    assert!(is_le());

    for x in known_uuid {
        if *uuid == x.uuid {
            assert!(x.name.len() <= 36);
            return x.name;
        }
    }
    String::from("")
}

pub fn is_le() -> bool {
    cfg!(target_endian = "little")
}

pub fn assert_ds() {
    assert!(std::mem::size_of::<crate::gpt::GptHdr>() == 92 + 4);
    assert!(std::mem::size_of::<crate::gpt::GptEnt>() == 128);
    assert!(std::mem::size_of::<crate::uuid::Uuid>() == 16);
}

pub fn read_u8(fp: &mut std::fs::File) -> Result<u8, std::io::Error> {
    fp.read_u8()
}

pub fn read_le16(fp: &mut std::fs::File) -> Result<u16, std::io::Error> {
    fp.read_u16::<byteorder::LittleEndian>()
}

pub fn read_le32(fp: &mut std::fs::File) -> Result<u32, std::io::Error> {
    fp.read_u32::<byteorder::LittleEndian>()
}

pub fn read_le64(fp: &mut std::fs::File) -> Result<u64, std::io::Error> {
    fp.read_u64::<byteorder::LittleEndian>()
}
