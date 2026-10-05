use proc_macro::TokenStream;
use quote::quote;
use syn::{Token, parse::Parse, punctuated::Punctuated};

extern crate proc_macro;
/// 注册处理对应拓展名文件的属性宏，要求实现`FileHandler` trait,并提供new()构造参数，参考例子
/// ```Rust
/// #[regiser_handler(exts=["xml","xhtml"])]
/// pub(crate) struct XmlFileHandler;
/// impl FileHandler for XmlFileHandler {
///     fn extract_text(&self, file_path: &Path) -> Result<String> {
///         let file = std::fs::File::open(file_path)?;
///         let reader = EventReader::new(BufReader::new(file));
///         let mut buffer = String::new();
///         for event in reader.into_iter() {
///             let event = event.map_err(|err| {
///                 eprintln!("XML Read Event Error:{:?}", err);
///                 Error::new(ErrorKind::InvalidData, err)
///             })?;
///             if let XmlEvent::Characters(text) = event {
///                 buffer.push_str(&text);
///                 buffer.push(' ');
///             }
///         }
///         Ok(buffer)
///     }
/// }
/// impl XmlFileHandler {
///     pub(crate) fn new() -> Self {
///         Self
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn regiser_handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let structure = syn::parse_macro_input!(input as syn::ItemStruct);
    let register_args = syn::parse_macro_input!(args as RegisterArtgs);
    let name = &structure.ident;
    let exts = register_args
        .exts
        .iter()
        .map(|lit_str| lit_str.value())
        .collect::<Vec<_>>();
    eprintln!("DEBUG: processing the registered file hander:{name:>10}");
    quote! {
        #structure
        type HR=crate::file_handler::HandlerRegistration;
        inventory::submit!{
            HR{
                exts:&[#(#exts),*],
                create: || std::boxed::Box::new(#name::new())
            }
        }
    }
    .into()
}
struct RegisterArtgs {
    exts: Vec<syn::LitStr>,
}
impl Parse for RegisterArtgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        //读取exts
        let key: syn::Ident = input.parse()?;
        //检查参数名
        if key != "exts" {
            return Err(syn::Error::new(key.span(), "expected `exts`"));
        }
        //读取等号
        input.parse::<Token![=]>()?;
        //读取[]，填充内容到content
        let content;
        syn::bracketed!(content in input);
        let exts = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(&content)?;
        Ok(RegisterArtgs {
            exts: exts.into_iter().collect(),
        })
    }
}
