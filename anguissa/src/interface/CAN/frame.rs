pub struct Frame<'a> {
    pub id: u32,
    pub data: &'a[u8],
    pub rtr: bool,
    pub err: bool,
}

impl Frame {
    
}