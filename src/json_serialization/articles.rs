use std::vec::Vec;
use serde::Serialize;
use futures::future::{ready, Ready};
use actix_web::{Responder, Error, HttpResponse, HttpRequest};

use crate::json_serialization::article::Article;

/// This struct packages the raw struct fields to package items for JSON serialization.
///
/// # Parameters
/// * pending_items (Vec<Base>): vector containing the statuses and titles of pending items
/// * done_items (Vec<Base>):    vector containing the statuses and titles of the done items
/// * pending_item_count (i8):   the number of pending items
/// * done_item_count (i8):      the number of done items
#[derive(Serialize)]
pub struct Articles {
  pub articles: Vec<Article>,
}

impl Articles {
  /// This function constructs the Articles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(input_articles: Vec<Article>) -> Articles {
    let mut articles = Vec::new();

    for input_article in input_articles {
      articles.push(input_article);
    }

    return Articles {
      articles: articles
    }
  }
}

impl Responder for Articles {
  type Error  = Error;
  type Future = Ready<Result<HttpResponse, Error>>;
  /// This function gets fired when the struct is being returned in an actix view.
  ///
  /// # Arguments
  /// * _req (&HttpRequest): The request belonging to the view.
  ///
  /// # Returns
  /// * (Self::Future): An OK HTTP response with the serialized struct in the body.
  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}