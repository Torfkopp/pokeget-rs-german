#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use std::io::Cursor;

use crate::pokemon::Region;
use bimap::BiHashMap;
use inflector::Inflector;
use rand::Rng;

/// A parsed representation of `names.csv`.
///
/// Used to derive filenames from Pokedex ID's, and to
/// format image filenames back into proper pokemon names.
pub struct List {
    /// The Pokedex IDs and their corresponding filenames.
    ids: BiHashMap<usize, String>,

    /// All the proper, formatted names in order of Pokedex ID.
    names: Vec<String>,
    german_names: Vec<String>,
}

impl List {
    /// Reads a new [`List`] from `data/names.csv`.
    pub fn read() -> Self {
        const FILE: &'static str = include_str!("../data/names.csv");

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(FILE));

        const CAPACITY: usize = 1200; // Over 1000 Pokemon already :O

        let mut ids = BiHashMap::with_capacity(CAPACITY);
        let mut names = Vec::with_capacity(CAPACITY);
        let mut german_names = Vec::with_capacity(CAPACITY);

        for (i, entry) in reader.deserialize().enumerate() {
            let record: (String, String, String) = entry.unwrap();

            ids.insert(i, record.2);
            names.push(record.0);
            german_names.push(record.1)
        }

        Self {
            ids,
            names,
            german_names,
        }
    }

    /// Takes a filename and looks up the proper display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use pokeget::list::List;
    /// let list = List::read();
    /// assert_eq!(list.format_name("mr-mime"), "Pantimos")
    /// ```
    pub fn format_name(&self, filename: &str) -> String {
        let raw_fmt = |x: &str| x.replace('-', " ").replace('\'', "").to_title_case();

        let Some(id) = self.ids.get_by_right(filename) else {
            return raw_fmt(filename);
        };
        let Some(name) = self.german_names.get(*id) else {
            return raw_fmt(filename);
        };

        name.clone()
    }

    /// Gets a pokemon filename by its name (English or German)
    pub fn get_by_name(&self, name: &str) -> Option<&String> {
        let mut id = self
            .german_names
            .iter()
            .position(|n| n.to_lowercase() == name.to_lowercase());
        if id == None {
            id = self
                .names
                .iter()
                .position(|n| n.to_lowercase() == name.to_lowercase());
        }

        return self.get_by_id(id?);
    }

    /// Gets a filename with fuzzy search
    pub fn get_by_name_fuzzy(&self, name: &str) -> Option<&String> {
        use rust_fuzzy_search::fuzzy_search_best_n;

        // Combine the vectors and convert to Vec<&str>
        let combined: Vec<&str> = self
            .german_names
            .iter()
            .chain(self.names.iter())
            .map(|s| s.as_str())
            .collect();

        // Perform fuzzy search
        let res: Vec<(&str, f32)> = fuzzy_search_best_n(name, &combined, 1);

        // Return the best match (res[0].0 is the matched string)
        let fuzzy_name = res.first().map(|(s, _)| s.to_string()).unwrap_or_default();

        return self.get_by_name(&fuzzy_name);
    }

    /// Gets a pokemon filename by a Dex ID.
    pub fn get_by_id(&self, id: usize) -> Option<&String> {
        self.ids.get_by_left(&id)
    }

    /// Gets a random pokemon & returns it's filename.
    pub fn random(&self) -> String {
        let mut rand = rand::thread_rng();

        let idx = rand.gen_range(0..self.ids.len());
        self.ids.get_by_left(&idx).unwrap().clone()
    }

    /// Gets a random pokemon by region
    pub fn get_by_region(&self, region: Region) -> String {
        let mut rand = rand::thread_rng();

        let region = match region {
            Region::Kanto => 0..=151,
            Region::Johto => 152..=251,
            Region::Hoenn => 252..=386,
            Region::Sinnoh => 387..=493,
            Region::Einall => 494..=649,
            Region::Kalos => 650..=721,
            Region::Alola => 722..=809,
            Region::Galar => 810..=905,
            Region::Paldea => 906..=1025,
        };

        let idx = rand.gen_range(region);
        self.ids.get_by_left(&idx).unwrap().clone()
    }
}
