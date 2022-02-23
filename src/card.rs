pub struct Card {
    pub title: String,
    pub link: String,
    pub description: String,
    pub image_link: String,
}

impl Card {
    pub fn new() -> Card {
        Card {
            title: "Title".to_string(),
            link: "http://localhost:8080".to_string(),
            description: "This is a description of a website".to_string(),
            image_link: "./static/rust.jpg".to_string(),
        }
    }

    pub fn title(mut self, title: String) -> Card {
        self.title = title;
        self
    }

    pub fn link(mut self, link: String) -> Card {
        self.link = link;
        self
    }

    pub fn description(mut self, description: String) -> Card {
        self.description = description;
        self
    }

    pub fn image_link(mut self, image_link: String) -> Card {
        self.image_link = image_link;
        self
    }

}