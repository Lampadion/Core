pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use icu::normalizer::{ComposingNormalizerBorrowed, DecomposingNormalizerBorrowed};

    #[test]
    fn it_works() {
        let nfc = ComposingNormalizerBorrowed::new_nfc();
        let nfd = DecomposingNormalizerBorrowed::new_nfd();

        // e + COMBINING ACUTE ACCENT
        let decomposed = "e\u{0301}";

        // LATIN SMALL LETTER E WITH ACUTE
        let composed = "é";

        let result_nfc = nfc.normalize(decomposed);
        let result_nfd = nfd.normalize(composed);

        println!("input NFD: {decomposed:?}");
        println!("NFC:        {result_nfc:?}");

        println!("input NFC: {composed:?}");
        println!("NFD:       {result_nfd:?}");

        assert_eq!(result_nfc, composed);
        assert_eq!(result_nfd, decomposed);

        assert!(nfc.is_normalized(composed));
        assert!(nfd.is_normalized(decomposed));

        assert!(!nfc.is_normalized(decomposed));
        assert!(!nfd.is_normalized(composed));
    }
}
