#[recursion_limit="128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use syn::Ident;
use syn::DeriveInput;

use proc_macro::TokenStream;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use darling::FromDeriveInput;
use darling::FromVariant;


#[proc_macro_derive(ServiceError,attributes(error))]
pub fn detail_error_fn(input: TokenStream)->TokenStream{
    let proc_macro2_token = proc_macro2::TokenStream::from(input);
    let derive_input=syn::parse2::<DeriveInput>(proc_macro2_token).unwrap();
    let details_error: DetailErrorEnum=DetailErrorEnum::from_derive_input(&derive_input).unwrap();
    println!("{:#?}",details_error);
    let ident = &details_error.ident;
    let variants = details_error.data.take_enum().unwrap();
    let code_fn_codegen : Vec<proc_macro2::TokenStream> = variants.iter().map(|variant|{
        let variant_ident = &variant.ident;
        let code = variant.code.unwrap_or(0);
        quote!{
           #ident::#variant_ident => #code
        }
    }).collect();
    let msg_fn_codegen : Vec<proc_macro2::TokenStream> = variants.iter().map(|variant|{
        let variant_ident = &variant.ident;
        let msg= variant.message.clone().unwrap_or("undefined message".to_string());
        quote!{
           #ident::#variant_ident => format!("{}",#msg)
        }
    }).collect();

    let output  = quote! {
       impl #ident{
         pub fn get_code(&self)-> u16{
           match self{
            #(#code_fn_codegen,)*
           }
         }

         pub fn get_msg(&self)->String{
           match self{
             #(#msg_fn_codegen,)*
           }
         }

       }
    };
    println!("{:#?}",output);
    TokenStream::from(output)
}

#[derive(Debug,FromDeriveInput)]
#[darling(attributes(detail),supports(enum_any))]
struct DetailErrorEnum{
    ident: syn::Ident,
    data: darling::ast::Data<DetailErrorVariant,darling::util::Ignored>
}

#[derive(Debug,FromVariant)]
#[darling(attributes(detail))]
struct DetailErrorVariant{
    ident: syn::Ident,
    // fields 的数据， 指的是 `InvalidEmail(String)` 里面的 `String`
    fields: darling::ast::Fields<syn::Field>,
    // 这里表示从 `FromMeta` 中取数据，这里特指 `#[detail(code=400)]`
    #[darling(default)]
    code: Option<u16>,
    // 这里表示从 `FromMeta` 中取数据，这里特指 `#[detail(message="detail message")]`
    #[darling(default)]
    message: Option<String>,
}