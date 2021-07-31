pub fn decrypt(s: &str) -> String {
    let bytes: Vec<_> = s.bytes().map(|b| b.wrapping_sub(1)).collect();
    String::from_utf8(bytes).unwrap()
}

#[allow(dead_code)]
pub fn encrypt(s: &str) -> String {
    let bytes: Vec<_> = s.bytes().map(|b| b.wrapping_add(1)).collect();
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn decryption_works() {
        assert_eq!(
            decrypt("=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?=GMBSNOFU!Wfstjpo>#11:44f#?=GMBSNEBUB!GmbsnJE>#111111#?"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FLARMNET Version=\"00933e\">\n<FLARMDATA FlarmID=\"000000\">\n"
        );
    }

    #[test]
    fn encryption_works() {
        assert_eq!(
            encrypt("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<FLARMNET Version=\"00933e\">\n<FLARMDATA FlarmID=\"000000\">\n"),
            "=@ynm!wfstjpo>#2/1#!fodpejoh>#VUG.9#@?=GMBSNOFU!Wfstjpo>#11:44f#?=GMBSNEBUB!GmbsnJE>#111111#?"
        );
    }
}
