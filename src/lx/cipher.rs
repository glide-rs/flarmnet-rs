use std::io::Read;

pub fn encrypt(s: &[u8]) -> Vec<u8> {
    s.iter().map(|b| b.wrapping_add(1)).collect()
}

#[derive(Clone)]
pub struct Reader<R: Read> {
    inner: R,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|result| {
            for byte in buf.iter_mut() {
                *byte = byte.wrapping_sub(1);
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{encrypt, Reader};
    use std::io::copy;

    fn decrypt(s: &[u8]) -> Vec<u8> {
        let mut reader = Reader::new(s);
        let mut vec = Vec::with_capacity(s.len());
        copy(&mut reader, &mut vec).unwrap();
        vec
    }

    #[test]
    fn decryption_works() {
        assert_eq!(
            decrypt(b"=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?=GMBSNOFU!Wfstjpo>#11:44f#?=GMBSNEBUB!GmbsnJE>#111111#?"),
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FLARMNET Version=\"00933e\">\n<FLARMDATA FlarmID=\"000000\">\n"
        );
    }

    #[test]
    fn encryption_works() {
        assert_eq!(
            encrypt(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FLARMNET Version=\"00933e\">\n<FLARMDATA FlarmID=\"000000\">\n"),
            b"=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?=GMBSNOFU!Wfstjpo>#11:44f#?=GMBSNEBUB!GmbsnJE>#111111#?"
        );
    }
}
