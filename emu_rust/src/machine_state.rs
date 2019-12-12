pub struct MachineState {
    pub i0: i64,
    pub i1: i64,
    pub i2: i64,
    pub i3: i64,
    pub i4: i64,
    pub i5: i64,
    pub i6: i64,
    pub i7: i64,

    pub pc: i64,
    pub sp: i64,

    pub memory: Vec<u8>,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            i0: 0,
            i1: 0,
            i2: 0,
            i3: 0,
            i4: 0,
            i5: 0,
            i6: 0,
            i7: 0,

            pc: 0x1000,
            sp: 0x0,
            memory: vec![0; 0x10000],
        }
    }
}
