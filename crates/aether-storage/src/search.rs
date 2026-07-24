use aether_core::{AetherError, Result, Track, TrackId};
use std::sync::{Arc, RwLock};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

#[derive(Clone)]
pub struct InstantSearchEngine {
    index: Index,
    reader: IndexReader,
    schema: Schema,
    field_id: Field,
    field_title: Field,
    field_artist: Field,
    field_album: Field,
    field_genre: Field,
    writer: Arc<RwLock<IndexWriter>>,
}

impl InstantSearchEngine {
    pub fn new_in_memory() -> Result<Self> {
        let mut schema_builder = Schema::builder();
        let field_id = schema_builder.add_text_field("id", STRING | STORED);
        let field_title = schema_builder.add_text_field("title", TEXT | STORED);
        let field_artist = schema_builder.add_text_field("artist", TEXT | STORED);
        let field_album = schema_builder.add_text_field("album", TEXT | STORED);
        let field_genre = schema_builder.add_text_field("genre", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());

        let writer = index
            .writer(50_000_000) // 50MB RAM buffer
            .map_err(|e| AetherError::Storage(format!("Tantivy writer error: {}", e)))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .for_index()
            .map_err(|e| AetherError::Storage(format!("Tantivy reader error: {}", e)))?;

        Ok(Self {
            index,
            reader,
            schema,
            field_id,
            field_title,
            field_artist,
            field_album,
            field_genre,
            writer: Arc::new(RwLock::new(writer)),
        })
    }

    pub fn index_track(&self, track: &Track) -> Result<()> {
        let mut writer = self
            .writer
            .write()
            .map_err(|_| AetherError::Storage("Writer lock error".into()))?;

        let mut doc = TantivyDocument::default();
        doc.add_text(self.field_id, &track.id.0);
        doc.add_text(self.field_title, &track.title);
        doc.add_text(self.field_artist, &track.artist);
        doc.add_text(self.field_album, &track.album);
        if let Some(genre) = &track.genre {
            doc.add_text(self.field_genre, genre);
        }

        writer
            .add_document(doc)
            .map_err(|e| AetherError::Storage(format!("Add doc error: {}", e)))?;

        writer
            .commit()
            .map_err(|e| AetherError::Storage(format!("Commit doc error: {}", e)))?;

        Ok(())
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<TrackId>> {
        if query_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.field_title,
                self.field_artist,
                self.field_album,
                self.field_genre,
            ],
        );

        // Add wildcards to support prefix matching while typing
        let formatted_query = format!("{}*", query_str.trim());
        let query = query_parser
            .parse_query(&formatted_query)
            .or_else(|_| query_parser.parse_query(query_str.trim()))
            .map_err(|e| AetherError::Storage(format!("Parse query error: {}", e)))?;

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .map_err(|e| AetherError::Storage(format!("Search execution error: {}", e)))?;

        let mut track_ids = Vec::with_capacity(top_docs.len());
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| AetherError::Storage(format!("Doc retrieval error: {}", e)))?;

            if let Some(id_val) = retrieved_doc.get_first(self.field_id) {
                if let Some(id_str) = id_val.as_str() {
                    track_ids.push(TrackId(id_str.to_string()));
                }
            }
        }

        Ok(track_ids)
    }
}
