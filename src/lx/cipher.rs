pub fn decrypt(s: &[u8]) -> Vec<u8> {
    s.iter().map(|b| b.wrapping_sub(1)).collect()
}

pub fn encrypt(s: &[u8]) -> Vec<u8> {
    s.iter().map(|b| b.wrapping_add(1)).collect()
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

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
