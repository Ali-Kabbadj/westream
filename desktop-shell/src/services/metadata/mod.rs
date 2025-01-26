use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub id: String,
    pub title: String,
    pub year: i32,
    pub poster: String,
}

#[allow(dead_code)]
pub struct MockMetadataService {
    mock_catalog: Vec<MediaItem>,
}

impl MockMetadataService {
    pub fn new() -> Self {
        // Initialize mock_catalog with test data

        Self {
            mock_catalog: vec![
                MediaItem {
                    id: "tt1375666".into(),
                    title: "Inception".into(),
                    year: 2010,
                    poster: "https://m.media-amazon.com/images/M/MV5BMjAxMzY3NjcxNF5BMl5BanBnXkFtZTcwNTI5OTM0Mw@@._V1_FMjpg_UX1000_.jpg".into(),
                },
                MediaItem {
                    id: "tt0816692".into(),
                    title: "Interstellar".into(),
                    year: 2014,
                    poster: "https://m.media-amazon.com/images/M/MV5BZjdkOTU3MDktN2IxOS00OGEyLWFmMjktY2FiMmZkNWIyODZiXkEyXkFqcGdeQXVyMTMxODk2OTU@._V1_FMjpg_UX1000_.jpg".into(),
                },
                MediaItem {
                    id: "tt0137523".into(),
                    title: "Fight Club".into(),
                    year: 1999,
                    poster: "https://m.media-amazon.com/images/M/MV5BNDIzNDU0YzEtYzE5Ni00ZjlkLTk5ZjgtNjM3NWE4YzA3Nzk3XkEyXkFqcGdeQXVyMjUzOTY1NTc@._V1_FMjpg_UX1000_.jpg".into(),
                }
            ],
        }
    }

    pub fn get_catalog(&self) -> &Vec<MediaItem> {
        &self.mock_catalog
    }
}
