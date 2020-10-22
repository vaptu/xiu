use bytes::{ BytesMut};

//5.3.1.1
#[derive(Eq, PartialEq, Debug)]
pub struct ChunkBasicHeader {
    pub format: u8,
    pub chunk_stream_id: u32,
}


impl ChunkBasicHeader {
    pub fn new(fmt: u8, csid: u32) -> ChunkBasicHeader {
        ChunkBasicHeader {
            format: fmt,
            chunk_stream_id: csid,
        }
    }
}

//5.3.1.2
#[derive(Eq, PartialEq, Debug)]
pub struct ChunkMessageHeader {
    timestamp: u64,
    msg_length: u32,
    msg_type_id: u8,
    msg_streamd_id: u32,
    timestamp_delta: u32,
}

impl ChunkMessageHeader {
    pub fn new() -> ChunkMessageHeader {
        ChunkMessageHeader {
            timestamp: 0,
            msg_length: 0,
            msg_type_id: 0,
            msg_streamd_id: 0,
            timestamp_delta: 0,
        }
    }
}

pub struct ChunkHeader {
    basic_header: ChunkBasicHeader,
    message_header: ChunkMessageHeader,
}

impl ChunkHeader {
    pub fn new() -> ChunkHeader {
        ChunkHeader {
            basic_header: ChunkBasicHeader::new(0, 0),
            message_header: ChunkMessageHeader::new(),
        }
    }
}

pub struct Chunk {
    basic_header: ChunkBasicHeader,
    message_header: ChunkMessageHeader,
    raw_data: BytesMut,
}

// impl Chunk {
//     pub fn chunk_read(&mut self, bytes: &[u8]) -> Result {
//         self.buffer.extend_from_slice(bytes);
//     }

//     pub fn read_basic_header(&mut self, bytes: &[u8]) -> Result<UnpackResult, ChunkUnpackError> {
//         if self.buffer.len() < 1 {
//             return Ok(UnpackResult::NotEnoughBytes);
//         }
//     }
// }