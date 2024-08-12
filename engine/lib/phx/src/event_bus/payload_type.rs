#[luajit_ffi_gen::luajit_ffi(with_impl = true)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventPayloadType {
    /// Lua object pointer/index to communicate inside scripts only
    Lua,

    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,

    BoolArray,
    I8Array,
    U8Array,
    I16Array,
    U16Array,
    I32Array,
    U32Array,
    I64Array,
    U64Array,
    F32Array,
    F64Array,
    StringArray,

    Table,
}

#[luajit_ffi_gen::luajit_ffi]
impl EventPayloadType {
    pub fn is_array(&self) -> bool {
        matches!(
            self,
            Self::BoolArray
                | Self::I8Array
                | Self::U8Array
                | Self::I16Array
                | Self::U16Array
                | Self::I32Array
                | Self::U32Array
                | Self::I64Array
                | Self::U64Array
                | Self::F32Array
                | Self::F64Array
                | Self::StringArray
        )
    }
}
