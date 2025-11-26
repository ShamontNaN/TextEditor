/// Данный модуль представляет собой Функционал работы с заголовками markdown, парсит их и создает  
#[derive(Debug)]
pub enum HeadingToken {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct Heading {
    level: HeadingToken,
    content: String,
}

impl Heading {
    pub fn new(level: HeadingToken, content: String) -> Self {
        Heading { level, content }
    }

    pub fn to_String(&self) -> String {
        let prefix = match self.level {
            HeadingToken::H1 => "#",
            HeadingToken::H2 => "##",
            HeadingToken::H3 => "###",
            HeadingToken::H4 => "####",
            HeadingToken::H5 => "#####",
            HeadingToken::H6 => "######",
        };
        format!("{} {}", prefix, self.content)
    }

    //pub fn from_String(string: String) -> Result<Heading,>
}
