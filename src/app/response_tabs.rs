#[derive(PartialEq)]
pub enum ResponseTabType {
    Headers,
    Json
}

#[derive(PartialEq)]
pub struct ResponseTabInfo {
    pub tab_type: ResponseTabType,
    pub name: String
}

pub struct ResponseTabs {
    pub tabs: [ResponseTabInfo; 2],
    pub visible_tab: ResponseTabType
}

impl Default for ResponseTabs {
    fn default() -> Self {
        ResponseTabs {
            tabs: [
                ResponseTabInfo {
                    tab_type: ResponseTabType::Headers, name: "Headers".to_string()
                },
                ResponseTabInfo {
                    tab_type: ResponseTabType::Json, name: "JSON".to_string()
                }
            ],
            visible_tab: ResponseTabType::Headers
        }
    }
}