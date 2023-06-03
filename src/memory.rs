#![warn(dead_code)]
pub enum ChunkType {
    INTEGER,
    BIN,
    TXT,
    BEGIN
}

pub struct Data {
    data: Vec<i32>,
    size: i32,
}

pub struct Memory {
    chunks: Vec<Chunk>,
    current_address: i32,
}

pub struct Chunk {
    size: usize,
    address: i32,
    data: Data,
}
impl Memory {
    fn new() -> Memory{
        let mut mem = Memory {
            chunks: vec![],
            current_address: 0x0001,
        };
        let data = Data {
            data: vec![0, 1, 55], 
            size: 1 
        };
        let chunk = Chunk::new(data, ChunkType::BEGIN, &mut mem);
        mem.chunks.push(chunk);

        return mem;        
    }
}
impl Data {
    fn getdata(&self) -> &Vec<i32> {
        &self.data
    }

    fn write(&mut self, content: Vec<i32>) -> &mut Self {
        self.data = content;
        self
    }
}

impl Chunk {
    fn new(data: Data, ctype: ChunkType, mem: &mut Memory) -> Chunk {
        match ctype {
            ChunkType::INTEGER => Chunk {
                size: 2,
                address: mem.current_address + 1000,
                data,
            },
            ChunkType::BIN => Chunk {
                size: 1,
                address: mem.current_address + 1000,
                data,
            },
            ChunkType::TXT => Chunk {
                size: 3,
                address: mem.current_address + 1000,
                data,
            },
            ChunkType::BEGIN => todo!(),
        }
    }

    fn inject(&mut self) -> &Data {
        &self.data
    }

    fn getdata(&self) -> &Data {
        &self.data
    }
}