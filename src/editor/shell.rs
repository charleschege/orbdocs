pub fn editor_shell() {}

pub struct WebShell {
    title: String,
    description: Option<String>,
    editor: Editor,
    footer: Copyright,
}

#[derive(Debug)]
pub struct Copyright {
    symbol: &'static str,
    date: String,
    institution: &'static str,
}

impl Copyright {
    //FIXME add a dynamic year
    pub fn gen_year(&mut self) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn build(&self) -> String {
        let mut copyright = String::new();
        copyright.push_str(self.symbol);
        copyright.push_str(&self.date);
        copyright.push_str(" ");
        copyright.push_str(self.institution);

        copyright
    }
}

impl Default for Copyright {
    fn default() -> Self {
        Self {
            symbol: "©",
            date: "2021".into(), //FIXME add tests for one week ahead and give panic notifications if year in conflict,
            institution: "·core43",
        }
    }
}

#[derive(Debug)]
pub struct Footer {
    copyright: Copyright,
    legal: Legal,
    social: Social,
}

#[derive(Debug)]
pub struct Legal {
    tos: String, //FIXME to url
    pp: String,  // FIXME to url
}

#[derive(Debug)]
pub struct Social {
    //FIXME chnage these to URLs
    tel: Vec<String>, //FIXME dynamic from DB
    core43chat: String,
    twitter: String,
    github: String,
    signal: String,
    whatsapp: String,
}

#[derive(Debug)]
pub struct Editor {}
