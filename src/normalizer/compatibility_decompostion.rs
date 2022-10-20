use std::borrow::Cow;

use unicode_normalization::UnicodeNormalization;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct CompatibilityDecompositionNormalizer;

impl Normalizer for CompatibilityDecompositionNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        let mut lemma = String::new();
        if options.create_char_map {
            let mut char_map = Vec::new();
            for c in token.lemma().chars() {
                let decomposition: String = c.nfkd().collect();
                char_map.push((c.len_utf8() as u8, decomposition.len() as u8));
                lemma.push_str(&decomposition);
            }
            token.char_map = Some(char_map);
        } else {
            lemma.push_str(&token.lemma().nfkd().collect::<String>());
        }
        token.lemma = Cow::Owned(lemma);
        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, _script: Script, _language: Option<Language>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("ｶﾞｷﾞｸﾞｹﾞｺﾞ".to_string()),
            char_end: 10,
            byte_end: 30,
            script: Script::Cj,
            language: Some(Language::Jpn),
            ..Default::default()
        }]
    }

    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("ガギグゲゴ".to_string()),
            char_end: 10,
            byte_end: 30,
            char_map: Some(vec![
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
                (3, 3),
            ]),
            script: Script::Cj,
            language: Some(Language::Jpn),
            ..Default::default()
        }]
    }

    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("ガギグゲゴ".to_string()),
            char_end: 5,
            byte_end: 30,
            char_map: Some(vec![(6, 3), (6, 3), (6, 3), (6, 3), (6, 3)]),
            script: Script::Cj,
            language: Some(Language::Jpn),
            ..Default::default()
        }]
    }

    test_normalizer!(
        CompatibilityDecompositionNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}
