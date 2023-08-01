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


/// Main function, exposes this library to JavaScript.
///
/// Example:
/// Run `node` in the `allbooks` directory after building with `npm install` and `npm run build --release`, then:
///
/// ```JavaScript
///     const allbooks = require('.')
///     allbooks.chadwick
/// ```
/// Output:
/// ```JavaScript
///    {
///    title: 'Chadwick the Crab',
///    author: 'Priscilla Cummings',
///    year: 2009
///   }
/// ````

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let book = Book { title: "Chadwick the Crab".to_string(), author: "Priscilla Cummings".to_string(), year: 2009 };
    let obj = book.to_object(&mut cx)?;
    cx.export_value("chadwick", obj)?;
    Ok(())
}
