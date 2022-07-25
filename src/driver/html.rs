use std::fs;

pub struct HtmlTemplate {
    header_labels: Vec<String>,
}

impl HtmlTemplate {
    pub fn from(label: String) -> HtmlTemplate {
        let header_labels: Vec<String> = vec![label];

        HtmlTemplate { header_labels }
    }

    pub fn append(&mut self, label: String) -> &HtmlTemplate {
        self.header_labels.push(label);

        self
    }

    pub fn write(&self) {
        rewrite_html_template(&self.header_labels.join("\n"))
    }
}

pub fn rewrite_html_template(header_label_template: &String) {
    let html_template = format!("
    <!doctype html>
    <html lang=\"en\">
    
    <head>
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <meta name=\"apple-mobile-web-app-status-bar-style\" content=\"black_translucent\">
        <meta name=\"theme-color\" content=\"#7ECCB4\">
        <meta name=\"apple-mobile-web-app-capable\" content=\"yes\" />
        <meta charset=\"utf-8\">
        <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">
        <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>
        <link href=\"https://fonts.googleapis.com/css2?family=Mulish:wght@200&display=swap\" rel=\"stylesheet\">
        <!-- <link rel=\"apple-touch-icon\" href=\"https://img-blog.csdnimg.cn/6024b595448e46de9902d5c23bef5be2.png\" /> -->
        <link data-trunk href=\"assets/styles/base.scss\" rel=\"scss\">
        <link data-trunk href=\"assets/styles/markdown.scss\" rel=\"scss\">
        <link data-trunk rel=\"copy-dir\" href=\"assets/images\">
        {}
        <style>
            html,
            body {{
                margin: 0;
                scroll-behavior: smooth;
                background: var(--underlay-color);
            }}
    
            * {{
                font-family: 'Mulish', sans-serif;
                line-height: 1.8;
                font-size: 16px;
                color: var(--text-color);
                transition: background-color 0.1s;
            }}
    
            .loading-wrapper {{
                height: 100%;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            }}
    
            .loading-icon {{
                width: 130px;
                animation: 1s ease-in-out infinite alternate loading;
            }}
    
            @keyframes loading {{
                from {{
                    opacity: 1;
                }}
    
                to {{
                    opacity: 0.3;
                }}
            }}
    
            @media (max-width: 600px) {{
                .loading-icon {{
                    width: 90px;
                }}
    
            }}
        </style>
    </head>
    
    <body>
    </body>
    
    </html>
    
    ", header_label_template);

    fs::write(".zzhack/app/index.html", html_template).unwrap();
}
