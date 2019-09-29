use bitflags::{bitflags};

bitflags! {
    pub struct CapabilityFlags: u32 {
        const CLIENT_LONG_PASSWORD                  = 0x00000001u32;
        const CLIENT_FOUND_ROWS                     = 0x00000002u32;
        const CLIENT_LONG_FLAG                      = 0x00000004u32;
        const CLIENT_CONNECT_WITH_DB                = 0x00000008u32;
        const CLIENT_NO_SCHEMA                      = 0x00000010u32;
        const CLIENT_COMPRESS                       = 0x00000020u32;
        const CLIENT_ODBC                           = 0x00000040u32;
        const CLIENT_LOCAL_FILES                    = 0x00000080u32;
        const CLIENT_IGNORE_SPACE                   = 0x00000100u32;
        const CLIENT_PROTOCOL_41                    = 0x00000200u32;
        const CLIENT_INTERACTIVE                    = 0x00000400u32;
        const CLIENT_SSL                            = 0x00000800u32;
        const CLIENT_IGNORE_SIGPIPE                 = 0x00001000u32;
        const CLIENT_TRANSACTIONS                   = 0x00002000u32;
        const CLIENT_RESERVED                       = 0x00004000u32;
        const CLIENT_SECURE_CONNECTION              = 0x00008000u32;
        const CLIENT_MULTI_STATEMENTS               = 0x00010000u32;
        const CLIENT_MULTI_RESULTS                  = 0x00020000u32;
        const CLIENT_PS_MULTI_RESULTS               = 0x00040000u32;
        const CLIENT_PLUGIN_AUTH                    = 0x00080000u32;
        const CLIENT_CONNECT_ATTRS                  = 0x00100000u32;
        const CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA = 0x00200000u32;
    }
}