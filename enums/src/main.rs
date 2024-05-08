fn main() {
    //Many languages let you create enumeration types. Typically, these are shorthand for constants. For example in TypeScript:
    // enum WeekDay {
    //     Monday = 0,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday
    // }

    // Rust enums are far more powerful and are a key tool in structuring data and programs well. You use them almost everywhere
    // (indirectly, through Option and Result) for error handling and type safety.

    // What makes Rust enums more powerful is that they capture more than just a constant. Each variant of the enum can also have data.

    // An enum has a name and it has variants. Each variant can either be a "unit" enum or it can have data associated with it. Additionally,
    // there are two ways you can add data for a variant: positionally, or with named fields.

    // Let's define an enum with one of each of these types of variants to see them in action:
    enum LanguageResource {
        // This variant has a field by position; in this case, it's probably the URL
        Website(String),

        // This variant has fields by name
        Book { title: String, pages: u64 },

        // This is a unit variant, with no data contained
        SelfTeaching,
    }

    // To create an instance of an enum, you instantiate one of the variants. You refer to it using the enum name and the variant name,
    // separated with ::

    // Let's see examples of creating each of the previous variants, and then accessing their fields:
    let site = LanguageResource::Website("https://yet-another-rust-resource.pages.dev/".to_owned());
    let book = LanguageResource::Book {
        title: "The Rust Programming Language".to_owned(),
        pages: 300,
    };
    let independent = LanguageResource::SelfTeaching;

    println!("the site is at {}", site.0);
    println!("the book {} is {} pages long", book.title, book.pages);

    // What's the reason we'd like to have an enum with data, instead of just a struct? What's special about doing Result as an enum?

    // There are a few main reasons you want to do it as an enum, and why you cannot always use a struct:

    // 1. It allows a more compact memory representation. A struct requires the memory of all its fields, and if you have fields that are present
    // in some arms of the enum but not others, you'd pay for those always. In contrast, an enum only requires as much memory as its largest
    // variant, plus a byte (or word) for its tag. So this can save a serious amount of memory! And for things like Option of a reference, it
    // can even be free, because the compiler can do tricks to reuse memory in the pointer itself.

    // 2. It allows you to do pattern matching, which you cannot do on a struct with private members.

    // 3. You can enforce things at compile time, like which fields are or are not set, rather than enforcing populating fields through logic at
    // run time.

    // The memory layout of these is pretty straightforward. Each enum has a "tag", which represents which variant it is holding, and then
    // it has enough bytes to hold the largest variant.

    // The upshot of this is that no matter what variant your instance is, it will be as large as the largest variant, plus a bit extra for
    // the tag.

    // How large is the tag? It can be as small as one byte, if you have fewer than 255 variants (if you have more, you need a larger tag)
    // and if the alignment of the type is 1
}

//https://yet-another-rust-resource.pages.dev/enums
