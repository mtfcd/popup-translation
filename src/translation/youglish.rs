use crate::translation::Translator;

#[derive(Copy, Clone)]
pub struct Youglish {}

impl Translator for Youglish {
    fn name(&self) -> String {
        "youglish".into()
    }

    fn url(&self, word: String) -> String {
        format!("https://youglish.com/pronounce/{word}/english?")
    }

    fn js_code(&self) -> String {
        include_str!("../js/youglish.js").into()
    }
}