mod api;

use iced::widget::{column, container, text, row, button, scrollable, text_input};
use iced::{Element, Length, Task};
use api::{AppInfo, SparkStoreApi};

// Constants
const MAX_DISPLAYED_APPS: usize = 50;
const MIN_SEARCH_LENGTH: usize = 2;

fn main() -> iced::Result {
    iced::run(Stardust::update, Stardust::view)
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    CategorySelected(Category),
    AppSelected(String),
    LoadApps,
    AppsLoaded(Result<Vec<AppInfo>, String>),
    SearchApps,
    SearchResults(Result<Vec<AppInfo>, String>),
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
    
    fn to_api_category(self) -> &'static str {
        match self {
            Category::All => "all",
            Category::Development => "development",
            Category::Graphics => "graphics",
            Category::Office => "office",
            Category::Games => "games",
            Category::Multimedia => "multimedia",
            Category::Network => "network",
            Category::Utilities => "utilities",
        }
    }
}

#[derive(Default)]
struct Stardust {
    search_query: String,
    selected_category: Category,
    apps: Vec<AppInfo>,
    loading: bool,
    error_message: Option<String>,
    api: SparkStoreApi,
}

impl Default for Category {
    fn default() -> Self {
        Category::All
    }
}

impl Stardust {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(query) => {
                self.search_query = query;
                // Only trigger search if query meets minimum length requirement
                if self.search_query.len() >= MIN_SEARCH_LENGTH {
                    return Task::perform(async {}, |_| Message::SearchApps);
                }
                Task::none()
            }
            Message::CategorySelected(category) => {
                self.selected_category = category;
                self.search_query.clear();
                self.loading = true;
                self.error_message = None;
                
                let api = self.api.clone();
                let cat = category.to_api_category().to_string();
                
                Task::perform(
                    async move {
                        api.fetch_app_list(&cat).await
                    },
                    Message::AppsLoaded
                )
            }
            Message::LoadApps => {
                self.loading = true;
                self.error_message = None;
                
                let api = self.api.clone();
                let cat = self.selected_category.to_api_category().to_string();
                
                Task::perform(
                    async move {
                        api.fetch_app_list(&cat).await
                    },
                    Message::AppsLoaded
                )
            }
            Message::AppsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(apps) => {
                        self.apps = apps;
                        self.error_message = None;
                    }
                    Err(error) => {
                        self.error_message = Some(error);
                        self.apps.clear();
                    }
                }
                Task::none()
            }
            Message::SearchApps => {
                if self.search_query.is_empty() {
                    return Task::none();
                }
                
                self.loading = true;
                self.error_message = None;
                
                let api = self.api.clone();
                let keyword = self.search_query.clone();
                
                Task::perform(
                    async move {
                        api.search_apps(&keyword).await
                    },
                    Message::SearchResults
                )
            }
            Message::SearchResults(result) => {
                self.loading = false;
                match result {
                    Ok(apps) => {
                        self.apps = apps;
                        self.error_message = None;
                    }
                    Err(error) => {
                        self.error_message = Some(error);
                        self.apps.clear();
                    }
                }
                Task::none()
            }
            Message::AppSelected(app_name) => {
                println!("Selected app: {}", app_name);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title_text = text("Stardust App Store")
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

        let content_area: Element<Message> = if self.loading {
            column![
                text("Loading applications from Spark Store...").size(16)
            ]
            .padding(20)
            .into()
        } else if let Some(ref error) = self.error_message {
            column![
                text(format!("Error: {}", error)).size(16)
            ]
            .padding(20)
            .into()
        } else if self.apps.is_empty() {
            let message = if self.search_query.is_empty() {
                format!(
                    "Browsing: {}\n\nWelcome to Stardust!\n\nConnected to Spark Store server ({})\nClick 'Load Apps' button or search for applications.",
                    self.selected_category.as_str(),
                    "https://cdn-d.spark-app.store/"
                )
            } else {
                format!(
                    "No results found for '{}'\n\nTry a different search term.",
                    self.search_query
                )
            };
            
            let load_button = button(text("Load Apps"))
                .on_press(Message::LoadApps)
                .padding(10);
            
            column![
                text(message).size(16),
                load_button
            ]
            .spacing(20)
            .padding(20)
            .into()
        } else {
            let mut app_list = column![
                text(format!("{} applications found from Spark Store", self.apps.len())).size(16)
            ]
            .spacing(10)
            .padding(20);
            
            for app in self.apps.iter().take(MAX_DISPLAYED_APPS) {
                let app_card = container(
                    column![
                        text(&app.name).size(18),
                        text(&app.desc).size(14),
                    ]
                    .spacing(5)
                )
                .padding(10)
                .width(Length::Fill);
                
                app_list = app_list.push(app_card);
            }
            
            app_list.into()
        };

        let scrollable_content = scrollable(content_area)
            .height(Length::Fill);

        let content = column![
            title_text,
            search_bar,
            categories,
            scrollable_content,
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
