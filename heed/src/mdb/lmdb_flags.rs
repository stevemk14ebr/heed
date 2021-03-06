// LMDB flags (see http://www.lmdb.tech/doc/group__mdb__env.html for more details).
#[repr(u32)]
pub enum Flags {
    MdbFixedmap = lmdb_sys::MDB_FIXEDMAP,
    MdbNoSubDir = lmdb_sys::MDB_NOSUBDIR,
    MdbNoSync = lmdb_sys::MDB_NOSYNC,
    MdbRdOnly = lmdb_sys::MDB_RDONLY,
    MdbNoMetaSync = lmdb_sys::MDB_NOMETASYNC,
    MdbWriteMap = lmdb_sys::MDB_WRITEMAP,
    MdbMapAsync = lmdb_sys::MDB_MAPASYNC,
    MdbNoTls = lmdb_sys::MDB_NOTLS,
    MdbNoLock = lmdb_sys::MDB_NOLOCK,
    MdbNoRdAhead = lmdb_sys::MDB_NORDAHEAD,
    MdbNoMemInit = lmdb_sys::MDB_NOMEMINIT,
}

#[repr(u32)]
pub enum DBFlags {
    MdbReverseKey = 0x02,
    MdbDupSort = 0x04,
    MdbIntegerKey = 0x08,
    MdbDupFixed = 0x10,
    MdbIntegerDup = 0x20,
    MdbReverseDup = 0x40,
    MdbCreate = 0x40000,
}