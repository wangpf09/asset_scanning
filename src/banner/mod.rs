use colorful::{Color, Colorful};

pub struct Banner {
    banner: String,
}

impl Banner {
    pub fn init() -> Self {
        let banner = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            r#" _____ _____   ___   _   _  _   _  ___________ "#,
            r#"/  ___/  __ \ / _ \ | \ | || \ | ||  ___| ___ \"#,
            r#"\ `--.| /  \// /_\ \|  \| ||  \| || |__ | |_/ /"#,
            r#" `--. \ |    |  _  || . ` || . ` ||  __||    / "#,
            r#"/\__/ / \__/\| | | || |\  || |\  || |___| |\ \ "#,
            r#"\____/ \____/\_| |_/\_| \_/\_| \_/\____/\_| \_|"#,
            r#"The Modern Day Assert Scanner."#
        );
        Self { banner }
    }

    pub fn print() {
        let banner = Self::init();
        Self::_print(banner)
    }

    fn _print(self) {
        println!("{}", self.banner.gradient(Color::Black).bold());
    }
}
