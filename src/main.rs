use iced::widget::{column, container, text, row, button, scrollable, text_input};
use iced::{Element, Length};

fn main() -> iced::Result {
    iced::run(Stardust::update, Stardust::view)
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    CategorySelected(Category),
    AppSelected(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    All,
    Development,
    Graphics,
    Office,
    Games,
    Multimedia,
    Network,
    Utilities,
}

impl Category {
    fn as_str(self) -> &'static str {
        match self {
            Category::All => "All",
            Category::Development => "Development",
            Category::Graphics => "Graphics",
            Category::Office => "Office",
            Category::Games => "Games",
            Category::Multimedia => "Multimedia",
            Category::Network => "Network",
            Category::Utilities => "Utilities",
        }
    }
}

#[derive(Default)]
struct Stardust {
    search_query: String,
    selected_category: Category,
}

impl Default for Category {
    fn default() -> Self {
        Category::All
    }
}

impl Stardust {
    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(query) => {
                self.search_query = query;
            }
            Message::CategorySelected(category) => {
                self.selected_category = category;
            }
            Message::AppSelected(app_name) => {
                // Handle app selection - to be implemented
                println!("Selected app: {}", app_name);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Stardust App Store")
            .size(28);

        let search_bar = text_input("Search applications...", &self.search_query)
            .on_input(Message::SearchChanged)
            .padding(10)
            .width(Length::Fill);

        let categories = row![
            self.category_button(Category::All),
            self.category_button(Category::Development),
            self.category_button(Category::Graphics),
            self.category_button(Category::Office),
            self.category_button(Category::Games),
            self.category_button(Category::Multimedia),
            self.category_button(Category::Network),
            self.category_button(Category::Utilities),
        ]
        .spacing(10)
        .padding(10);

        let content_text = if self.search_query.is_empty() {
            format!(
                "Browsing: {}\n\nWelcome to Stardust!\n\nThis is an experimental app store built with Rust and Iced.\nStart by searching for applications or browsing categories.",
                self.selected_category.as_str()
            )
        } else {
            format!(
                "Searching for '{}' in {}\n\nSearch results would appear here.",
                self.search_query,
                self.selected_category.as_str()
            )
        };

        let content_area = scrollable(
            container(text(content_text).size(16))
                .width(Length::Fill)
                .padding(20)
        )
        .height(Length::Fill);

        let content = column![
            title,
            search_bar,
            categories,
            content_area,
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn category_button(&self, category: Category) -> Element<'_, Message> {
        button(text(category.as_str()))
            .on_press(Message::CategorySelected(category))
            .padding(8)
            .into()
    }
}
