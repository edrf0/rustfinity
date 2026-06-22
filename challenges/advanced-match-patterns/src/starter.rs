#[derive(Debug, Clone)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    // TODO: Finish the implementation
    pub fn check_validity(&self) -> bool {
        match self {
            BookItem::Book { pages, discount } => {
                if pages > 0 && (discount >= 0 && discount <= 50) {
                    true
                } else { false }
            },
            BookItem::EBook(title, (_, b)) => {
                if !title.is_empty() && b > 0 {
                    true
                } else { false }
            },
            BookItem::Collection(items) => {
                if items.is_empty() {
                    return false;
                }
                for item in items {
                    if !self.check_validity(item) {
                        return false;
                    }
                }
                true
            },
            BookItem::OutOfPrint => false,
        }
    }
}

// Example usage
pub fn main() {
    let book_a = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
    let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

    assert!(
        !book_a.check_validity(),
        "Book with discount > 50 should be invalid"
    );
    assert!(
        ebook_b.check_validity(),
        "EBook with valid title and tuple should be valid"
    );
    assert!(
        !collection_c.check_validity(),
        "Collection containing invalid items should be invalid"
    );
    assert!(
        !BookItem::OutOfPrint.check_validity(),
        "OutOfPrint should always be invalid"
    );
}
