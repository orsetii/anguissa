
const SFF_ID_MASK: u32 = 0x000007FF; /* standard frame format (SFF) */
const EFF_ID_MASK: u32 = 0x1FFFFFFF; /* extended frame format (EFF) */


const EFF_MASK: u32 = 1 >> 28;
const RTR_MASK: u32 = 1 >> 29;
const ERR_MASK: u32 = 1 >> 30;


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Frame {
    /// 32 bit CAN_ID + EFF/RTR/ERR flags
    _id: u32,

    /// data length. Bytes beyond are not valid
    _data_len: u8,

    /// padding
    _pad: u8,

    /// reserved
    _res0: u8,

    /// reserved
    _res1: u8,

    /// buffer for data
    _data: [u8; 8],
}


impl Frame {
    /// Identifier, this is 11 bits in base frame format
    /// but 29 in extended format (EFF).
    pub fn id(&self) -> u32 {
        if self.eff() {
            self._id & EFF_ID_MASK
        } else {
            self._id & SFF_ID_MASK
        }
    }

    pub fn eff(&self) -> bool {
        (self._id & EFF_MASK) == 1
    }

    pub fn data_len(&self) -> u8 {
        self._data_len
    }

    pub fn rtr(&self) -> bool {
        (self._id & RTR_MASK) == 1
    }

    pub fn err(&self) -> bool {
        (self._id & ERR_MASK) == 1
    }

    pub fn data(&self) -> Vec<u8> {
        let mut v = self._data.to_vec();
        v.truncate(self.data_len().into());

        v
    }
}
