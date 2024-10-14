#![allow(non_snake_case)]

use assets::files::collapse_svg;
use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props, Clone, PartialEq)]
pub struct AppLayoutProps {
    title: String,
    fav_icon_src: String,
    collapse_svg_src: String,
    stylesheets: Vec<String>,
    section_class: String,
    js_href: String,
    header: Element,
    children: Element,
    sidebar: Element,
    sidebar_footer: Element,
    sidebar_header: Element,
}

pub fn BaseLayout(props: AppLayoutProps) -> Element {
    rsx!(
        head {
            title {
                "{props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            for href in &props.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            script {
                "type": "module",
                src: "{props.js_href}"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{props.fav_icon_src}"
            }
        }
        body {
            div {
                class: "flex h-screen",
                nav {
                    id: "sidebar",
                    class: "overflow-hidden bg-base-200 fixed md:relative h-full md:w-64 w-0 transition-all duration-300 flex flex-col justify-between",
                    div {
                        class: "flex items-center p-4",
                        {props.sidebar_header}
                    }
                    div {
                        class: "flex-1 overflow-y-auto",
                        {props.sidebar}
                    }
                    div {
                        class: "p-4",
                        {props.sidebar_footer}
                    }
                }
                turbo-frame {
                    id: "main-content",
                    class: "flex-1 flex flex-col",
                    header {
                        class: "flex items-center p-4",
                        button {
                            id: "toggleButton",
                            img {
                                height: "24",
                                width: "24",
                                class: "svg-icon mr-6",
                                src: collapse_svg.name
                            }
                        }
                        div {
                            class: "flex items-center w-full justify-between",
                            {props.header}
                        }
                    }
                    section {
                        class: "{props.section_class} flex-1 overflow-y-auto",
                        {props.children}
                    }
                }
            }
        }
    )
}