use neon::prelude::*;

/// A Book
/// Taken from https://neon-bindings.com/docs/objects
struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    /// Map this to JavaScript object.
    ///
    /// [Context<'a>] is a trait, so we can use it with  various contexts like [FunctionContext<'a>] and [ModuleContext<'a>].
    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();
        let title = cx.string(&self.title);
        obj.set(cx, "title", title)?;
        let author = cx.string(&self.author);
        obj.set(cx, "author", author)?;
        let year = cx.number(self.year);
        obj.set(cx, "year", year)?;
        Ok(obj)
    }
}

fn get_books() -> Vec<Book> {
    vec![
        Book { title: "Chadwick the Crab".to_string(), author: "Priscilla Cummings".to_string(), year: 2009 },
        Book { title: "The Little Prince".to_string(), author: "Antoine de Saint-Exupéry".to_string(), year: 1943 },
        Book { title: "The Hobbit".to_string(), author: "J. R. R. Tolkien".to_string(), year: 1937 },
    ]
}

/// Convert a Rust vector to a JavaScript array.
///
/// See https://neon-bindings.com/docs/arrays
fn vec_to_array<'a, C: Context<'a>>(vec: &Vec<Book>, cx: &mut C) -> JsResult<'a, JsArray> {
    let arr = JsArray::new(cx, vec.len() as u32);
    for (i, b) in vec.iter().enumerate() {
        let js_book = b.to_object(cx)?;
        arr.set(cx, i as u32, js_book)?;
    }
    Ok(arr)
}

fn get_books_as_js(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_arr = vec_to_array(&get_books(), &mut cx)?;
    Ok(js_arr)
}



/// Main function, exposes this library to JavaScript.
///
/// # Examples
/// Run `node` in the `allbooks` directory after building with `npm install` and `npm run build --release`, then:
///
/// ```JavaScript
///     const allbooks = require('.')
///     allbooks.chadwick
/// ```
///
/// Output:
///
/// ```JavaScript
///    {
///    title: 'Chadwick the Crab',
///    author: 'Priscilla Cummings',
///    year: 2009
///   }
/// ```
/// ## Example Array
///
/// ```JavaScript
///     const allbooks = require('.')
///     allbooks.books
/// ```
///
/// Output:
///
/// ```JavaScript
///     [
///       {
///         title: 'Chadwick the Crab',
///         author: 'Priscilla Cummings',
///         year: 2009
///       },
///       {
///         title: 'The Little Prince',
///         author: 'Antoine de Saint-Exupéry',
///         year: 1943
///       },
///       { title: 'The Hobbit', author: 'J. R. R. Tolkien', year: 1937 }
///     ]
/// ```
///
/// ## Example Function
///
/// To call a Rust function exposed to Node, you can do this:
///
/// ```JavaScript
///     const allbooks = require('.')
///     allbooks.get_books()
/// ```
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {

    // Expose a Book struct
    let book = Book { title: "Chadwick the Crab".to_string(), author: "Priscilla Cummings".to_string(), year: 2009 };
    let obj = book.to_object(&mut cx)?;
    cx.export_value("chadwick", obj)?;

    // Expose the list of books as an Array
    let books = get_books();
    let obj = vec_to_array(&books, &mut cx)?;
    cx.export_value("books", obj)?;

    // Expose a function
    cx.export_function("get_books", get_books_as_js)?;

    Ok(())
}
