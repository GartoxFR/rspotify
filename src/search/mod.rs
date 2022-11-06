use std::collections::BTreeMap;

use rspotify_model::SearchFilter;

/// Builder used to create search query.
///
/// Note that when calling the same function multiple time, the filter will be the text from the
/// last call
///
/// This is converted to the query string using into()
///
/// Exemple
/// ```rust
/// SearchQuery::default()
///     .any("foo")
///     .album("bar")
/// // Filter on album containing "bar" and anything containing "foo"
/// ```
///
/// For more informations on the different filters, look at the [soptify
/// documentation](https://developer.spotify.com/documentation/web-api/reference/#/operations/search)
#[derive(Debug, Default)]
pub struct SearchQuery<'a> {
    no_filter_query: &'a str,
    query_map: BTreeMap<SearchFilter, &'a str>,
}

impl<'a> SearchQuery<'a> {
    /// Basic filter where the given string can be anything
    pub fn any(&mut self, str: &'a str) -> &mut Self {
        self.no_filter_query = str;
        self
    }

    pub fn album(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Album, str);
        self
    }

    pub fn artist(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Artist, str);
        self
    }

    pub fn track(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Track, str);
        self
    }

    pub fn year(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Year, str);
        self
    }

    pub fn upc(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Upc, str);
        self
    }

    pub fn tag_new(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagNew, "");
        self
    }

    pub fn tag_hipster(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagHipster, "");
        self
    }

    pub fn isrc(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Isrc, str);
        self
    }

    pub fn genre(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Genre, str);
        self
    }
}

impl From<&SearchQuery<'_>> for String {
    fn from(val: &SearchQuery) -> Self {
        let mut rep = val.no_filter_query.to_owned();

        if val.query_map.is_empty() {
            return rep;
        }

        rep.push(' ');
        rep.push_str(
            val.query_map
                .iter()
                .map(|entry| match entry.0 {
                    SearchFilter::TagNew | SearchFilter::TagHipster => format!("{} ", entry.0),
                    _ => format!("{}:{} ", entry.0, entry.1),
                })
                .collect::<String>()
                .trim(),
        );

        rep
    }
}

impl From<&mut SearchQuery<'_>> for String {
    fn from(val: &mut SearchQuery) -> Self {
        String::from(&(*val))
    }
}

impl From<SearchQuery<'_>> for String {
    fn from(val: SearchQuery) -> Self {
        String::from(&val)
    }
}

#[cfg(test)]
mod test {
    use super::SearchQuery;

    #[test]
    fn test_search_query() {
        let query: String = SearchQuery::default()
            .any("foo bar")
            .album("wrong album")
            .album("arrival")
            .artist("abba")
            .tag_new()
            .tag_hipster()
            .track("foo")
            .year("2020")
            .upc("bar")
            .isrc("foo")
            .genre("metal")
            .into();

        let expected = "foo bar album:arrival artist:abba track:foo year:2020 upc:bar \
                        tag:hipster tag:new isrc:foo genre:metal";

        assert_eq!(expected, query);
    }

    #[test]
    fn test_empty_query() {
        let query: String = SearchQuery::default().into();

        assert_eq!(query, "");
    }
}