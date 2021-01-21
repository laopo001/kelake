use crate::react::HtmlVNode;
use crate::tag::TagTokens;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream, Result},
    Token,
};

pub struct HtmlElement {
    name: String,
    // props: ElementProps,
    children: HtmlElementChildren,
}

impl PeekValue<()> for HtmlElement {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlElementOpen::peek(cursor)
            .or_else(|| HtmlElementClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlElementClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlElementClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }
        dbg!(13);
        let open = input.parse::<HtmlElementOpen>()?;
        // Return early if it's a self-closing tag

        if open.is_self_closing() {
            return Ok(HtmlElement {
                name: open.name,
                // props: open.props,
                children: HtmlElementChildren::new(),
            });
        }

        // if let TagName::Lit(name) = &open.name {
        //     // Void elements should not have children.
        //     // See https://html.spec.whatwg.org/multipage/syntax.html#void-elements
        //     //
        //     // For dynamic tags this is done at runtime!
        //     match name.to_ascii_lowercase_string().as_str() {
        //         "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
        //         | "meta" | "param" | "source" | "track" | "wbr" => {
        //             return Err(syn::Error::new_spanned(open.to_spanned(), format!("the tag `<{}>` is a void element and cannot have children (hint: rewrite this as `<{0}/>`)", name)));
        //         }
        //         _ => {}
        //     }
        // }

        // let open_key = open.name.get_key();
        let mut children = HtmlElementChildren::new();
        let mut count = 1;
        let mut a = 1;
        dbg!(input.to_string());
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening tag has no corresponding closing tag",
                ));
            }

            if HtmlElementClose::peek(input.cursor()).is_some() {
                dbg!("HtmlElementClose");
                count -= 1;
                if count == 0 {
                    break;
                }
            } else {
                
            }
            a += 1;
            if (a == 10000) {
                return Err(syn::Error::new_spanned(open.to_spanned(), "无限循环"));
            }
            children.parse_child(input)?;
        }

        input.parse::<HtmlElementClose>()?;

        Ok(Self {
            name: open.name.clone(),
            // props: open.props,
            children,
        })
    }
}


impl ToTokens for HtmlElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();   
        // let mut t = quote! {};
        // let children = self.children.to_tokens(&mut t);
        let children = "abc".to_string();
        // dbg!(&name,&children);
        tokens.extend(quote! { 
            VNode::new(#name, vec![VNodeChild::VText(#children)]) 
            // "234"
        });
    }
}

struct HtmlElementOpen {
    tag: TagTokens,
    name: String,
    // props: ElementProps,
}
impl HtmlElementOpen {
    fn is_self_closing(&self) -> bool {
        self.tag.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<()> for HtmlElementOpen {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        Some(())
    }
}

impl Parse for HtmlElementOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            dbg!(input.to_string());
            // let at = input.parse::<TokenTree>()?;
            let name = input.parse::<Ident>()?.to_string();
            // let mut props = input.parse::<ElementProps>()?;

            Ok(Self { tag, name })
        })
    }
}

struct HtmlElementClose {
    tag: TagTokens,
    _name: String,
}
impl HtmlElementClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<()> for HtmlElementClose {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        // let (tag_key, cursor) = TagName::peek(cursor)?;
        // if let TagKey::Lit(name) = &tag_key {
        //     non_capitalized_ascii(&name.to_string()).as_option()?;
        // }
        let (ident, cursor) = cursor.ident()?;
        // dbg!(ident.to_string());
        // dbg!(punct.as_char());
        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;
        // dbg!(punct.as_char());
        Some(())
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let name = input.parse::<Ident>()?.to_string();

            // if let TagName::Expr(name) = &name {
            //     if let Some(expr) = &name.expr {
            //         return Err(syn::Error::new_spanned(
            //         expr,
            //         "dynamic closing tags must not have a body (hint: replace it with just `</@>`)",
            //     ));
            //     }
            // }

            Ok(Self { tag, _name: name })
        })
    }
}

pub struct HtmlElementChildren(Vec<HtmlVNode>);

impl HtmlElementChildren {
    pub fn new() -> Self {
        HtmlElementChildren(vec![])
    }
    pub fn parse_child(&mut self, input: ParseStream) -> Result<()> {
        self.0.push(input.parse()?);
        Ok(())
    }
}


impl ToTokens for HtmlElementChildren {
    fn to_tokens(&self, tokens: &mut TokenStream) {
    
        let children = self.0.iter().map(|x|{
            let mut t = quote! {};
            x.to_tokens(&mut t);
            t
        });
        tokens.extend(quote! { 
            #(
                #children
            ),* 
        });
    }
}