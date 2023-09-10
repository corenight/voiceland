/// Header bit mask
pub enum BitMask {
    /// - 0: Data stream
    /// - 1: Operation stream
    KIND = 1,

    /// - 0: Ok
    /// - 1: Error
    STATUS = 1 << 1,

    /// - 0: free money
    /// - 1: buy now
    /// I 'll assign a purpose to remaining bits in a future
    FREE = 1 << 2,
    B3 = 1 << 3,
    B4 = 1 << 4,
    B5 = 1 << 5,
    B6 = 1 << 6,
    B7 = 1 << 7,
}

// Parse to packet
/* pub fn InputPacket {

} */
