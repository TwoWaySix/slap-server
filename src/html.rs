use crate::Config;


pub struct HtmlBuilder {
    pub config: Config,
    pub html: String
}

impl HtmlBuilder {
    pub fn from_config(config: Config) -> HtmlBuilder {
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
            \n<div class=\"container\">
            \n{}\
            \n</div>
            \n</body>",
            self.create_body_content()
        );

        self.html += &body;
        self
    }

    fn create_body_content(&self) -> String {
        self.create_rows()
    }

    fn create_rows(&self) -> String {
        let number_of_rows = (
            (self.config.cards.len() as f64) / (self.config.n_cols as f64)
        ).ceil() as usize;

        let mut rows = String::from("");
        for i in 0..number_of_rows {
            let row = self.create_row(
                i * number_of_rows,
                i * number_of_rows + self.config.n_cols
            );
            rows += &row;
        }
        rows
    }

    fn create_row(&self, idx0: usize, idx1: usize) -> String {
        let mut row = "\n<div class=\"row\">".to_string();

        let mut idx1_corr = idx1;
        if idx1 >= self.config.cards.len() {
            idx1_corr = self.config.cards.len()
        };

        for i in idx0..idx1_corr {
            if let Some(card) = self.config.cards.get(i) {
                row += &format!(
                    "\n<div class=\"col-sm\">\
                    \n<a href=\"{}\">\
                    \n<img src=\"{}\" style=\"width:100%\">
                    \n<h1>{}</h1>\
                    \n<p>{}</p>\
                    \n</a>\
                    \n</div>",
                    card.link,
                    card.image_link,
                    card.title,
                    card.description
                );
            }
        }
        row += "\n</div>";
        row
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

