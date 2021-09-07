#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;
        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx_tutorial/include/blobstore.h");
        type BlobstoreClient;
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn print();
    }
}

pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize;
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice);
}

fn main() {
    // let client = ffi::new_blobstore_client();
    ffi::print();
}
