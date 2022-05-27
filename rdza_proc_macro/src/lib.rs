use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Źle" => "Err",
        "Dobrze" => "Ok",
        "Ciąg" => "String",
        "MapaHaszy" | "MapaSkrótów" => "HashMap",
        "Wektor" => "Vec",
        "Domyślny" | "Domyślna" | "Domyślne" => "Default",
        "Błąd" => "Error",
        "Opcja" => "Option",
        "Jakiś" | "Jakaś" | "Jakieś" => "Some",
        "Żaden" | "Żadna" | "Żadne" => "None",
        "Wynik" => "Result",
        "Się" => "Self",
        "standardowy" | "standardowa" | "standardowe" => "std",
        "kolekcje" => "collections",
        "drukujln" => "println",
        "przełam" => "break",
        "współbieżny" | "współbieżna" | "współbieżne" => "async",
        "zaczekaj" => "await",
        "zapętl" => "loop",
        "przenieś" => "move",
        "skrzynka" | "skrzynki" | "skrzynkę" | "skrzynia" | "skrzyni" | "skrzynię" => "crate",
        "nieosiągalny_kod" => "unreachable_code",
        "jako" => "as",
        "stały" | "stała" | "stałe" => "const",
        "cecha" => "trait",
        "niebezpieczny" | "niebezpieczna" | "niebezpieczne" => "unsafe",
        "w" => "in",
        "z" => "from",
        "dynamiczny" | "dynamiczna" | "dynamiczne" => "dyn",
        "odpakuj" => "unwrap",
        "domyślny" | "domyślna" | "domyślne" => "default",
        "jako_referencja" => "as_ref",
        "zewnętrzny" | "zewnętrzna" | "zewnętrzne" => "extern",
        "fałsz" => "false",
        "funkcja" => "fn",
        "nadrzędny" | "nadrzędna" | "nadrzędne" => "super",
        "wstaw" => "insert",
        "wyjmij" => "get",
        "pozwól" => "allow",
        "kurwa" | "jerōnie" | "panikuj" => "panic",
        "moduł" => "mod",
        "zmienny" | "zmienna" | "zmienne" => "mut",
        "nowy" | "nowa" | "nowe" => "new",
        "gdzie" => "where",
        "dla" => "for",
        "wyjmij_lub_wstaw_z" => "get_or_insert_with",
        "główny" | "główna" | "główne" => "main",
        "publiczny" | "publiczna" | "publiczne" => "pub",
        "zwróć" => "return",
        "implementacja" => "impl",
        "referencja" => "ref",
        "dopasuj" => "match",
        "jeżeli" => "if",
        "inaczej" => "else",
        "się" => "self",
        "niech" => "let",
        "statyczny" | "statyczna" | "statyczne" => "static",
        "struktura" => "struct",
        "spodziewaj_się" => "expect",
        "dopóki" => "while",
        "użyj" => "use",
        "do" => "into",
        "rób" => "do",
        "prawda" => "true",
        "enumeracja" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rdza(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
