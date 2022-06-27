use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Salah" => "Err",
        "Ya" => "Ok",
        "Naskah" => "String",
        "Kamus" => "HashMap",
        "Bawaan" => "Default",
        "Kesalahan" => "Error",
        "Pilihan" => "Option",
        "Beberapa" => "Some",
        "Kosong" => "None",
        "Hasil" => "Result",
        "Sendiri" => "Self",
        "cetak" => "println",
        "hancur" => "break",
        "asinkron" => "async",
        "tunggu" => "await",
        "putaran" => "loop",
        "bergerak" => "move",
        "peti" => "crate",
        "kode_tidak_dapat_diakses" => "unreachable_code",
        "sebagai" => "as",
        "konstan" => "const",
        "sifat" => "trait",
        "bahaya" => "unsafe",
        "di" => "in",
        "dari" => "from",
        "dinamis" => "dyn",
        "membuka" => "unwrap",
        "bawaan" => "default",
        "sebagai_referensi" => "as_ref",
        "wk" => "io",
        "luar" => "extern",
        "salah" => "false",
        "fungsi" => "fn",
        "ganas" => "super",
        "masukan" => "insert",
        "baca" => "get",
        "izinkan" => "allow",
        "panik" | "ketar_ketir" | "jancuk"| "panteq"| "bangsat" => "panic",
        "modul" => "mod",
        "plin_plan" => "mut",
        "baru" => "new",
        "dimana" => "where",
        "untuk" => "for",
        "ambil_atau_masuk_dengan" => "get_or_insert_with",
        "utama" => "main",
        "umum" => "pub",
        "gak_punya" => None?,
        "kembalikan" => "return",
        "penerapan" => "impl",
        "referensi" => "ref",
        "cocok" => "match",
        "jika" => "if",
        "lain" => "else",
        "tubuh" => "self",
        "misal" => "let",
        "statis" => "static",
        "struktur" => "struct",
        "mengharap" => "expect",
        "selagi" => "while",
        "memakai" => "use",
        "ke_dalam" => "into",
        "benar" => "true",
        "cacah" => "enum",

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
pub fn karat(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
