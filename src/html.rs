use crate::SlapConfig;

pub struct HtmlBuilder {
    pub config: SlapConfig,
    pub html: String
}

impl HtmlBuilder {
    pub fn from_config(config: SlapConfig) -> HtmlBuilder {
        let builder = HtmlBuilder {
            config,
            html: String::from("<!doctype html>\n<html>")
        };
        builder.add_head().add_body()
    }

    pub fn add_head(mut self) -> HtmlBuilder {
        let header = format!(
            "\n<head>\
            \n<meta charset=\"utf-8\">\
            \n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\">\
            \n{}\
            \n<title>{}</title> \
            \n</head>",
            bootstrap_cdn_link(),
            self.config.title
        );

        self.html += &header;
        self
    }

    pub fn add_body(mut self) -> HtmlBuilder {
        let body = format!(
            "\n<body>\
            \n<h1>{}</h1>\
            \n</body>",
            self.create_body_content()
        );

        self.html += &body;
        self
    }

    fn create_body_content(&self) -> String {
        "Hello".to_string() // TODO: create body from config
    }

    // Consuming the builder and creating the html as a String
    pub fn build(mut self) -> String {
        self.html += "\n</html>";
        self.html
    }
}

fn bootstrap_cdn_link() -> String {
    String::from(r##"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@4.3.1/dist/css/bootstrap.min.css" integrity="sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T" crossorigin="anonymous">"##)
}

