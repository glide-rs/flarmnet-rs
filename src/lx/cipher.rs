use std::io::{Read, Write};

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

#[derive(Clone)]
pub struct Writer<W: Write> {
    inner: W,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let new_buf: Vec<_> = buf.iter().map(|b| b.wrapping_add(1)).collect();
        self.inner.write(&new_buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::{Reader, Writer};
    use std::io::copy;

    fn decrypt(s: &[u8]) -> Vec<u8> {
        let mut reader = Reader::new(s);
        let mut vec = Vec::with_capacity(s.len());
        copy(&mut reader, &mut vec).unwrap();
        vec
    }

    fn encrypt(mut s: &[u8]) -> Vec<u8> {
        let vec = Vec::with_capacity(s.len());
        let mut writer = Writer::new(vec);
        copy(&mut s, &mut writer).unwrap();
        writer.into_inner()
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
