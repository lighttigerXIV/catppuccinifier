use gtk4::CssProvider;

pub fn app_css() -> CssProvider{

    let provider = CssProvider::new();


    provider.load_from_data(
"

        .title{
            font-size: 1.3rem;
        }

        .version{
            background: @insensitive_base_color;
            padding: 4px 16px 4px 16px;
            border-radius: 8px;
            font-size: 1.1rem;
        }

        .small-padding{
            padding: 8px;
        }

        .medium-padding{
            padding: 16px;
        }

        .round-button{
            border-radius: 1rem;
            background: @theme_base_color;
            color: @theme_fg_color;
        }

        .round-button:hover{
            filter: brightness(120%);
            border-radius: 8px;
        }



        .foreground{
            background: @insensitive_base_color;
        }

        .generated-column{
            border-radius: 16px;
            padding: 8px;
        }

        .about-item{
            background: @insensitive_base_color;
            border-radius: 8px;
            padding: 8px;
        }

        .about-item:hover{
            filter: brightness(120%);
        }
    "
    );

    provider
}